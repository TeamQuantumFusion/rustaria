use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, FnArg, ItemFn, Meta, NestedMeta, PatType,
};

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

enum Lua {
    /// If set to this setting, a *shared* reference to the [`mlua::Lua`] instance
    /// is supplied to the function.
    Ref,
    /// If set to this setting, the provided *shared* Lua instance is hidden from the callee.
    Hidden,
}

/// A handy, albeit a bit janky macro for converting idiomatic Rust functions
/// to a function interoperable with `mlua`.
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
        Lua::Hidden => parse_quote!(_lua: &mlua::Lua),
    };
    inputs.insert(0, lua_param);
    input.sig.inputs = inputs;

    let items = quote! {
        #input
    };
    TokenStream::from(items)
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
}
