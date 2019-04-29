// generating compile error information
// it's easy for debugging
pub fn compile_error_info(span: proc_macro2::Span, msg: &str) -> proc_macro2::TokenStream {
    syn::Error::new(span, msg).to_compile_error()
}

type Msg = &'static str;

pub const ARG_DUPLICATE_DEFINITION: Msg = "Duplicate definitions of arguments.";
pub const ORDER_ERROR: Msg = "The order of parameters is wrong. All [optional parameters] must be defined after the <required parameters>.";
pub const ENTRY_ONLY_MAIN: Msg = "#[entry] can be used for fn main only.";
pub const DON_NOT_MATCH: Msg = "The name of sub-command should be same as the name of it's function.";
pub const OPT_DUPLICATE_DEFINITION: Msg = "Duplicate definitions of options.";
pub const NO_SUB_CMD_NAMES_MAIN: Msg = "Sub-command can't be named as [main].";