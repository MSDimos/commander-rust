use syn::{ Ident, token, bracketed, LitStr };
use syn::parse::{ Parse, ParseStream, Result };
use quote::ToTokens;
use quote::{ quote };
use proc_macro2::{ TokenStream as TokenStream2 };

#[derive(Debug)]
pub(crate) struct SubFnsList {
    pub(crate) inner: Vec<Ident>,
}

impl Parse for SubFnsList {
    fn parse(stream: ParseStream) -> Result<Self> {
        let mut list = SubFnsList {
            inner: vec![],
        };

        while stream.peek(Ident) {
            list.inner.push(stream.parse::<Ident>()?);

            if stream.peek(token::Comma) {
                stream.parse::<token::Comma>()?;
            }
        }

        Ok(list)
    }
}

#[derive(Debug)]
pub(crate) struct Register {
    pub(crate) cmd: Ident,
    pub(crate) sub_fns_list: SubFnsList,
}

impl Parse for Register {
    fn parse(stream: ParseStream) -> Result<Self> {
        let cmd = stream.parse::<Ident>()?;
        let content;

        stream.parse::<token::Comma>()?;
        bracketed!(content in stream);
        let sub_fns_list = content.parse::<SubFnsList>()?;

        Ok(Register {
            cmd,
            sub_fns_list,
        })
    }
}


#[derive(Debug)]
pub(crate) struct OptionVersion(Option<LitStr>);

impl Parse for OptionVersion {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.peek(LitStr) {
            Ok(OptionVersion(Some(stream.parse::<LitStr>()?)))
        } else {
            Ok(OptionVersion(None))
        }
    }
}

impl ToTokens for OptionVersion {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let tmp = if let Some(lit_str) = &self.0 {
            quote! { { #lit_str } }
        } else {
            quote! { { std::env!("CARGO_PKG_VERSION") } }
        };

        tmp.to_tokens(stream);
    }
}
