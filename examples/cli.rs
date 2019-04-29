#![feature(proc_macro_hygiene)]

use commander_rust::{ option, command, Cli, entry, run };

#[option(-c, --cn, "Chinese")]
#[option(-e, --en, "English")]
#[command(say <word>, "Say hello")]
fn say(word: String, cli: Cli) {
    if cli.has("cn") {
        println!("词语是{}", word);
    } else if cli.has("en") {
        println!("word is {}", word);
    }
}

#[entry]
fn main() {
    run!();
}