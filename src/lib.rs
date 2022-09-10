#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, __private::ext::RepToTokensExt};
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, ItemFn, Lit, NestedMeta};

macro_rules! macro_error {
    ($msg:expr) => {
        quote::quote! {
            compile_error!($msg);
        }
        .into()
    };

    ($msg:tt, $span:expr) => {
        quote::quote_spanned! { $span =>
            compile_error!($msg);
        }
        .into()
    };
}

/// The main autolog macro
///
/// When this macro is applied to a function, it will log either the default message or the provided string literal
///
/// The default message prints the function name and the arguments passed to the function
///
/// In the custom message, the function name and arguments passed to the function are available as `fn_name`
///
/// # Examples
/// ```
/// # use autolog::autolog;
/// #[autolog]
/// fn default_message(name: &str) {
///    println!("Hello, {}!", name);
/// }
/// ```
#[proc_macro_attribute]
pub fn autolog(arg_tokens: TokenStream1, input: TokenStream1) -> TokenStream1 {
    let fn_decl = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(arg_tokens as AttributeArgs);
    let fn_ident = &fn_decl.sig.ident;
    let async_key = &fn_decl.sig.asyncness;
    let fn_args = &fn_decl.sig.inputs;

    let mut print_message = quote!("\"{fn_name}\" was called");

    for (i, arg) in args.iter().enumerate() {
        match arg {
            NestedMeta::Lit(x) => {
                if i == 0 {
                    match x {
                        Lit::Str(x) => {
                            print_message = x.to_token_stream();
                        }
                        _ => {
                            return macro_error!(
                                "expected string literal for logging message",
                                x.span()
                            );
                        }
                    }
                } else {
                    return macro_error!("logging message must be the first argument", arg.span());
                }
            }
            NestedMeta::Meta(x) => {
                if i == 0 {
                    return macro_error!("logging message must be a string literal", x.span());
                }

                let p = x.path().to_token_stream().to_string();

                return quote! {
                    compile_error!(#p);
                }
                .into();
            }
        }
    }

    let print_args = if print_message.to_string().contains("{fn_name}") {
        quote! {, fn_name = stringify!(#fn_ident)}
    } else {
        TokenStream::new()
    };

    let print_macro = if cfg!(feature = "tracing") {
        quote! { tracing::trace! }
    } else {
        quote! { println! }
    };

    quote! {
        #async_key fn #fn_ident(#fn_args) {
            #print_macro (#print_message #print_args);
        }
    }
    .into()
}
