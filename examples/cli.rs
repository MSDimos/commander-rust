#![feature(proc_macro_hygiene)]

use commander_rust::{ option, command, Cli, entry, run };

#[option(-c, --cn, "Chinese")]
#[option(-e, --en, "English")]
#[option(-j, --jp, "English")]
#[option(-r, --ru, "Russian")]
#[option(-f, --fr, "French")]
#[option(-n, --name <name>, "Who I am?")]
#[command(hello, "Say hello")]
fn hello(cli: Cli) {
    if cli.has("cn") {
        println!("你好，世界");
    } else if cli.has("en") {
        println!("hello, world!");
    } else if cli.has("jp") {
        println!("こんにちは、世界");
    } else if cli.has("ru") {
        println!("Здравствуй, мир");
    } else if cli.has("fr") {
        println!("Salut, le monde.");
    }

    if cli.has("peace") || !cli.has("peace") {
        println!("Whether the world is peaceful or not, I still love peace.")
    }

    let who =  cli.get_or("name", String::from("Double Dimos"));

    println!("I am {} ❤", who);
}


#[command(wish <who>, "best wishes")]
fn wish(who: String, cli: Cli) {
    if !cli.has("peace") {
        println!("Best wishes to you, from {}", who);
    }
}

#[option(-p, --peace, "I love peace")]
#[entry]
fn main() {
    run!();
}