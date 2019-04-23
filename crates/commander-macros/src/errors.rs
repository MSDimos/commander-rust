use std::process::exit;

pub fn error(msg: &str, target: &str) {
    eprintln!("\n\n{}: {}. Occurred by `{}`.\n", ERR_RED, msg, target);
    exit(-1);
}

pub fn error_nt(msg: &str) {
    eprintln!("\n\n{}: {}\n", ERR_RED, msg);
    exit(-1);
}

type Msg = &'static str;

pub const ERR_RED: Msg = "\x1b[0;31mCompile time error\x1b[0m";
pub const ARG_DUPLICATE_DEFINITION: Msg = "Duplicate definitions of arguments";
pub const ORDER_ERROR: Msg = "The parameter order is incorrect. All `<>` must appear before all `[]`, only the last parameter can be `[...]` or `<...>`";
pub const ENTRY_ONLY_MAIN: Msg = "#[entry] can be used for fn main only";
pub const DON_NOT_MATCH: Msg = "The name of sub-command should be same as the name of it's function";
pub const OPT_DUPLICATE_DEFINITION: Msg = "Duplicate definitions of options";
pub const INVALID_ARGUMENTS: Msg = "Invalid argument, this means we don't know why. Please open issue with your code";
pub const NO_SUB_CMD_NAMES_MAIN: Msg = "Sub-command can't be named as [main]";