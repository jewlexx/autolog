use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn logging_gen(args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_decl = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(args as AttributeArgs);
    let fn_ident = &fn_decl.sig.ident;
    let mut print_message = {
        let message = format!("\"{}\" was called", fn_ident);
        quote! { #message }
    };

    for arg in args {
        match arg {
            NestedMeta::Lit(x) => match x {
                Lit::Str(x) => {
                    print_message = x.to_token_stream();
                }
                Lit::ByteStr(_) => todo!(),
                Lit::Byte(_) => todo!(),
                Lit::Char(_) => todo!(),
                Lit::Int(_) => todo!(),
                Lit::Float(_) => todo!(),
                Lit::Bool(_) => todo!(),
                Lit::Verbatim(_) => todo!(),
            },
            _ => (),
        }
    }

    quote! {
        fn #fn_ident() {
            println!(#print_message);
        }
    }
    .into()
}
