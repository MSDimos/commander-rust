# why this ?

For a long time, developing cli in `Rust` is difficult.
Since Rust is a static language, the compiler needs to know all the details at compile time. 
It conflicts with the dynamics of the CLI.
The community offers a wide range of solutions. Yes, they're excellent, but they're not very simple.

Inspired by [commander.js](https://github.com/tj/commander.js) & [rocket.rs](https://rocket.rs), the crate was born.

# limit

If you want to use this crate, please guarantee that you have follow rules below:
+ using `Rust 2018` (full proc macro support is required, including [proc_macro] & [proc_macro_attribute])
+ using `cargo` (`cargo` will produce some environment variable according to `Cargo.toml`, we need that)
+ be familiar with `Rust` (because it's developed for `Rust` )

As a reference, my versions are:
+ `cargo`: cargo 1.35.0-nightly (95b45eca1 2019-03-06)
+ `rustc`: rustc 1.35.0-nightly (e68bf8ae1 2019-03-11)
+ `Linux kernal`: 4.15.0-47-generic
+ `Ubuntu`: 16.04

# usage

#### install `commander-rust`

Two ways supported: from `Github` or `crates.io`.
The difference between them is that `Github` is latest but unstable and `crates.io` is stable but might not be latest.

##### install from `Github`
(warn, it does not work now! I'm testing to guarantee that crate will work perfectly)
```toml
[dependencies.commander_rust]
git = "https://github.com/MSDimos/commander_rust"
branch = "master"
```

#### install from `crates.io`

```toml
[dependencies]
commander_rust = "^1.0.0" # or other version you want to install
```

#### using it

We offer a simple but complete example, you can learn all through it.
Yes, That's all. Very easy!

```rust

// this is required! Beacuse we used `run!()`, it's a proc_macro
#![feature(proc_macro_hygiene)]

// Only four items you will use!
use commander_rust::{ Cli, command, option, entry, run };


fn _rmdir(dir: String, other_dirs: Option<Vec<String>>, cli: Cli) {
    if cli.get_or("recursive", false) {
        let quite: bool = cli.get("quite").into();
        
        if quite {
            // silently delete all files
            // just like `rm -rf /`
        } else {
            // tell the world I'm going to delete the files
        }
    } else {
        // drink a cup of coffee, relax.
    }
}


// what's option? what's command? 
// See `commander.js` and document of `commander-rust` for more!
// Note, types of parameters are not fixed, any type implemented `From<Raw>` is valid!
// So you can even use `rmdir(dir: i32, other_dirs: Vec<i32>, cli: Cli)` here.
// And `Cli` is not required! So you can miss it.
// See document of `commander-rust` for more details.
#[option(-s, --format <format>, "format output")]
#[option(-r, --recursive, "recursively")]
#[command(rmdir <dir> [otherDirs...], "remove files and directories")]
fn rmdir(dir: String, other_dirs: Option<Vec<String>>, cli: Cli) {
    // You might be confused, 
    // Why call a function instead of doing everything here? Isn't that superfluous ?
    // This is a bug of compiler. 
    // If you do everything here, the compiler can't give good advice When your code goes wrong.
    // You can have a try, using `let three = 1 + "2"` replace code below. See compiler's error message.
    // See more details: `https://github.com/dtolnay/syn/issues/622`
    
     _rmdir(dir, other_dirs, cli);
}

// options here are public, options above `#[command]` are private
#[option(-q, --quite <quite_or_not>, "dont display anything")]
#[entry]
fn main() {
     // Run it now!!!!!
     let app = run!();
     println!("app is {:#?}", app);
}
```

#### try it

try to input `[pkg-name] --help`.

# version & description & cli-name?

`version`, `description`, `cli-name` of application are from `Cargo.toml`.

For instance:
```toml
# part of Cargo.toml
[package]
name = "example-test"
version = "0.1.0"
description = "Using for test"
```

# rules

There are several rules you should follow.
1. All `#[options]` should defined above `#[command]` and `#[entry]`
2. DO NOT define options duplicate!!! As a concession, You can define the same option on different command and entry.
3. Private options are visible to specific sub-commands. Public options are visible to all sub-commands.
4. At least one sub-command!


# warn

The crate is developed using `Ubuntu 16.04`, we can't assume that all platforms will work well.
If you find that it can't work well in other platforms, please tell me.

# contribute

Any useful contribute are welcome. You can send pull request to me.

# License

GPL. Because open source is future.