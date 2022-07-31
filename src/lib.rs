use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn logging_gen(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let fn_decl = parse_macro_input!(input as ItemFn);

    quote! {}.into()
}
