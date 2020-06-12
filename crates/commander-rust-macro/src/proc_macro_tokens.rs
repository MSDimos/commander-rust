use syn::{ Ident, token, bracketed };
use syn::parse::{ Parse, ParseStream, Result };

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