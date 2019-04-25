use std::collections::HashSet;

use syn::{ Ident, LitStr, token, Token, bracketed };
use syn::parse::{ Parse, ParseStream, Result };
use quote::{ ToTokens, quote };
use proc_macro2::{ TokenStream as TokenStream2 };

use crate::errors::{error, ARG_DUPLICATE_DEFINITION, ORDER_ERROR, error_nt};

#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[doc(hidden)]
pub enum ArgumentType {
    RequiredSingle,
    OptionalSingle,
    RequiredMultiple,
    OptionalMultiple,
}

#[derive(Debug)]
#[doc(hidden)]
pub struct Argument {
    pub name: Ident,
    pub ty: ArgumentType,
}

#[derive(Debug)]
#[doc(hidden)]
pub struct CommandToken {
    pub name: Ident,
    pub args: Vec<Argument>,
    pub desc: Option<LitStr>,
}

#[derive(Debug)]
#[doc(hidden)]
pub struct OptionsToken {
    pub short: Ident,
    pub long: Ident,
    pub arg: Option<Argument>,
    pub desc: Option<LitStr>,
}

impl ToTokens for ArgumentType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            &ArgumentType::RequiredSingle => {
                (quote! {
                    _commander_rust_ArgumentType::RequiredSingle
                }).to_tokens(tokens);
            },
            &ArgumentType::RequiredMultiple => {
                (quote! {
                    _commander_rust_ArgumentType::RequiredMultiple
                }).to_tokens(tokens);
            },
            &ArgumentType::OptionalSingle => {
                (quote! {
                    _commander_rust_ArgumentType::OptionalSingle
                }).to_tokens(tokens);
            },
            &ArgumentType::OptionalMultiple => {
                (quote! {
                    _commander_rust_ArgumentType::OptionalMultiple
                }).to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Argument {
            name,
            ty,
        } = self;
        let name = format!("{}", name);
        let expand = quote! {
            _commander_rust_Argument {
                name: String::from(#name),
                ty: #ty,
            }
        };

        expand.to_tokens(tokens);
    }
}


impl CommandToken {
    pub fn check(&self) {
        let mut set = HashSet::new();
        let mut flag = true;

        for (idx, arg) in self.args.iter().enumerate() {
            let name = format!("{}", arg.name);

            // check duplicate
            if set.contains(&name) {
                error(ARG_DUPLICATE_DEFINITION, &name);
            } else {
                set.insert(name);
            }
            // check order
            if arg.ty == ArgumentType::RequiredSingle || arg.ty == ArgumentType::RequiredMultiple {
                if !flag {
                    error_nt(ORDER_ERROR);
                }
            } else {
                flag = false;
            }

            // check last argument
            if idx != self.args.len() - 1 {
                if arg.ty == ArgumentType::RequiredMultiple || arg.ty == ArgumentType::OptionalMultiple {
                    error_nt(ORDER_ERROR);
                }
            }
        }

    }
}

// command(rmdir <dir> [otherDirs...], "yes")
impl Parse for CommandToken {
    fn parse(tokens: ParseStream) -> Result<Self> {
        let name = tokens.parse()?;
        let mut args = vec![];
        let mut desc = None;
        let cmd_token: CommandToken;

        while !tokens.is_empty() {
            let lookhead = tokens.lookahead1();

            if lookhead.peek(token::Lt) {
                // skip <
                tokens.parse::<token::Lt>()?;
                let name = tokens.parse()?;
                let ty = if tokens.peek(token::Dot3) {
                    tokens.parse::<token::Dot3>()?;
                    ArgumentType::RequiredMultiple
                } else {
                    ArgumentType::RequiredSingle
                };

                args.push(Argument {
                    name,
                    ty,
                });
                // skip >
                tokens.parse::<token::Gt>()?;
            } else if lookhead.peek(token::Bracket) {
                let content;
                bracketed!(content in tokens);
                let name = content.parse()?;
                let ty = if content.peek(token::Dot3) {
                    content.parse::<token::Dot3>()?;
                    ArgumentType::OptionalMultiple
                } else {
                    ArgumentType::OptionalSingle
                };

                args.push(Argument {
                    name,
                    ty,
                });
            } else {
                break;
            }
        }

        if tokens.peek(token::Comma) && tokens.peek2(LitStr) {
            tokens.parse::<token::Comma>()?;
            desc = tokens.parse()?;
        }

        cmd_token = CommandToken {
            name,
            args,
            desc,
        };
        cmd_token.check();
        Ok(cmd_token)
    }
}

impl ToTokens for CommandToken {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let CommandToken {
            name,
            args,
            desc,
        } = self;
        let name = format!("{}",name);
        let desc = if let Some(litstr) = desc {
            quote! {
                Some(String::from(#litstr))
            }
        } else {
            quote!(None)
        };
        let expand = quote! {
            _commander_rust_Command {
                name: String::from(#name),
                args: vec![#( #args ),*],
                desc: #desc,
                // 这里保留，因为还要插入options
                opts: vec![],
            }
        };

        expand.to_tokens(tokens);
    }
}

// option(-s, --simple <dir>, "Hello world!")
impl Parse for OptionsToken {
    fn parse(tokens: ParseStream) -> Result<Self> {
        let opt_token: OptionsToken;
        let short;
        let arg;
        let mut long;
        let mut desc = None;

        // skip -
        tokens.parse::<Token![-]>()?;
        short = tokens.parse()?;
        // skip , --
        tokens.parse::<token::Comma>()?;
        tokens.parse::<Token![-]>()?;
        tokens.parse::<Token![-]>()?;
        long = tokens.parse::<Ident>()?;

        if tokens.peek(Token![-]) {
            let long_right;
            let span = long.span();

            tokens.parse::<Token![-]>()?;
            long_right = tokens.parse::<Ident>()?;
            long = Ident::new(&format!("{}_{}", long, long_right), span);
        }

        if tokens.peek(token::Lt) {
            // skip <
            tokens.parse::<token::Lt>()?;
            let name = tokens.parse()?;
            let ty= if tokens.peek(token::Dot3) {
                tokens.parse::<token::Dot3>()?;
                ArgumentType::RequiredMultiple
            } else {
                ArgumentType::RequiredSingle
            };

            arg = Some(Argument {
                name,
                ty,
            });
            tokens.parse::<token::Gt>()?;

        } else if tokens.peek(token::Bracket) {
            let content;
            bracketed!(content in tokens);
            let name = content.parse()?;
            let ty = if content.peek(token::Dot3) {
                content.parse::<token::Dot3>()?;
                ArgumentType::OptionalMultiple
            } else {
                ArgumentType::OptionalSingle
            };

            arg = Some(Argument {
                name,
                ty,
            });
        } else {
            arg = None;
        }

        if tokens.peek(token::Comma) && tokens.peek2(LitStr) {
            tokens.parse::<token::Comma>()?;
            desc = tokens.parse()?;
        }

        opt_token = OptionsToken {
            short,
            long,
            arg,
            desc,
        };

        Ok(opt_token)
    }
}

impl ToTokens for OptionsToken {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let OptionsToken {
            short,
            long,
            arg,
            desc,
            ..
        } = self;
        let short = format!("{}", short);
        let long = format!("{}", long);
        let desc = if let Some(litstr) = desc {
            quote! {
                Some(String::from(#litstr))
            }
        } else {
            quote!(None)
        };
        let arg = if let Some(a) = arg {
            quote!(Some(#a))
        } else {
            quote!(None)
        };

        let expand = quote! {
            _commander_rust_Options {
                short: String::from(#short),
                long: String::from(#long),
                arg: #arg,
                desc: #desc,
            }
        };

        expand.to_tokens(tokens);
    }
}