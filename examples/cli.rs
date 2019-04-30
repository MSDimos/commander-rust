#![feature(proc_macro_hygiene)]

use commander_rust::{ option, command, Cli, entry, run, direct };
use core::borrow::BorrowMut;

#[option(-c, --cn, "Chinese")]
#[option(-e, --en, "English")]
#[option(-j, --jp, "Japanese")]
#[option(-r, --ru, "Russian")]
#[option(-f, --fr, "French")]
#[option(-n, --name <name>, "Who I am?")]
#[command(hello, "Say hello")]
fn hello(cli: Cli) {
    let who =  cli.get_or("name", String::from("Double Dimos"));

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

    println!("I am {} ❤", who);
}


#[command(wish <who>, "best wishes")]
fn wish(who: String, cli: Cli) {
    if !cli.has("peace") {
        println!("Best wishes to you, from {}", who);
    }
}

#[direct(<a> <b> [c] [d])]
fn direct(a: String, b: String, cli: Cli) {
    let peace = cli.has("peace");

    if peace {
        println!("It's peace!");
    }

    println!("hello! {} {}", a, b);
}

#[option(-p, --peace, "I love peace")]
#[entry]
fn main() {
    run!();
}