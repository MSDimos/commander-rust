use syn::punctuated::Punctuated;
use syn::{ FnArg, Ident };
use syn::token;
use quote::quote;
use proc_macro2::{ TokenStream as TokenStream2 };

/// Generate inputs of command processing function.
///
/// Why need it? Because the length of inputs is unknown, maybe 1 maybe 10.
/// But we need a common way to call it, so we need to generate inputs tokens needed.

#[doc(hidden)]
pub fn generate_call_fn(inputs: &Punctuated<FnArg, token::Comma>, call_fn_name: &Ident, fn_name: &Ident) -> TokenStream2 {
    let mut tokens: Vec<TokenStream2> = vec![];

    for (idx, arg) in inputs.iter().enumerate() {
        if let FnArg::Captured(cap) = arg {
            let ty = &cap.ty;

            if idx < inputs.len() - 1 {
                tokens.push((quote! {
                    {
                        <#ty>::from(raws[#idx].clone())
                    }
                }).into());
            } else {
                let ts = TokenStream2::from(quote! {
                    #ty
                });
                let mut ts_str = ts.to_string();

                ts_str.retain(|c| !char::is_whitespace(c));

                if ts_str != "Cli" {
                    tokens.push((quote! {
                        {
                            <#ty>::from(raws[#idx].clone())
                        }
                    }).into());
                } else {
                    tokens.push((quote! {
                        {
                            cli
                        }
                    }).into());
                }
            }
        }
    }

    (quote! {
        fn #call_fn_name(raws: &Vec<_commander_rust_Raw>, cli: _commander_rust_Cli) {
            #fn_name(#(#tokens,)*)
        }
    }).into()
}
