#![recursion_limit = "256"]
#![allow(unused_mut, dead_code)]

mod tokens;
mod errors;
mod tools;

extern crate proc_macro;

use proc_macro::{ TokenStream };
use std::collections::HashMap;

use lazy_static::lazy_static;
use proc_macro2::Span as Span2;
use quote::quote;
use syn::{ Ident, ItemFn, parse_macro_input };
use tokens::{ CommandToken, OptionsToken};

use crate::errors::{error, DONT_MATCH, ENTRY_ONLY_MAIN, NO_SUB_CMD_NAMES_MAIN, OPT_DUPLICATE_DEFINITION};
use crate::tools::generate_call_fn;

macro_rules! prefix {
    ($($i: tt),*) => {
        {
            let mut prefix_str = String::from("_commander_rust");
            $(
                prefix_str.push_str(&format!("_{}", $i));
            )*
            prefix_str
        }
    };

    ($e: expr) => {
        {
            format!("_commaner_rust_{}", $e)
        }
    }
}

// generating the Command Ident for runtime
macro_rules! command_ident {
    ($($i: tt),*) => {
        {
            format!("{}_{}", prefix!($($i,)*), "Command")
        }
    };
}

// generating the Options Ident for runtime
macro_rules! options_ident {
    ($($i: tt),*) => {
        {
            format!("{}_{}", prefix!($($i,)*), "Options")
        }
    };
}

macro_rules! application_ident {
    ($($i: tt),*) => {
        {
            format!("{}_{}", prefix!($($i,)*), "Application")
        }
    };
}

macro_rules! import {
    ($o: ident as $r: ident) => {
        quote! {
            use commander_rust::{ $o as $r };
        }
    };
    ($o: ident as $r: ident from $f: path) => {
        quote! {
            use $f::{ $o as $r };
        }
    }
}

lazy_static! {
    static ref COMMAND_OPTIONS: std::sync::Mutex<HashMap<String, Vec<String>>> = std::sync::Mutex::new(HashMap::new());
    static ref GET_FN_NAMES: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(vec![]);
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
    let ItemFn {
        ident,
        decl,
        ..
    } = &method;
    let name = format!("{}", ident);
    let get_fn = Ident::new(&prefix!(name), ident.span());
    let cmd_token = Ident::new(&command_ident!(), ident.span());
    let opts = COMMAND_OPTIONS.lock().unwrap();
    let mut get_fn_names = GET_FN_NAMES.lock().unwrap();
    let mut get_fns = vec![];


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
    let raw_ident = Ident::new(&prefix!("Raw"), Span2::call_site());
    let call_fn = generate_call_fn(&decl.inputs, &call_fn_name, &raw_ident, &ident);

    if format!("{}", command.name) != name {
        error(DONT_MATCH);
    }

    if name == "main" {
        error(NO_SUB_CMD_NAMES_MAIN);
    }

    command.check();

    TokenStream::from(quote! {
        fn #get_fn() -> #cmd_token {
            let mut command = #command;
            let mut fns = CALL_FNS.lock().unwrap();

            if !fns.contains_key(#name) {
                fns.insert(String::from(#name), #call_fn_name);
            }

            command.opts = {
                let mut v = vec![];
                #(
                    v.push(#get_fns());
                )*
                v
            };
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
    let ItemFn {
        ident,
        ..
    } = &method;
    let name = format!("{}", ident);
    let opt_name = format!("{}", option.long);
    let fn_name = prefix!(name, opt_name);
    let get_fn = Ident::new(&fn_name, option.long.span());
    let opt_token = Ident::new(&options_ident!(), ident.span());
    let mut opts = COMMAND_OPTIONS.lock().unwrap();

    if opts.contains_key(&name) {
        if let Some(v) = opts.get_mut(&name) {
            if v.contains(&opt_name) {
                error(OPT_DUPLICATE_DEFINITION)
            } else {
                v.push(opt_name);
            }
        }
    } else {
        opts.insert(name, vec![opt_name]);
    }

    TokenStream::from(quote! {
        fn #get_fn() -> #opt_token {
            #option
        }

        #method
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
pub fn entry(_: TokenStream, main: TokenStream) -> TokenStream {
    let main: ItemFn = parse_macro_input!(main as ItemFn);
    let ItemFn {
        ident,
        ..
    } = &main;
    let target = format!("{}", ident);
    let opts = COMMAND_OPTIONS.lock().unwrap();
    let imports = vec![
        import!(Argument as _commander_rust_Argument),
        import!(ArgumentType as _commander_rust_ArgumentType),
        import!(Command as _commander_rust_Command),
        import!(Options as _commander_rust_Options),
        import!(Raw as _commander_rust_Raw),
        import!(normalize as _commander_rust_normalize),
        import!(Instance as _commander_rust_Instance),
        import!(ls as _commander_rust_ls),
        import!(Application as _commander_rust_Application),
        import!(Cli as _commander_rust_Cli),
    ];
    let get_fn = Ident::new(&prefix!("main"), ident.span());
    let app_token = Ident::new(&application_ident!(), ident.span());
    let get_fn_names = GET_FN_NAMES.lock().unwrap();
    let mut get_cmds_fns = vec![];
    let mut get_opts_fns = vec![];

    // init can be used with fn main only
    if target != String::from("main") {
        error(ENTRY_ONLY_MAIN);
    }

    if let Some(v) = opts.get("main") {
        for i in v {
            get_opts_fns.push(Ident::new(&prefix!("main", i), ident.span()));
        }
    }

    for i in get_fn_names.iter() {
        get_cmds_fns.push(Ident::new(&prefix!(i), ident.span()));
    }

    TokenStream::from(quote! {
        mod _commander_rust_Inner {
            use crate::_commander_rust_ls;
            use crate::_commander_rust_Raw;
            use crate::_commander_rust_Cli;

            type Raw = _commander_rust_Raw;
            type Map = std::collections::HashMap<String, fn(raws: &Vec<Raw>, app: _commander_rust_Cli)>;
            type Mutex = std::sync::Mutex<Map>;

            _commander_rust_ls! {
               pub static ref CALL_FNS: Mutex = Mutex::new(Map::new());
            }

            pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
            pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
            pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
        }

        use _commander_rust_Inner::{ CALL_FNS, VERSION, DESCRIPTION, APP_NAME };
        #(#imports)*

        #main

        fn #get_fn() -> #app_token {
            let mut application = #app_token {
                name: String::from(APP_NAME),
                desc: String::from(DESCRIPTION),
                opts: vec![],
                cmds: vec![],
            };

            application.opts = {
                let mut v = vec![];
                #(
                    v.push(#get_opts_fns());
                )*
                v
            };

            application.cmds = {
                let mut v = vec![];
                #(
                    v.push(#get_cmds_fns());
                )*
                v
            };
            application
        }
    })
}


/// Run cli now.
///
/// `run!()` instead of `run()`. You can use this macro to get application of cli.
/// See `Application` for more details.
///
#[proc_macro]
pub fn run(_: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        {
            let mut app = _commander_rust_main();
            let ins = _commander_rust_normalize(std::env::args().into_iter().collect::<Vec<String>>());

            app.derive();

            let cli = _commander_rust_Cli::from(&ins, &app);
            let fns = CALL_FNS.lock().unwrap();

            if let Some(cli) = cli {
                if let Some(f) = fns.get(&cli.get_name()) {
                    if cli.has("help") || cli.has("h") {
                        if cli.cmd.is_some() {
                            let mut showed = false;

                            for cmd in &app.cmds {
                                if cmd.name == cli.get_name() {
                                    println!("{:#?}", cmd);
                                    showed = true;
                                    break;
                                }
                            }

                            if !showed {
                                println!("{:#?}", app);
                            }
                        } else {
                            println!("{:#?}", app);
                        }
                    } else if cli.has("version") || cli.has("V") {
                        println!("version: {}", VERSION);
                    } else {
                        f(&cli.get_raws(), cli);
                    }
                } else if cli.has("version") || cli.has("V") {
                    println!("version: {}", VERSION);
                } else {
                    println!("{:#?}", app);
                }
            }

            app
        }
    })
}