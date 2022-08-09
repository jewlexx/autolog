use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, NestedMeta};

macro_rules! macro_error {
    ($msg:literal) => {
        quote::quote! {
            compile_error!($msg);
        }
        .into()
    };
}

#[proc_macro_attribute]
pub fn logging_gen(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let fn_decl = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(args as AttributeArgs);
    let fn_ident = &fn_decl.sig.ident;
    let async_key = &fn_decl.sig.asyncness;
    let fn_args = &fn_decl.sig.inputs;
    let mut print_message = {
        let message = r#""{fn_name}" was called"#;
        quote! { #message }
    };

    for arg in args {
        match arg {
            NestedMeta::Lit(x) => match x {
                Lit::Str(x) => {
                    print_message = x.to_token_stream();
                }
                _ => {
                    return macro_error!("expected string literal for logging message.");
                }
            },
            _ => {
                return macro_error!("did not expect arguments");
            }
        }
    }

    let print_args = if print_message.to_string().contains("{fn_name}") {
        quote! {, fn_name = stringify!(#fn_ident)}
    } else {
        TokenStream::new()
    };

    quote! {
        #async_key fn #fn_ident(#fn_args) {
            println!(#print_message #print_args);
        }
    }
    .into()
}
