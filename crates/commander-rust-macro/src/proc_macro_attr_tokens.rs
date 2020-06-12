#![allow(dead_code)]

use quote::{ ToTokens, quote, format_ident };
use proc_macro2::{ TokenStream as TokenStream2, Span as Span2 };
use syn::{ Ident, LitStr, Token, bracketed, token };
use syn::parse::{ Parse, ParseStream, Result };
use std::collections::HashSet;
use std::fmt;
use crate::utils::{ import_raw_type, import_raw_trait };
use crate::utils::{ TOKEN_ARGUMENT_TYPE, TOKEN_ARGUMENT,
                    TOKEN_OPTIONS, TOKEN_SUB_COMMAND,
                    TRAIT_PUSH_ARGUMENT, TRAIT_PUSH_OPTIONS,
                    TOKEN_COMMAND, TRAIT_PUSH_SUB_COMMAND };
use crate::errors::compile_error;
use crate::errors::msg::{ MULTIPLY_ARGUMENT_IS_ONLY_LAST, ARGUMENTS_ORDER_ERROR, ARGUMENT_IS_NON_DUPLICATED };

#[derive(Debug, Clone)]
pub(crate) enum ArgumentType {
    RequiredSingle,
    OptionalSingle,
    RequiredMultiple,
    OptionalMultiple,
}

impl ArgumentType {
    pub(crate) fn is_multiply(&self) -> bool {
        match self {
            &ArgumentType::RequiredMultiple | &ArgumentType::OptionalMultiple => true,
            _ => false,
        }
    }

    pub(crate) fn is_required(&self) -> bool {
        match self {
            &ArgumentType::RequiredSingle | &ArgumentType::RequiredMultiple => true,
            _ => false,
        }
    }
}

impl ToTokens for ArgumentType {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let hidden_ident = import_raw_type(vec![TOKEN_ARGUMENT_TYPE]);
        let ty = match &self {
            ArgumentType::RequiredSingle => quote! { #hidden_ident::RequiredSingle },
            ArgumentType::RequiredMultiple => quote! { #hidden_ident::RequiredMultiple },
            ArgumentType::OptionalSingle => quote! { #hidden_ident::OptionalSingle },
            ArgumentType::OptionalMultiple => quote! { #hidden_ident::OptionalMultiple },
        };

        ty.to_tokens(stream);
    }
}

#[derive(Debug)]
pub(crate) struct Words {
    pub(crate) inner: Vec<Ident>,
}

impl Words {
    pub(crate) fn to_ident(&self) -> Ident {
        if !self.inner.is_empty() {
            let mut idents = self.inner[0].clone();

            for ident in self.inner.iter().skip(1) {
                idents = format_ident!("{}_{}", idents, ident);
            }

            idents
        } else {
            Ident::new("", Span2::call_site())
        }
    }
}

impl fmt::Display for Words {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = if self.inner.is_empty() {
            String::new()
        } else {
            format!("{}", self.inner[0])
        };

        for word_ident in self.inner.iter().skip(1) {
            str = format!("{}-{}", str, word_ident);
        }

        write!(f, "{}", str)
    }
}

// parser for format like "dog-and-cat"
impl Parse for Words {
    fn parse(stream: ParseStream) -> Result<Self> {
        let mut words = Words {
            inner: vec![stream.parse::<Ident>()?],
        };

        while stream.peek(Token![-]) {
            stream.parse::<Token![-]>()?;
            words.inner.push(stream.parse::<Ident>()?);
        }

        Ok(words)
    }
}

impl ToTokens for Words {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        if !self.inner.is_empty() {
            let mut words_ident = format_ident!("{}", self.inner[0]);

            for word_ident in self.inner.iter().skip(1) {
                words_ident = format_ident!("{}_{}", words_ident, word_ident);
            }

            let expr = quote! { #words_ident };
            expr.to_tokens(stream);
        }
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub(crate) struct Argument {
    pub(crate) name: Ident,
    pub(crate) ty: ArgumentType,
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp = match &self.ty {
            ArgumentType::RequiredSingle => format!("<{}>", self.name),
            ArgumentType::RequiredMultiple => format!("<..{}>", self.name),
            ArgumentType::OptionalSingle => format!("[{}]", self.name),
            ArgumentType::OptionalMultiple => format!("[..{}]", self.name),
        };

        write!(f, "{}", tmp)
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Argument { name, ty } = self;
        let name = format!("{}", name);
        let argument_name = import_raw_type(vec![TOKEN_ARGUMENT]);
        let argument_expr = quote! {
            #argument_name {
                name: String::from(#name),
                ty: #ty,
            }
        };

        argument_expr.to_tokens(stream);
    }
}

impl Parse for Argument {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.peek(token::Lt) {
            // skip <
            stream.parse::<token::Lt>()?;
            // .. and ... are both ok
            // Note, "..." will be parsed into two patterns, one are ".." and ".", another is "..."
            // So parsed `token::Dot3` firstly
            let (name, ty) = if stream.peek(token::Dot3) || stream.peek(token::Dot2) {
                if stream.peek(token::Dot3) {
                    stream.parse::<token::Dot3>()?;
                } else {
                    stream.parse::<token::Dot2>()?;
                }
                (stream.parse::<Ident>()?, ArgumentType::RequiredMultiple)
            } else {
                (stream.parse::<Ident>()?, ArgumentType::RequiredSingle)
            };

            stream.parse::<token::Gt>()?;
            Ok(Argument {
                name,
                ty,
            })
        } else if stream.peek(token::Bracket) {
            let content;
            bracketed!(content in stream);
            // .. and ... are both ok
            // Note, "..." will be parsed into two patterns, one are ".." and ".", another is "..."
            // So parsed `token::Dot3` firstly
            let (name, ty) = if content.peek(token::Dot3) || content.peek(token::Dot2) {
                if content.peek(token::Dot3) {
                    content.parse::<token::Dot3>()?;
                } else {
                    content.parse::<token::Dot2>()?;
                }
                (content.parse::<Ident>()?, ArgumentType::OptionalMultiple)
            } else {
                (content.parse::<Ident>()?, ArgumentType::OptionalSingle)
            };

            Ok(Argument {
                name,
                ty,
            })
        } else {
            Ok(Argument {
                name: Ident::new("", Span2::call_site()),
                ty: ArgumentType::OptionalSingle,
            })
        }
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub(crate) struct Arguments {
    pub(crate) inner: Vec<Argument>,
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = if self.inner.is_empty() {
            String::new()
        } else {
            format!("{}", self.inner[0])
        };

        for arg in self.inner.iter().skip(1) {
            args = format!("{} {}", args, arg);
        }

        write!(f, "{}", args)
    }
}

impl ToTokens for Arguments {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let args = &self.inner;
        let expr = quote! { vec![#(#args),*] };

        expr.to_tokens(stream);
    }
}

impl Parse for Arguments {
    fn parse(stream: ParseStream) -> Result<Self> {
        let mut inner = vec![];

        while stream.peek(token::Lt) || stream.peek(token::Bracket) {
            if let Ok(arg) = stream.parse::<Argument>() {
                inner.push(arg);
            }
        }

        Ok(Arguments {
            inner,
        })
    }
}

impl Arguments {
    pub fn try_get_errors(&self) -> Option<TokenStream2> {
        let mut opt_start = false;
        let mut names = HashSet::new();

        for (idx, arg) in self.inner.iter().enumerate() {
            if !arg.ty.is_required() {
                opt_start = true;
            }

            if idx != self.inner.len() - 1 && arg.ty.is_multiply() {
                // only last argument could be optional, error
                return Some(compile_error(arg.name.span(), MULTIPLY_ARGUMENT_IS_ONLY_LAST));
            }


            if arg.ty.is_required() && opt_start {
                // all optional arguments should follow all required arguments, error
                // for instance, <a> <b> [c] [d] is valid, but <a> [b] <c> [d] is invalid
                return Some(compile_error(arg.name.span(), ARGUMENTS_ORDER_ERROR));
            }

            let name = format!("{}", arg.name);

            if names.contains(&name) {
                return Some(compile_error(arg.name.span(), ARGUMENT_IS_NON_DUPLICATED));
            } else {
                names.insert(name);
            }
        }

        None
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub(crate) struct Options {
    pub(crate) short: Option<Ident>,
    pub(crate) long: Words,
    pub(crate) opt_args: Arguments,
    pub(crate) desc: Option<LitStr>,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(short) = &self.short {
            write!(f, "{}", format!("-{}, ", short)).unwrap();
        }

        write!(f, "--{}", self.long).unwrap();

        if !self.opt_args.inner.is_empty() {
            write!(f, " {}", self.opt_args).unwrap();
        }

        if let Some(desc) = &self.desc {
            write!(f, r#", "{}""#, desc.value()).unwrap();
        }

        write!(f, "")
    }
}

impl Parse for Options {
    fn parse(stream: ParseStream) -> Result<Self> {
        let short;
        let long;
        let opt_args;
        let desc;

        // parse -s
        short = if stream.peek(Token![-]) && !stream.peek2(Token![-]) {
            stream.parse::<Token![-]>()?;
            let tmp = stream.parse::<Ident>()?;
            // skip ,
            stream.parse::<token::Comma>()?;
            Some(tmp)
        } else {
            None
        };


        // parse --long-option
        stream.parse::<Token![-]>()?;
        stream.parse::<Token![-]>()?;
        long = stream.parse::<Words>()?;

        // parse arguments <a> <b> and more
        opt_args = stream.parse::<Arguments>()?;

        // parse description if it exists
        if stream.peek(token::Comma) && stream.peek2(LitStr) {
            stream.parse::<token::Comma>()?;
            desc = stream.parse()?;
        } else {
            desc = None;
        }

        Ok(Options {
            short,
            long,
            opt_args,
            desc,
        })
    }
}

impl ToTokens for Options {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Options { short, long, opt_args, desc } = self;
        let opt_args = &opt_args.inner;
        let short = if let Some(tmp) = short {
            let tmp = format!("{}", tmp);
            quote! { Some(String::from(#tmp)) }
        } else {
            quote! { None }
        };
        let long = format!("{}", long);
        let options_name = import_raw_type(vec![TOKEN_OPTIONS]);
        let trait_needed = import_raw_trait(TRAIT_PUSH_ARGUMENT);
        let description = if let Some(lit_str) = desc {
            quote! { Some(String::from(#lit_str)) }
        } else {
            quote! { None }
        };
        let options_expr = quote! {
            {
                // `commander_rust::traits::PushArgument` needed
                #trait_needed;
                let mut options = #options_name::new(
                    #short,
                    String::from(#long),
                    #description
                );
                #(options.push_argument(#opt_args);)*
                options
            }
        };

        options_expr.to_tokens(stream);
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub(crate) struct SubCommand {
    pub(crate) belong: Option<Ident>,
    pub(crate) name: Ident,
    pub(crate) cmd_args: Arguments,
    pub(crate) options: Vec<Options>,
    pub(crate) desc: Option<LitStr>,
}

impl fmt::Display for SubCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", self.name)).unwrap();

        if let Some(desc) = &self.desc {
            write!(f, r#", "{}""#, desc.value()).unwrap();
        }

        write!(f, "")
    }
}

// pattern: name [<a> <b> [c] [..d]], ["description"]
impl Parse for SubCommand {
    fn parse(stream: ParseStream) -> Result<Self> {
        let name = stream.parse::<Ident>()?;
        let cmd_args = stream.parse::<Arguments>()?;
        let desc = if stream.peek(token::Comma) {
            stream.parse::<token::Comma>()?;
            Some(stream.parse::<LitStr>()?)
        } else if stream.peek(LitStr) {
            Some(stream.parse::<LitStr>()?)
        } else {
            None
        };

        Ok(SubCommand {
            belong: None,
            name,
            cmd_args,
            options: vec![],
            desc,
        })
    }
}

impl ToTokens for SubCommand {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let SubCommand {
            belong,
            name,
            cmd_args,
            options,
            desc,
        } = self;
        let belong = if let Some(belong) = belong {
            belong.to_string()
        } else {
            String::new()
        };
        let name = format!("{}", name);
        let cmd_args = &cmd_args.inner;
        let cmd_name = import_raw_type(vec![TOKEN_SUB_COMMAND]);
        let desc = if let Some(lit_str) = desc {
            quote! { Some(String::from(#lit_str)) }
        } else {
            quote! { None }
        };
        let traits_needed = vec![
            import_raw_trait(TRAIT_PUSH_ARGUMENT),
            import_raw_trait(TRAIT_PUSH_OPTIONS),
        ];
        let cmd_expr = quote! {
            {
                #(#traits_needed;)*
                let mut sub_cmd = #cmd_name::new(
                    String::from(#belong),
                    String::from(#name),
                    #desc
                );
                #(sub_cmd.push_argument(#cmd_args);)*
                #(sub_cmd.push_option(#options);)*
                sub_cmd
            }
        };

        cmd_expr.to_tokens(stream);
    }
}

#[derive(Debug)]
pub(crate) struct Command {
    pub(crate) name: Ident,
    pub(crate) sub_cmds: Vec<SubCommand>,
    pub(crate) cmd_args: Arguments,
    pub(crate) options: Vec<Options>,
    pub(crate) desc: Option<LitStr>,
}

impl Parse for Command {
    fn parse(stream: ParseStream) -> Result<Self> {
        let name = stream.parse::<Ident>()?;
        let cmd_args = stream.parse::<Arguments>()?;
        let desc = if stream.peek(token::Comma) {
            stream.parse::<token::Comma>()?;
            Some(stream.parse::<LitStr>()?)
        } else {
            None
        };

        Ok(Command {
            name,
            sub_cmds: vec![],
            cmd_args,
            options: vec![],
            desc,
        })
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Command {
            name,
            sub_cmds,
            cmd_args,
            options,
            desc,
        } = self;
        let cmd_name = name.to_string();
        let cmd_args = &cmd_args.inner;
        let desc = if let Some(desc) = desc {
            quote! { Some(String::from(#desc)) }
        } else {
            quote! { None }
        };
        let hidden_output = import_raw_type(vec![TOKEN_COMMAND]);
        let traits_needed = vec![
            import_raw_trait(TRAIT_PUSH_ARGUMENT),
            import_raw_trait(TRAIT_PUSH_OPTIONS),
            import_raw_trait(TRAIT_PUSH_SUB_COMMAND),
        ];
        let expr = quote! {
            {
                #(#traits_needed;)*
                let mut cmd = #hidden_output::new(String::from(#cmd_name), #desc);
                #(cmd.push_sub_command(#sub_cmds);)*
                #(cmd.push_argument(#cmd_args);)*
                #(cmd.push_option(#options);)*
                cmd
            }
        };

        expr.to_tokens(stream);
    }
}