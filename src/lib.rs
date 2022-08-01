use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

#[proc_macro_attribute]
pub fn logging_gen(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    for arg in args {
        match arg {
            NestedMeta::Lit(x) => {}
            _ => (),
        }
    }

    let fn_decl = parse_macro_input!(input as ItemFn);

    quote! {}.into()
}
