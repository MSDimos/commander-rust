#![feature(proc_macro_hygiene)]

mod methods;

use commander_rust::{ Cli, command, option, entry, run };
use md5::{ compute, Digest };

use methods::{ write_file, read_file };
use std::process::exit;
use std::path::Path;

#[option(-d, --display, "display hash")]
#[option(-o, --output <file>, "output into directory")]
#[option(-f, --force, "if output file is not existed, create it")]
#[command(gen <file>, "generate hash of file")]
fn gen(file: String, cli: Cli) {
    let file_path: &Path = file.as_ref();
    // get private option `output`, if it doesn't exist, return ".hash"
    let output = cli.get_or("output", String::from(".hash"));
    // get private option `force`, because `force` doesn't have argument, so using has(idx: &str) instead
    let force = cli.has("force");
    // get public option `length`, if it doesn't exist, return 32
    let len = cli.get_or("length", 32);
    let file_content = {
        if file_path.is_file() {
            read_file(file_path)
        } else {
            file
        }
    };
    let mut content = format!("{:x}", compute(file_content));

    if output.len() == 0 {
        eprintln!("please specify the output file");
        exit(0);
    }

    content.truncate(len);

    // get private option `display`, because `force` doesn't have argument, so using has(idx: &str) instead
    if cli.has("display") {
        println!("hash is {}", content);
    }

    // you can use has(idx: &str) to check if user input `output` or not here.
    if cli.has("output") {
        write_file(output.as_ref(), content, force);
    }
}


#[command(cmp <A> <B>, "compare A with B. A is file path or literal string, B is file path or hash")]
fn cmp(original: String, target: String, cli: Cli) {
    let ori_path: &Path = original.as_ref();
    let mut ori_hash = {
        if ori_path.is_file() {
            format!("{:x}", compute(read_file(ori_path)))
        } else  {
            format!("{:x}", compute(original))
        }
    };
    let tar_path: &Path = target.as_ref();
    let mut tar_hash = {
        if tar_path.is_file() {
            format!("{:x}", compute(read_file(tar_path)))
        } else if tar_path.is_dir() && tar_path.join(".hash").is_file() {
            read_file(tar_path.join(".hash").as_path())
        } else {
            target
        }
    };

    ori_hash.truncate(cli.get_or("length", 32));
    tar_hash.truncate(cli.get_or("length", 32));
    println!("MD5 of A is: {}", ori_hash);
    println!("MD5 of B is: {}", tar_hash);

    if ori_hash == tar_hash {
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