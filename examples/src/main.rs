#![feature(proc_macro_hygiene)]

use commander_rust::{ Cli, command, option, entry, run };

fn _rmdir(dir: i32, other_dirs: Option<Vec<bool>>, cli: Cli) {
    // println!("dir is {}, other_dirs is {:#?}", dir, other_dirs);
    let recursive: Vec<i32> = cli.get_or_else("recursive", || vec![1, 2, 3]);
    // println!("{:#?} {:#?}", dir, other_dirs);
}


#[command(rmdir <dir> [otherDirs...], "remove files and directories")]
fn rmdir(dir: i32, other_dirs: Option<Vec<bool>>, cli: Cli) {
     _rmdir(dir, other_dirs, cli);
}

#[option(-s, --simple [dir], "simplify sth")]
#[option(-r, --recursive, "recursively1")]
#[command(copy <dir> [otherDirs...], "yes")]
fn copy(dir: Vec<i32>, other_dirs: Option<Vec<String>>) {
    println!("dir is {:#?}, other_dirs is {:#?}", dir, other_dirs);
}

#[option(-t, --total [dir], "simplify sth")]
#[entry]
fn main() {
     let app = run!();
    // same as input `[pkg-name] --help`
    // println!("app is {:#?}", app);
}

