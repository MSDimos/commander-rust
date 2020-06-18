#![feature(proc_macro_hygiene)]

use commander_rust::{ option, sub_command, command, execute, default_options, };
use commander_rust::{Opts, Mixed};
use commander_rust::traits::{ FromArg };

// types of named arguments (e.g. here, [extra]) defined in the `#[command]` should implement the trait `commander_rust::traits::FromArg`
// if named arguments is multiply (e.g., <..extra> or [..extra]), the types of them should implement the trait `commander_rust::traits::FromArgs`
// other arguments of function (e.g., `cmd: &Command` of `fn adder`) should implement the trait `commander_rust::traits::FromApp`
// there are several types which implement the trait `commander_rust::traits::FromApp`
// 1. &Command
// 2. Opts (doesn't contains global options)
// 3. GlobalOpts
// 4. Application (alias `App`) and &Application (alias `&App`)
// note: all named arguments in `#[command]` should be used, or error will be raised
#[command("0.0.1-test", adder [extra], "add two numbers using sub-command")]
fn adder(extra: Option<String>) {
    if let Some(extra_info) = extra {
        println!("{}", extra_info);
    }
}

#[allow(dead_code)]
#[option(-a, --abs, "absolute value")]
#[option(-o, --overflow-protection <strategy>, "overflow protection")]
#[option(--verbose, "display verbose information")]
#[default_options]
#[sub_command(add <a> <b>, "add two numbers")]
// `opts` is used for get options (doesn't include global options)
// if you want to get global options, use `global_opts: GlobalOpts`
fn add(a: i32, b: i32, opts: Opts) {
    if let Some(overflow_strategy) = opts.get("overflow-protection") {
        if let Some(Mixed::Single(s)) = overflow_strategy.get("strategy") {
            let strategy = String::from_arg(s).unwrap_or("i128".to_string());

            let sum = match strategy.as_str() {
                "i128" => (a as i128) + (b as i128),
                "i64" => ((a as i64) + (b as i64)) as i128,
                _ => (a + b) as i128,
            };

            if opts.contains_key("verbose") {
                println!("{} + {} = {}", a, b, sum);
            } else {
                println!("{}", sum);
            }
        }
    } else {
        // non overflow-protection
        if opts.contains_key("verbose") {
            println!("{} + {} = {}", a, b, a + b);
        } else {
            println!("{}", a + b);
        }
    }
}

fn main() {
    execute!(adder, [add]);
}