use colored::Colorize;
use std::process::exit;

// these all are runtime error
pub const MISMATCHED_ARGS: &str = "Mismatched arguments.";
pub const UNKNOWN_SUB: &str = "Unknown sub-command:";
pub const UNKNOWN_OPT: &str = "Unknown option:";
pub const INTERNAL_ERROR: &str = "Internal error, give us feedback on Github pls";


// this will panic!
pub fn raise_error(msg: String) -> bool {
    if !cfg!(feature = "test") {
        let prefix = "CLI runtime error: ".bold().red();
        // output to the standard error pipe
        eprintln!("\n{}{}\n", prefix, msg);
        exit(1);
    }

    false
}