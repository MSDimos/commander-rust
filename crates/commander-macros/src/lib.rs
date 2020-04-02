#![recursion_limit = "256"]

mod tokens;
mod errors;
mod tools;

extern crate proc_macro;

use proc_macro::{ TokenStream };
use proc_macro2::{ TokenStream as TokenStream2, Span as Span2 };
use std::collections::HashMap;

use lazy_static::lazy_static;
use quote::quote;
use syn::{ Ident, ItemFn, parse_macro_input, ReturnType };
use tokens::{ CommandToken, OptionsToken};
use std::sync::{Mutex};

use crate::errors::{DON_NOT_MATCH, ENTRY_ONLY_MAIN, NO_SUB_CMD_NAMES_MAIN, OPT_DUPLICATE_DEFINITION, compile_error_info, DIRECT_ONLY_ONCE};
use crate::tools::generate_call_fn;
use crate::tokens::{PureArguments, check_arguments};
use syn::spanned::Spanned;

/// adds _commander_rust prefix to the name e.g.
/// prefix!("main") -> _commander_rs_main
/// prefix!("main", "silent") -> _commander_rs_main_silent
macro_rules! prefix {
    ($($i: tt),*) => {
        {
            let mut prefix_str = String::new();

            prefix_str.push_str("_commander_rust");
            $(
                prefix_str.push_str(&format!("_{}", $i));
            )*
            prefix_str
        }
    };

    ($e: expr) => {
        {
            format!("_commander_rust_{}", $e)
        }
    }
}


lazy_static! {
    static ref COMMAND_OPTIONS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
    static ref OPTIONS: Mutex<Vec<String>> = Mutex::new(vec![]);

    // Ever declared command name will is pushed into here (compile time).
    static ref GET_FN_NAMES: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref DIRECT_NAME: Mutex<Option<String>> = Mutex::new(None);
}


/// Define command.
///
/// # Format
///
/// `#[command(sub_cmd $(<rs>|[os]|<rm...>|[om...]),* , desc)]`, explain:
/// - `sub_name`: the name of sub-command you defined, it should be same as the name of command processing function
/// - `$(<rs>|[os]|<rm...>|[om...]),*`: arguments list, divided by comma. Only four types of arguments: <RequiredSingle>,[OptionalSingle],<RequiredMultiple>,[OptionalMultiple]
/// - `desc`: description of this sub-command, it's using for display help information.
///
/// # Note
///
/// `#[command]` should be placed after all `#[option]`
#[proc_macro_attribute]
pub fn command(cmd: TokenStream, method: TokenStream) -> TokenStream {
    let method: ItemFn = parse_macro_input!(method as ItemFn);
    let args = &method.sig.inputs;
    let ret = &method.sig.output;
    let ident = &method.sig.ident;
    let name = format!("{}", ident);
    let get_fn = Ident::new(&prefix!(name), ident.span());
    let opts = COMMAND_OPTIONS.lock().unwrap();
    let mut get_fn_names = GET_FN_NAMES.lock().unwrap();
    let mut get_fns = vec![];

    // clear all options in OPTIONS
    OPTIONS.lock().unwrap().clear();

    if let Some(v) = opts.get(&name) {
        for i in v {
            get_fns.push(Ident::new(&prefix!(name, i), ident.span()));
        }
    }

    if !get_fn_names.contains(&name) {
        get_fn_names.push(name.clone());
    }

    let command: CommandToken = parse_macro_input!(cmd as CommandToken);
    // generating call function， because we can't call unstable (uncertain quantity parameters) function
    let call_fn_name = Ident::new(&prefix!(name, "call"), ident.span());
    let call_fn = generate_call_fn(args, &call_fn_name, ident, ret);
    let mut error_info = check_arguments(&command.args);

    if format!("{}", command.name) != "main" && format!("{}", command.name) != name {
        error_info = compile_error_info(command.name.span(), DON_NOT_MATCH);
    }

    if name == "main" {
        error_info = compile_error_info(ident.span(), NO_SUB_CMD_NAMES_MAIN)
    }

    TokenStream::from(quote! {
        #error_info

        fn #get_fn() -> ::commander_rust::Command {
            COMMANDER.register_command_handler(String::from(#name), #call_fn_name);

            let mut command = #command;
            command.opts = vec![#(#get_fns(),)*];

            command
        }

        #call_fn

        #method
    })
}


/// Define option of command or public.
///
/// # Format
///
/// it's similar with `command`. The only difference between them is the length of arguments.
///
/// `#[command(sub_cmd <rs>|[os]|<rm...>|[om...] , desc)]`, explain:
/// - `sub_name`: the name of sub-command you defined, it should be same as the name of command processing function
/// - `<rs>|[os]|<rm...>|[om...]`: only one. Only four types of arguments: <RequiredSingle>,[OptionalSingle],<RequiredMultiple>,[OptionalMultiple]
/// - `desc`: description of this sub-command, it's using for display help information.
///
/// # Note
///
/// all `#[option]` should be placed before `#[command]`
///
#[proc_macro_attribute]
pub fn option(opt: TokenStream, method: TokenStream) -> TokenStream {
    let option: OptionsToken = parse_macro_input!(opt as OptionsToken);
    let method: ItemFn = parse_macro_input!(method as ItemFn);
    let ident = &method.sig.ident;
    let name = format!("{}", ident);
    let opt_name = format!("{}", option.long);
    let fn_name = prefix!(name, opt_name);
    let get_fn = Ident::new(&fn_name, option.long.span());
    let mut opts = COMMAND_OPTIONS.lock().unwrap();
    let mut error_info = TokenStream2::new();
    let mut all_opts = OPTIONS.lock().unwrap();


    // check if options are duplicate definition
    if all_opts.contains(&format!("{}", option.short)) {
        error_info = compile_error_info(option.short.span(), OPT_DUPLICATE_DEFINITION);
    } else if all_opts.contains(&format!("{}", option.long)) {
        error_info = compile_error_info(option.long.span(), OPT_DUPLICATE_DEFINITION);
    } else {
        all_opts.push(format!("{}", option.short));
        all_opts.push(format!("{}", option.long));

        if opts.contains_key(&name) {
            if let Some(v) = opts.get_mut(&name) {
                v.push(opt_name);
            }
        } else {
            opts.insert(name, vec![opt_name]);
        }
    }

    if error_info.is_empty() {
        TokenStream::from(quote! {
            #error_info

            fn #get_fn() -> ::commander_rust::Options {
                #option
            }

            #method
        })
    } else {
        TokenStream::from(quote! {
            #error_info

            #method
        })
    }

}

/// Define direct function of CLI
///
/// It allows users that don't use sub-command. It can be used directly.
///
/// # example
///
/// ```ignore
/// #[direct(<a> <b> [c] [d])]
///  fn direct(a: String, b: String) {
///      println!("hello! {} {}", a, b);
///  }
/// ```
/// When you input `[pkg-name] 1 2 3`, cli will print "hello! 1 2".
/// So it allows you that don't need to define a sub-command anymore in some simple situations.
///
/// # Format
///
/// Only accept arguments, such as `#[direct(<a> <b> <c>)]`.

#[proc_macro_attribute]
pub fn direct(pure_args: TokenStream, func: TokenStream) -> TokenStream {
    let func: ItemFn = parse_macro_input!(func as ItemFn);
    let ident = &func.sig.ident;
    let ret = &func.sig.output;
    let args = &func.sig.inputs;
    let name = format!("{}", ident);
    let pure_args: PureArguments = parse_macro_input!(pure_args as PureArguments);
    let direct_fn: &mut Option<String> = &mut (*DIRECT_NAME.lock().unwrap());
    let direct_get_fn = Ident::new(&prefix!(name), ident.span());
    let call_fn_name = Ident::new(&prefix!(name, "call"), ident.span());
    let call_fn = generate_call_fn(args, &call_fn_name, ident, ret);
    let mut error_info: TokenStream2 = check_arguments(&pure_args.0);


    if let Some(_) = direct_fn {
        error_info = compile_error_info(pure_args.span(), DIRECT_ONLY_ONCE);
    } else {
        *direct_fn = Some(format!("{}", ident));
    }

    TokenStream::from(quote! {
        #error_info

        #func

        fn #direct_get_fn() -> Vec<::commander_rust::Argument> {
            COMMANDER.register_direct_handler(#call_fn_name);
            #pure_args
        }

        #call_fn
    })
}

/// Define entry of CLI.
///
/// Only using for `main` function. No parameter needed.
///
/// # Note
///
/// If you want to define public options, put all options before `#[entry]`.
/// In other word, you can regard `#[entry]` as `#[command]` without parameters.
///
#[proc_macro_attribute]
pub fn entry(pure_arguments: TokenStream, main: TokenStream) -> TokenStream {
    let pure_args: PureArguments = parse_macro_input!(pure_arguments as PureArguments);
    let main: ItemFn = parse_macro_input!(main as ItemFn);
    let ident = &main.sig.ident;
    let ret = &main.sig.output;
    let out = match ret {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, t) => quote! { #t },
    };
    let target = format!("{}", ident);
    let mut error_info = check_arguments(&pure_args.0);

    // entry can only be used with fn main.
    if target != String::from("main") {
        error_info = compile_error_info(ident.span(), ENTRY_ONLY_MAIN);
    }


    let entry = quote! {
        #error_info

        commander_rust::ls! {
            pub static ref COMMANDER: ::commander_rust::Commander<#out> =
            ::commander_rust::Commander::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_DESCRIPTION")
            );
        }

        #main
    };
    entry.into()
}

/// Run cli now.
///
/// `run!()` instead of `run()`. You can use this macro to get application of cli.
/// See `Application` for more details.
///
#[proc_macro]
pub fn run(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    let mut get_cmds_fns:Vec<Ident> = vec![];
    let mut get_opts_fns:Vec<Ident> = vec![];
    let direct_fn = &(*DIRECT_NAME.lock().unwrap());

    // Options for main are global. For each option e.g. `--foo` we will collect
    // identifier (in this example `_commander_rs_main_foo`) into
    // `get_opts_fns`.
    let opts = COMMAND_OPTIONS.lock().unwrap();
    if let Some(v) = opts.get("main") {
        for i in v {
            get_opts_fns.push(Ident::new(&prefix!("main", i), Span2::call_site()));
        }
    }

    // For each registered command e.g. `find` we collect identifier
    // (in this example `_commander_rs_find`) into `get_cmds_fns`.
    let get_fn_names = GET_FN_NAMES.lock().unwrap();
    for i in get_fn_names.iter() {
        get_cmds_fns.push(Ident::new(&prefix!(i), Span2::call_site()));
    }

    let direct_get_fn = if let Some(df) = direct_fn {
        let direct_get = Ident::new(&prefix!(df), Span2::call_site());
        quote! { #direct_get() }
    } else {
        quote! { vec![] }
    };

    let run = quote! {
        COMMANDER.run(
            vec![#(#get_cmds_fns(),)*],
            vec![#(#get_opts_fns(),)*],
            #direct_get_fn
        )
    };
    run.into()
}
