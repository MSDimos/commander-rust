use syn::{Ident, FnArg, token};
use proc_macro2::{TokenStream as TokenStream2, Span as Span2};
use quote::{quote, format_ident, quote_spanned};
use std::fmt::Display;
use syn::punctuated::Punctuated;
use crate::proc_macro_attr_tokens::Arguments;
use crate::errors::compile_error;
use std::collections::HashMap;
use syn::spanned::Spanned;
use crate::errors::msg::UNUSED_ARGUMENT;

const TOKEN_PREFIX: &str = "_commander_rust_prefix_";
const TOKEN_SUFFIX: &str = "_commander_rust_suffix_";

pub(crate) const TOKEN_ARGUMENT_TYPE: &str = "ArgumentType";
pub(crate) const TOKEN_ARGUMENT: &str = "Argument";
pub(crate) const TOKEN_OPTIONS: &str = "Options";
pub(crate) const TOKEN_SUB_COMMAND: &str = "SubCommand";
pub(crate) const TOKEN_COMMAND: &str = "Command";
pub(crate) const TOKEN_SEGMENT: &str = "Segment";
pub(crate) const TOKEN_SEGMENT_WRAPPER: &str = "SegmentWrapper";
pub(crate) const TOKEN_APPLICATION: &str = "Application";
pub(crate) const TOKEN_ARG: &str = "Arg";
pub(crate) const TOKEN_ARGS: &str = "Args";
pub(crate) const TOKEN_MIXED: &str = "Mixed";
pub(crate) const TOKEN_TERMINATOR_KIND: &str = "TerminatorKind";
pub(crate) const TRAIT_PUSH_ARGUMENT: &str = "PushArgument";
pub(crate) const TRAIT_PUSH_OPTIONS: &str = "PushOptions";
pub(crate) const TRAIT_PUSH_SUB_COMMAND: &str = "PushSubCommand";
pub(crate) const TRAIT_FROM_ARG: &str = "FromArg";
pub(crate) const TRAIT_FROM_ARGS: &str = "FromArgs";
pub(crate) const TRAIT_FROM_APP: &str = "FromApp";
pub(crate) const PATH_PARSER: &str = "parser";
pub(crate) const PATH_TRAITS: &str = "traits";
pub(crate) const FN_CALL_EXTRA_TOKEN: &str = "extra_token";

pub(crate) fn decorate_ident(source: Ident) -> Ident {
    format_ident!("{}{}{}", TOKEN_PREFIX, source, TOKEN_SUFFIX)
}

pub(crate) fn decorate_raw_idents<T: Display>(sources: Vec<T>) -> Ident {
    if sources.is_empty() {
        Ident::new("", Span2::call_site())
    } else {
        let mut idents = TOKEN_PREFIX.to_string();

        for (idx, source) in sources.iter().enumerate() {
            if idx == 0 {
                idents = format!("{}{}", idents, source);
            } else if idx == sources.len() - 1 {
                idents = format!("{}_{}{}", idents, source, TOKEN_SUFFIX);
            } else {
                idents = format!("{}_{}", idents, source);
            }
        }

        format_ident!("{}", idents)
    }
}

pub(crate) fn import_raw_type<T: Display>(source: Vec<T>) -> TokenStream2 {
    let mut idents: Vec<Ident> = source
        .into_iter()
        .map(|s| format_ident!("{}", format!("{}", s)))
        .collect();
    idents.insert(0, format_ident!("commander_rust"));

    quote! { #(#idents)::* }
}

pub(crate) fn import_raw_trait<T: Display>(source: T) -> TokenStream2 {
    let ident = format_ident!("{}", format!("{}", source));
    quote! { use commander_rust::traits::#ident }
}

//
pub(crate) fn get_inputs_runtime_asserts(inputs: &Punctuated<FnArg, token::Comma>, def_args: &Arguments) -> TokenStream2 {
    let mut args_map = HashMap::new();
    let mut idents_map = HashMap::new();
    let mut exprs = vec![];
    let ty_arg = import_raw_type(vec![TOKEN_ARG]);
    let ty_args = import_raw_type(vec![TOKEN_ARGS]);
    let ty_app = import_raw_type(vec![TOKEN_APPLICATION]);
    let trait_from_arg = import_raw_type(vec![PATH_TRAITS, TRAIT_FROM_ARG]);
    let trait_from_args = import_raw_type(vec![PATH_TRAITS, TRAIT_FROM_ARGS]);
    let trait_from_app = import_raw_type(vec![PATH_TRAITS, TRAIT_FROM_APP]);

    for def_arg in def_args.inner.iter() {
        args_map.insert(def_arg.name.to_string(), def_arg.ty.clone());
        idents_map.insert(def_arg.name.to_string(), def_arg.name.clone());
    }

    for input in inputs.iter() {
        if let FnArg::Typed(pat) = input {
            if let syn::Pat::Ident(pat_name) = &*pat.pat {
                let arg_name = pat_name.ident.to_string();
                let ty = &pat.ty;
                let span = input.span();

                if let Some(def_arg_ty) = args_map.remove(&arg_name) {
                    // if argument is one of the defined arguments by `#[sub_command]` or `#[command]`
                    // it should implement the trait `FromArg` or `FromArgs`
                    // 1. if argument is defined as `<a>` or `[a]`, it should implement the trait `FromArg`
                    // 2. if argument is defined as `<..a>` or `[..a]`, it should implement the trait `FromArgs`
                    if def_arg_ty.is_multiply() {
                        exprs.push(quote_spanned! {span=>
                            {
                                let args = #ty_args(vec![]);
                                <#ty as #trait_from_args>::from_args(&args);
                            }
                        });
                    } else {
                        exprs.push(quote_spanned! {span=>
                            {
                                let arg = #ty_arg(String::new());
                                <#ty as #trait_from_arg>::from_arg(&arg);
                            }
                        });
                    }
                } else {
                    // if arguments are not,
                    // it should implement the trait `FromApp`
                    exprs.push(quote_spanned! {span=>
                        {
                            let app = <#ty_app>::default();
                            <#ty as #trait_from_app>::from_app(&app);
                        }
                    });
                }
            }
        }
    }

    for arg_name in args_map.keys() {
        if let Some(arg_ident) = idents_map.get(arg_name) {
            exprs.push(compile_error(arg_ident.span(), UNUSED_ARGUMENT));
        }
    }

    let span = inputs.span();
    let tmp = quote_spanned! {span=>
        #[allow(unused_must_use)]
        {
            #(#exprs)*
        }
    };

    tmp
}

pub(crate) fn generate_inputs(
    inputs: &Punctuated<FnArg, token::Comma>,
    def_args: &Arguments,
    app_ident: &Ident,
    is_sub_command: bool,
) -> TokenStream2 {
    let mut args_map = HashMap::new();
    let mut really_inputs = vec![];
    let ty_arg = import_raw_type(vec![TOKEN_ARG]);
    let ty_args = import_raw_type(vec![TOKEN_ARGS]);
    let ty_mixed = import_raw_type(vec![TOKEN_MIXED]);
    let trait_from_arg = import_raw_type(vec![PATH_TRAITS, TRAIT_FROM_ARG]);
    let trait_from_args = import_raw_type(vec![PATH_TRAITS, TRAIT_FROM_ARGS]);
    let trait_from_app = import_raw_type(vec![PATH_TRAITS, TRAIT_FROM_APP]);
    let fn_get_arg = if is_sub_command { format_ident!("get_sub_arg") } else { format_ident!("get_cmd_arg") };

    for def_arg in def_args.inner.iter() {
        args_map.insert(def_arg.name.to_string(), def_arg.ty.clone());
    }

    for input in inputs.iter() {
        if let FnArg::Typed(pat) = input {
            if let syn::Pat::Ident(pat_name) = &*pat.pat {
                let arg_name = pat_name.ident.to_string();
                let ty = &pat.ty;
                let span = input.span();

                if let Some(def_arg_ty) = args_map.remove(&arg_name) {
                    if def_arg_ty.is_multiply() {
                        really_inputs.push(quote_spanned! {span=>
                            {
                                let mut args = &#ty_args(vec![]);

                                if let Some(#ty_mixed::Multiply(_args)) = #app_ident.#fn_get_arg(#arg_name) {
                                    args = _args;
                                }

                                if let Ok(tmp) = <#ty as #trait_from_args>::from_args(args) {
                                    tmp
                                } else {
                                    eprintln!("parse failed, can't parse input `{}` as type `{}`", args, stringify!(#ty));
                                    std::process::exit(1);
                                }
                            }
                        });
                    } else {
                        really_inputs.push(quote_spanned! {span=>
                            {
                                let mut arg = &#ty_arg(String::new());

                                if let Some(#ty_mixed::Single(_arg)) = #app_ident.#fn_get_arg(#arg_name) {
                                    arg = _arg;
                                }

                                if let Ok(tmp) = <#ty as #trait_from_arg>::from_arg(arg) {
                                    tmp
                                } else {
                                    eprintln!("parse failed, can't parse input `{}` as type `{}`", arg, stringify!(#ty));
                                    std::process::exit(1);
                                }
                            }
                        })
                    }
                } else {
                    really_inputs.push(quote_spanned! {span=>
                        if let Ok(tmp) = <#ty as #trait_from_app>::from_app(&#app_ident) {
                            tmp
                        } else {
                            eprintln!("parse failed, can't parse type `App` as type `{}`", stringify!(#ty));
                            std::process::exit(1);
                        }
                    });
                }
            }
        }
    }

    let tmp = quote! { #(#really_inputs,)* };
    tmp
}