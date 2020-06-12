mod proc_macro_attr_tokens;
mod proc_macro_tokens;
mod errors;
mod utils;

extern crate proc_macro;

use utils::{ decorate_ident, decorate_raw_ident,
             import_raw_type, import_raw_trait,
             decorate_raw_idents, };
use utils::{ TOKEN_OPTIONS, TOKEN_SUB_COMMAND,
             GET_ONLY_COMMAND, TOKEN_COMMAND,
             TRAIT_PUSH_OPTIONS, TRAIT_PUSH_SUB_COMMAND,
             TOKEN_SEGMENT_WRAPPER, TOKEN_SEGMENT,
             TOKEN_APPLICATION, TRAIT_FROM_ARG,
             TRAIT_FROM_ARGS, TRAIT_FROM_APP };
use errors::compile_error;
use errors::msg::{ OPTION_IS_NON_DUPLICATED, SUB_CMD_IS_NON_DUPLICATED,
                   REGISTER_UNKNOWN_SUB_CMD, CMD_IS_ONLY,
                   REGISTER_UNKNOWN_CMD, DONT_NEED_PARAMETERS,
                   HAVENT_CALLED_REGISTER_FNS, DONT_CALL_REGISTER_MULTIPLE_TIMES, };
use proc_macro::TokenStream;
use syn::{ parse_macro_input, ItemFn, Ident };
use quote::quote;
use lazy_static::lazy_static;
use proc_macro_tokens::Register;
use proc_macro_attr_tokens::{ Options, SubCommand, Command };
use std::collections::HashMap;
use std::sync::RwLock;
use proc_macro2::{ Span as Span2 };
use crate::utils::{get_inputs_runtime_asserts, PATH_PARSER, FN_CALL_EXTRA_TOKEN, generate_inputs};

type OptsStore = HashMap<String, (Vec<String>, Vec<String>)>;

lazy_static! {
    // fn_name -> (long_option_name, short_option_name or empty)
    static ref OPTS_STORE: RwLock<OptsStore> = RwLock::new(HashMap::new());
    // fn_name -> cmd_name
    static ref CMD_REGISTER: RwLock<Option<(String, String)>> = RwLock::new(None);
    // fn_name -> sub_cmd_name
    static ref SUBS_REGISTER: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
    // sub_cmd_name -> fn_name
    static ref SUBS_REGISTER_REV: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
    static ref SUB_FNS_REGISTERED: RwLock<Vec<String>> = RwLock::new(vec![]);
    // have called `register` or not
    static ref REGISTERED: RwLock<bool> = RwLock::new(false);
    // is there any errors?
    static ref IS_ERROR: RwLock<bool> = RwLock::new(false);
}

#[proc_macro_attribute]
pub fn option(opt_stream: TokenStream, func_stream: TokenStream) -> TokenStream {
    let options: Options = parse_macro_input!(opt_stream as Options);
    let func = parse_macro_input!(func_stream as ItemFn);
    let func_name = func.sig.ident.to_string();
    let short = if let Some(short) = &options.short { short.to_string() } else { String::new() };
    let long = options.long.to_string().replace('-', "_");
    // define hidden fn which is using for get instance of the specified `Options`
    let fn_get_opt = decorate_raw_idents(vec![func_name.clone(), long.clone()]);
    let fn_out_ty = import_raw_type(vec![TOKEN_OPTIONS]);

    // check arguments of option
    let mut errors = options.opt_args.try_get_errors().unwrap_or(quote! {});

    {
        let opts_store = &mut OPTS_STORE.write().unwrap();
        let is_error = &mut IS_ERROR.write().unwrap();

        if let Some(opts) = opts_store.get_mut(&func_name) {
            if opts.0.contains(&long) || (!short.is_empty() && opts.1.contains(&short)) {
                // redefined option (short or long) of the option
                let tmp = compile_error(Span2::call_site(), OPTION_IS_NON_DUPLICATED);
                errors = quote! { #errors #tmp };
            } else {
                opts.0.push(long);
                opts.1.push(short);
            }
        } else {
            opts_store.insert(func_name, (vec![long], vec![short]));
        }

        **is_error = **is_error || !errors.is_empty();
    }

    let tmp = if !errors.is_empty() { quote! {} } else {
        quote! {
            // pattern is: `_command_rust_prefix_` + `${fn_name}` + `${option_long_name}` + '_commander_rust_suffix_'.
            fn #fn_get_opt() -> #fn_out_ty {
                #options
            }
        }
    };

    TokenStream::from(quote! {
        #errors
        #func
        #tmp
    })
}

#[proc_macro_attribute]
pub fn sub_command(opt_stream: TokenStream, func_stream: TokenStream) -> TokenStream {
    let sub_cmd: SubCommand = parse_macro_input!(opt_stream as SubCommand);
    let func = parse_macro_input!(func_stream as ItemFn);
    let func_name = func.sig.ident.to_string();
    let sub_name = sub_cmd.name.to_string();
    // define hidden fn which is using for get instance of the specified `Options`
    let fn_get_sub = decorate_ident(func.sig.ident.clone());
    let fn_out_ty = import_raw_type(vec![TOKEN_SUB_COMMAND]);
    let traits_needed = import_raw_trait(TRAIT_PUSH_OPTIONS);
    let runtime_asserts = get_inputs_runtime_asserts(&func.sig.inputs, &sub_cmd.cmd_args);
    let mut errors = sub_cmd.cmd_args.try_get_errors().unwrap_or(quote! {});
    let mut fns_get_opts = vec![];
    // fn called by dispatcher
    let fn_call = decorate_raw_idents(vec![FN_CALL_EXTRA_TOKEN, func_name.as_str()]);
    let ty_app = import_raw_type(vec![TOKEN_APPLICATION]);
    let var_app = Ident::new("app", Span2::call_site());
    let func_inputs = generate_inputs(&func.sig.inputs, &sub_cmd.cmd_args, &var_app);
    let func_ident = &func.sig.ident;

    {
        let opts_store = &mut OPTS_STORE.read().unwrap();
        let subs_register = &mut SUBS_REGISTER.write().unwrap();
        let subs_register_rev = &mut SUBS_REGISTER_REV.write().unwrap();
        let is_error = &mut IS_ERROR.write().unwrap();

        if subs_register_rev.contains_key(&sub_name) {
            // redefined sub_command
            let tmp = compile_error(Span2::call_site(), SUB_CMD_IS_NON_DUPLICATED);
            errors = quote! { #errors #tmp };
        } else if let Some((opts, _)) = opts_store.get(&func_name) {
            for opt in opts.iter() {
                fns_get_opts.push(decorate_raw_idents(vec![func_name.clone(), opt.clone()]));
            }
        }

        subs_register.insert(func_name.clone(), sub_name.clone());
        subs_register_rev.insert(sub_name, func_name);
        **is_error = **is_error || !errors.is_empty();
    };

    let tmp = if !errors.is_empty() { quote! {} } else {
        quote! {
            // pattern is: `${TOKEN_PREFIX}` + `${fn_name}` + '${TOKEN_SUFFIX}'.
            fn #fn_get_sub() -> #fn_out_ty {
                #runtime_asserts
                #traits_needed;
                let mut tmp = #sub_cmd;
                #(tmp.push_option(#fns_get_opts());)*
                tmp
            }

            // pattern is: `${TOKEN_PREFIX}` + `${FN_CALL_EXTRA_TOKEN}` + `${fn_name}` + '${TOKEN_SUFFIX}'.
            fn #fn_call(#var_app: &#ty_app) {
                #func_ident(#func_inputs);
            }
        }
    };

    TokenStream::from(quote! {
        #errors
        #func
        #tmp
    })
}

#[proc_macro_attribute]
pub fn command(opt_stream: TokenStream, func_stream: TokenStream) -> TokenStream {
    let cmd: Command = parse_macro_input!(opt_stream as Command);
    let func = parse_macro_input!(func_stream as ItemFn);
    let func_name = func.sig.ident.to_string();
    let cmd_name = cmd.name.to_string();
    // define hidden fn which is using for get instance of the specified `Options`
    let fn_get_cmd = decorate_ident(func.sig.ident.clone());
    let fn_out_ty = import_raw_type(vec![TOKEN_COMMAND]);
    let traits_needed = import_raw_trait(TRAIT_PUSH_OPTIONS);
    let mut errors = cmd.cmd_args.try_get_errors().unwrap_or(quote! {});
    let mut fns_get_opts = vec![];
    // fn called by dispatcher
    let fn_call = decorate_raw_idents(vec![FN_CALL_EXTRA_TOKEN, func_name.as_str()]);
    let ty_app = import_raw_type(vec![TOKEN_APPLICATION]);
    let var_app = Ident::new("app", Span2::call_site());
    let func_inputs = generate_inputs(&func.sig.inputs, &cmd.cmd_args, &var_app);
    let func_ident = &func.sig.ident;

    {
        let opts_store = &mut OPTS_STORE.read().unwrap();
        let cmd_register = &mut CMD_REGISTER.write().unwrap();
        let is_error = &mut IS_ERROR.write().unwrap();

        if cmd_register.is_some() {
            // redefined command
            let tmp = compile_error(Span2::call_site(), CMD_IS_ONLY);
            errors = quote! { #errors #tmp };
        } else if let Some((opts, _)) = opts_store.get(&func_name) {
            for opt in opts.iter() {
                fns_get_opts.push(decorate_raw_idents(vec![func_name.clone(), opt.clone()]));
            }
        }

        cmd_register.replace((func_name, cmd_name));
        **is_error = **is_error || !errors.is_empty();
    }

    let tmp = if !errors.is_empty() { quote! {} } else {
        quote! {
            // pattern is: `${TOKEN_PREFIX}` + `${fn_name}` + '${TOKEN_SUFFIX}'.
            fn #fn_get_cmd() -> #fn_out_ty {
                #traits_needed;
                let mut tmp = #cmd;
                #(tmp.push_option(#fns_get_opts());)*
                tmp
            }

            // pattern is: `${TOKEN_PREFIX}` + `${FN_CALL_EXTRA_TOKEN}` + `${fn_name}` + '${TOKEN_SUFFIX}'.
            fn #fn_call(#var_app: &#ty_app) {
                 #func_ident(#func_inputs);
            }
        }
    };

    TokenStream::from(quote! {
        #errors
        #func
        #tmp
    })
}

#[proc_macro]
pub fn register(stream: TokenStream) -> TokenStream {
    let Register { cmd, sub_fns_list } = parse_macro_input!(stream as Register);
    // only way to get the only `Command` instance
    let fn_hidden = decorate_raw_ident(GET_ONLY_COMMAND);
    let hidden_output = import_raw_type(vec![TOKEN_COMMAND]);
    let traits_needed = import_raw_trait(TRAIT_PUSH_SUB_COMMAND);
    let fn_get_cmd = decorate_ident(cmd.clone());
    let mut fn_subs = vec![];
    let mut errors = quote! {};
    let mut subs_belong = String::new();

    {
        let cmd_register = &mut CMD_REGISTER.read().unwrap();
        let subs_register = &mut SUBS_REGISTER.read().unwrap();
        let sub_fns_registered = &mut SUB_FNS_REGISTERED.write().unwrap();
        let registered = &mut REGISTERED.write().unwrap();
        let is_error = &mut IS_ERROR.write().unwrap();

        if **registered {
            let tmp = compile_error(cmd.span(), DONT_CALL_REGISTER_MULTIPLE_TIMES);
            errors = quote! { #errors #tmp };
        }

        // clippy's bug?
        #[allow(clippy::cmp_owned)]
        if let Some((sub_fn_name, sub_name)) = cmd_register.as_ref() {
            if cmd.to_string() != sub_fn_name.to_string() {
                let tmp = compile_error(cmd.span(), REGISTER_UNKNOWN_CMD);
                errors = quote! { #errors #tmp };
            }

            subs_belong = sub_name.to_string();
        }

        if !**is_error {
            for sub_fn in sub_fns_list.inner.iter() {
                if !subs_register.contains_key(&sub_fn.to_string()) {
                    let tmp = compile_error(sub_fn.span(), REGISTER_UNKNOWN_SUB_CMD);
                    errors = quote! { #errors #tmp };
                    break;
                }

                sub_fns_registered.push(sub_fn.to_string());
                fn_subs.push(decorate_ident(sub_fn.clone()));
            }
        }

        **is_error = **is_error || !errors.is_empty();
        **registered = true;
    }

    let tmp = if errors.is_empty() {
        quote! {
            fn #fn_hidden() -> #hidden_output {
                 #traits_needed;
                 let mut cmd = #fn_get_cmd();
                 #({
                     let mut sub = #fn_subs();
                     sub.belong = String::from(#subs_belong);
                     cmd.push_sub_command(sub);
                  })*
                 cmd
            }
        }
    } else { quote! {} };

    TokenStream::from(quote! {
        #errors
        #tmp
    })
}

#[proc_macro]
pub fn run(stream: TokenStream) -> TokenStream {
    let fn_get_cmd = decorate_raw_ident(GET_ONLY_COMMAND);
    let seg_wrapper = import_raw_type(vec![PATH_PARSER, TOKEN_SEGMENT_WRAPPER]);
    let segment = import_raw_type(vec![PATH_PARSER, TOKEN_SEGMENT]);
    let traits = vec![
        import_raw_trait(TRAIT_FROM_ARG),
        import_raw_trait(TRAIT_FROM_ARGS),
        import_raw_trait(TRAIT_FROM_APP),
    ];
    let ty_app = import_raw_type(vec![TOKEN_APPLICATION]);
    let mut error = if stream.is_empty() { quote! {} } else {
        let tmp = compile_error(Span2::call_site(), DONT_NEED_PARAMETERS);
        quote! { #tmp }
    };
    let mut fn_cmd = Ident::new("__empty__", Span2::call_site());
    let mut conditions = vec![];
    let mut executors = vec![];
    let any_error;

    {
        let registered = &REGISTERED.read().unwrap();
        let sub_fns_registered = &SUB_FNS_REGISTERED.read().unwrap();
        let subs_register = &SUBS_REGISTER.read().unwrap();
        let cmd_register = &CMD_REGISTER.read().unwrap();
        let is_error = &IS_ERROR.read().unwrap();

        if !**registered {
            let tmp = compile_error(Span2::call_site(), HAVENT_CALLED_REGISTER_FNS);
            error = quote! { #error #tmp };
        }

        // if `registered`, `cmd_register` will never be `None`
        if let Some((cmd_fn_name, _)) = cmd_register.as_ref() {
            fn_cmd = decorate_raw_idents(vec![FN_CALL_EXTRA_TOKEN, cmd_fn_name]);
        }

        for sub_fn_registered in sub_fns_registered.iter() {
            if let Some(sub_name) = subs_register.get(sub_fn_registered) {
                conditions.push(sub_name.to_string());
                executors.push(decorate_raw_idents(vec![FN_CALL_EXTRA_TOKEN, sub_fn_registered]));
            }
        }

        any_error = **is_error || !error.is_empty();
    }

    let tmp = if any_error { quote! {} } else {
        let match_expr = {
            let mut expr = quote! {};

            for i in 0..conditions.len() {
                let condition = &conditions[i];
                let executor = &executors[i];

                expr = quote! {
                    #expr
                    #condition => #executor(&app),
                };
            }

            expr
        };

        quote! {
            {
                #(#traits;)*
                let command = #fn_get_cmd();
                let parser_result = #seg_wrapper::parse_cli(&command);
                let app = #ty_app::from_parser_result(&parser_result, &command).unwrap();

                if let Ok(((cmd, sub_cmd), local_opts, global_opts)) = parser_result {
                    if cmd.is_none() && sub_cmd.is_none() {
                        #fn_cmd(&app);
                    } else {
                        if let Some(seg) = &cmd {
                            if let #segment::Command(_, args) = seg {
                                #fn_cmd(&app);
                            }
                        }

                        if let Some(seg) = &sub_cmd {
                            if let #segment::Command(Some(sub_name), args) = seg {
                                match sub_name.as_str() {
                                    #match_expr
                                    _ => {},
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    TokenStream::from(quote! {
        #error
        #tmp
    })
}
