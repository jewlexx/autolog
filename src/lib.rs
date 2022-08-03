use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn logging_gen(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let fn_decl = parse_macro_input!(input as ItemFn);

    let async_keyword = if fn_decl.sig.asyncness.is_some() {
        quote! { async }
    } else {
        quote! {}
    };

    let fn_vis = fn_decl.vis;

    quote! {
        #fn_vis #async_keyword fn {
        }
    }
    .into()
}
