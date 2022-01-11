//! A handy, albeit a bit janky proc macro for converting idiomatic Rust
//! functions to a function interoperable with `mlua`.
//!
//! # Example
//! ```
//! use mlua::prelude::*;
//! use mooncake::mooncake;
//! use std::collections::HashMap;
//!
//! #[mooncake]
//! fn register(map: HashMap<String, u8>, id: String) -> LuaResult<()> {
//!     // ...
//! # Ok(())
//! }
//! ```
//! The generated code looks like this:
//! ```
//! # use mlua::prelude::*;
//! # use mooncake::mooncake;
//! # use std::collections::HashMap;
//! fn register(_: &mlua::Lua, (tiles, id): (HashMap<String, u8>, String)) -> LuaResult<()> {
//!     // ...
//! # Ok(())
//! }
//! ```
//!
//! # Supported Parameters
//! If the parameter `lua` is provided, the normally inaccessible provided Lua
//! instance becomes accessible to the user under the parameter named `lua`.
//! ```
//! use mlua::prelude::*;
//! use mooncake::mooncake;
//!
//! #[mooncake]
//! fn without_lua(i: u8) -> LuaResult<()> { Ok(()) }
//!
//! #[mooncake(lua)]
//! fn with_lua(i: u8) -> LuaResult<()> { Ok(()) }
//! ```
//! The macro expands into something like this:
//! ```
//! # use mlua::prelude::*;
//! # use mooncake::mooncake;
//! fn without_lua(_: &mlua::Lua, (i): (u8)) -> LuaResult<()> { Ok(()) }
//!
//! fn with_lua(lua: &mlua::Lua, (i): (u8)) -> LuaResult<()> { Ok(()) }
//! ```
//!
//! # Mechanism
//! `mooncake` works by transforming a function's signature, to match the signature
//! specified for [`mlua::Lua::create_function`] and [`mlua::Lua::create_function_mut`].
//!
//! Firstly, a parameter of type `&mlua::Lua` is prepended, which may or may not be
//! accessible for the caller, [depending on the attribute provided](#supported-parameters).
//!
//! It also packs other parameters into a tuple:
//! ```
//! use mlua::prelude::*;
//! use mooncake::mooncake;
//!
//! #[mooncake]
//! fn no_args() -> LuaResult<()> { Ok(()) }
//! #[mooncake]
//! fn one_arg(a: u8) -> LuaResult<()> { Ok(()) }
//! #[mooncake]
//! fn two_args(a: u8, b: String) -> LuaResult<()> { Ok(()) }
//! #[mooncake]
//! fn three_args(a: u8, b: String, c: f32) -> LuaResult<()> { Ok(()) }
//! ```
//! Expanded:
//! ```
//! # use mlua::prelude::*;
//! # use mooncake::mooncake;
//! // a blank parameter is needed here
//! fn no_args(_: &mlua::Lua, _: ()) -> LuaResult<()> { Ok(()) }
//! fn one_arg(_: &mlua::Lua, (a): (u8)) -> LuaResult<()> { Ok(()) }
//! fn two_args(_: &mlua::Lua, (a, b): (u8, String)) -> LuaResult<()> { Ok(()) }
//! fn three_args(_: &mlua::Lua, (a, b, c): (u8, String, f32)) -> LuaResult<()> { Ok(()) }
//! ```
//!
//! # Errors and Limitations
//! This macro currently only works on free-standing and associated functions.
//! Methods are **not supported**.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, FnArg, ItemFn, Meta, NestedMeta, PatType,
};

/// The macro itself!
/// Check out the [module-level documentation](index.html) for details.
#[proc_macro_attribute]
pub fn mooncake(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attrs);
    let mut input = parse_macro_input!(item as ItemFn);

    let mut inputs = input.sig.inputs;

    if inputs.is_empty() {
        inputs.push(parse_quote!(_: ()));
    } else {
        let (pats, tys): (Vec<_>, Vec<_>) = inputs
            .iter_mut()
            .map(|i| match i {
                FnArg::Receiver(_) => panic!("methods are currently unsupported"),
                FnArg::Typed(PatType { pat, ty, .. }) => (pat, ty),
            })
            .unzip();
        inputs = parse_quote!((#(#pats),*): (#(#tys),*));
    }

    let lua_param = match attrs.lua {
        Lua::Ref => parse_quote!(lua: &mlua::Lua),
        Lua::Hidden => parse_quote!(_: &mlua::Lua),
    };
    inputs.insert(0, lua_param);
    input.sig.inputs = inputs;

    let items = quote! {
        #input
    };
    TokenStream::from(items)
}

/// Attributes of the `#[mooncake]` macro.
/// Check out the [module-level documentation](index.html) for details.
struct Attrs {
    lua: Lua,
}
impl Attrs {
    fn parse_lua(m: NestedMeta) -> Lua {
        if let NestedMeta::Meta(Meta::Path(p)) = m {
            if p.is_ident("lua") {
                return Lua::Ref;
            }
        }
        Lua::Hidden
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let m = input.parse();
        let lua = m.map(Self::parse_lua).unwrap_or(Lua::Hidden);
        Ok(Self { lua })
    }
}

/// Settings for the added Lua parameter.
enum Lua {
    /// If set to this setting, a *shared* reference to the [`mlua::Lua`] instance
    /// is supplied to the function.
    Ref,
    /// If set to this setting, the provided *shared* Lua instance is hidden from the callee.
    Hidden,
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
    t.compile_fail("tests/ui/fails/*.rs");
}
