extern crate gl_generator;

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Generator, Profile, Registry};
use gl_generator::{Binding, Cmd, Enum};

fn main() {
    let path = Path::new("./src/gl.rs");
    if !path.exists() {
        let mut file = File::create(&path).unwrap();

        let mut bindings = Vec::new();
        Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
            .write_bindings(NiceGenerator, &mut bindings)
            .unwrap();

        file.write_all(bindings.as_ref()).unwrap();
    }
}

pub struct NiceGenerator;

impl Generator for NiceGenerator {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        // Header
        dest.write_all(
            br#"
#![allow(
    non_upper_case_globals, 
    non_snake_case, 
    non_camel_case_types,
    dead_code,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
)]
            "#
        )?;
        dest.write_all("use std::ffi::c_void;\n".as_ref())?;
        dest.write_all("use std::mem::transmute;\n".as_ref())?;

        // Types
        let mut write = Vec::new();
        gl_generator::generators::gen_types(registry.api, &mut write)?;
        dest.write_all(
            String::from_utf8(write)
                .unwrap()
                .replace("super::__gl_imports::raw::", "std::os::raw::")
                .as_ref(),
        )?;

        // Constants
        for en in &registry.enums {
            write_const(dest, en);
        }

        // Functions
        for cmd in &registry.cmds {
            write_fn(dest, cmd);
        }

        // Storage
        for cmd in &registry.cmds {
            write_fn_storage(dest, cmd);
        }

        // Loading
        dest.write_all(
            br#"

static ERR: extern "system" fn() = gl_not_loaded;
extern "system" fn gl_not_loaded() {
    panic!("GL Function not loaded")
}
"#
        )?;

        dest.write_all(
    br#"
#[inline(never)]
unsafe fn load(
    func: &mut dyn FnMut(&'static str) -> *const c_void,
    symbol: &'static str,
    fallbacks: &[&'static str]
) -> *const c_void {
    let mut ptr = func(symbol);
    if ptr.is_null() {
        for &sym in fallbacks {
            ptr = func(sym);
            if !ptr.is_null() { break; }
        }
    }
    if ptr.is_null() {
        ptr = transmute(ERR as *const std::ffi::c_void);
    }
    ptr
}
"#
        )?;

        dest.write_all(
            "pub unsafe fn load_gl<F: FnMut(&'static str) -> *const c_void>(func: &mut F) {"
                .as_ref(),
        )?;
        for x in &registry.cmds {
            let indent = &x.proto.ident;
            let fallbacks = match registry.aliases.get(indent) {
                Some(v) => {
                    let names = v
                        .iter()
                        .map(|name| format!("\"{name}\""))
                        .collect::<Vec<_>>();
                    format!("&[{}]", names.join(", "))
                }
                None => "&[]".to_string(),
            };
            dest.write_all(
                format!("_{indent} = transmute(load(func, \"{indent}\", {fallbacks}));\n").as_ref(),
            )?;
        }
        dest.write_all(b"}")?;

        Ok(())
    }
}

pub fn write_fn_storage<W: io::Write>(w: &mut W, cmd: &Cmd) {
    // pub static mut _VertexP4ui: extern "system" fn(type_: i32, value: u32) = unsafe { std::mem::transmute(ERR) };
    let name = &cmd.proto.ident;
    let fn_str = param_ret_str(&cmd.params, &cmd.proto, false);
    let string = format!(
        "static mut _{name}: extern \"system\" fn{fn_str} = unsafe {{ transmute(ERR) }};\n"
    );
    w.write_all(string.as_ref()).unwrap();
}

pub fn write_fn<W: io::Write>(w: &mut W, cmd: &Cmd) {
    // pub unsafe fn VertexP4ui(type_: i32, value: u32) { _VertexP4ui(type_, value) }
    let name = &cmd.proto.ident;
    let fn_str = param_ret_str(&cmd.params, &cmd.proto, true);
    let call: Vec<String> = cmd.params.iter().map(|param| param.ident.clone()).collect();
    let call = call.join(", ");
    let string = format!("pub unsafe fn {name}{fn_str} {{ _{name}({call}) }}\n");
    w.write_all(string.as_ref()).unwrap();
}

pub fn write_const<W: io::Write>(w: &mut W, en: &Enum) {
    let string = format!(
        "pub const {name}: {ty} = {value};\n",
        name = en.ident,
        ty = ty_str(&en.ty),
        value = en.value
    );
    w.write_all(string.as_ref()).unwrap();
}

fn ty_str(str: &str) -> String {
    str.replace("types::", "")
        .replace("__gl_imports::raw::", "")
}

fn param_ret_str(params: &[Binding], proto: &Binding, names: bool) -> String {
    let params = params_str(params, names);
    if proto.ty != "()" {
        format!("({params}) -> {}", ty_str(&proto.ty))
    } else {
        format!("({params})")
    }
}

fn params_str(params: &[Binding], names: bool) -> String {
    let params: Vec<String> = params
        .iter()
        .map(|bind| {
            if names {
                format!("{}: {}", bind.ident, ty_str(&bind.ty))
            } else {
                ty_str(&bind.ty)
            }
        })
        .collect();
    params.join(", ")
}
