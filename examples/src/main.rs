#![feature(proc_macro_hygiene)]

mod methods;

use commander_rust::{ Cli, command, option, entry, run };
use md5::{ compute };

use methods::{ write_file, read_file };
use std::process::exit;
use std::path::Path;


#[option(-d, --display, "display hash")]
#[option(-o, --output <file>, "output into directory")]
#[option(-f, --force, "if output file is not existed, create it")]
#[command(gen <file>, "generate hash of file")]
fn gen(file: String, cli: Cli) {
    let force = cli.has("force");
    let display = cli.get_or("display", false);
    let output = cli.get_or("output", String::from("./"));
    let length = cli.get_or("length", 32);
    let path: &Path = file.as_ref();
    let content;
    let mut md5;

    if !path.exists() {
        eprintln!("{:#?} doesn't exist. Specify a file.", path);
        exit(0);
    }

    if !path.is_file() {
        eprintln!("{:#?} is not a file.", path);
        exit(0);
    }

    content = read_file(path);
    md5 = format!("{:?}", compute(content));
    md5.truncate(length);

    if display {
        println!("MD5 is {}", md5);
    }

    write_file(output.as_ref(), md5, force);

}


#[command(cmp <A_file> <B_hash>, "compare A with B. A is file, B is hash")]
fn cmp(file: String, hash: String, cli: Cli) {
    let length = cli.get_or("length", 32);
    let path: &Path = file.as_ref();
    let content;
    let mut md5;


    if !path.exists() {
        eprintln!("{:#?} doesn't exist. Specify a file.", path);
        exit(0);
    }

    if !path.is_file() {
        eprintln!("{:#?} is not a file.", path);
        exit(0);
    }

    content = read_file(path);
    md5 = format!("{:?}", compute(content));
    md5.truncate(length);

    println!("hash of A is {}", md5);
    println!("B is {}", hash);

    if md5 == hash {
        println!("They are same");
    } else {
        println!("They are different");
    }

}


#[option(-l, --length <length>, "specify length of hash, maximum is 32")]
#[entry]
fn main() {
    run!();
}