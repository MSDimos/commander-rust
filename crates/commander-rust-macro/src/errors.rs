use proc_macro2::{ Span as Span2, TokenStream as TokenStream2 };
use syn::Error;


pub fn compile_error(span: Span2, msg: &str) -> TokenStream2 {
    Error::new(span, msg).to_compile_error()
}

pub mod msg {
    pub const ARGUMENTS_ORDER_ERROR: &str = "order of arguments is invalid, all [optional arguments] should be placed after all <required arguments>.";
    pub const MULTIPLY_ARGUMENT_IS_ONLY_LAST: &str = "only last argument could be multiply argument.";
    pub const ARGUMENT_IS_NON_DUPLICATED: &str = "arguments duplicate, define arguments with same name more than once.";
    pub const OPTION_IS_NON_DUPLICATED: &str = "option duplicate, define options with same name more than once.";
    pub const SUB_CMD_IS_NON_DUPLICATED: &str = "sub-command duplicate, define sub-commands with same name more than once.";
    pub const CMD_IS_ONLY: &str = "can only define #[command] once.";
    pub const REGISTER_UNKNOWN_SUB_CMD: &str = "try to register unknown sub-command, it was not defined as sub-command using #[sub_command].";
    pub const REGISTER_UNKNOWN_CMD: &str = "try to register an unknown command, it was not defined as command using #[command].";
    pub const UNUSED_ARGUMENT: &str = "unused argument.";
    // default options used
    pub const OPTION_VERSION_RESERVED: &str = "`-v` and `--version` are reserved keywords, remove them from your options";
    pub const OPTION_HELP_RESERVED: &str = "`-h` and `--help` are reserved keyword, remove them from your options";
}
