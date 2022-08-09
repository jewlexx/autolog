use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn logging_gen(args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_decl = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(args as AttributeArgs);
    let fn_ident = &fn_decl.sig.ident;
    let async_key = &fn_decl.sig.asyncness;
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
                    return quote! {
                            compile_error!("expected string literal for logging message.")
                    }
                    .into();
                }
            },
            NestedMeta::Meta(x) => match x {
                Meta::Path(_) => todo!(),
                Meta::List(_) => todo!(),
                Meta::NameValue(val) => {
                    let val_ident = &val.path.segments.first().unwrap().ident;

                    match val_ident.to_string().as_str() {
                        "print_macro" => {
                            let lit: proc_macro2::TokenStream =
                                val.lit.to_token_stream().to_string().parse().unwrap();
                            return quote! {
                                compile_error!(#lit);
                            }
                            .into();
                        }
                        _ => {
                            let val_ident_str = val_ident.to_string();
                            return quote! {
                                    compile_error!(concat!("invalid argument \"", #val_ident_str, "\""));
                            }
                            .into();
                        }
                    }
                }
            },
        }
    }

    quote! {
        #async_key fn #fn_ident() {
            println!(#print_message, fn_name = stringify!(#fn_ident));
        }
    }
    .into()
}
