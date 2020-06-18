# Glance
See `example/glance.rs` for more details.
```rust
#[default_options]
#[option(--https, "use https instead of http")]
#[sub_command(connect <address>, "connect to the address")]
fn connect(address: Option<Address>, opts: Opts, global_opts: GlobalOpts) {
    /* .. */
}

#[default_options]
#[sub_command(disconnect <address>, "disconnect the connection")]
fn disconnect(address: Option<Address>) {
    /* .. */
}

#[default_options]
#[option(--proxy <proxy_address>, "use proxy to connect")]
#[command(net, "network tool")]
fn net() { /* .. */}

fn main() {
    execute!(net, [connect, disconnect]);
}
```

# Note
The current `master` branch will no longer be supported. 
And the branch `pre-alpha` will be the `master` branch once it's stable.

# Usage
The current `commander_rust` in `crate.io` is no longer supported.
Before it becomes stable, `github` will be used as distribution source. Add it to your dependencies.
```toml
[dependencies.commander_rust]
git = "https://github.com/MSDimos/commander-rust/"
branch = "pre-alpha"

# or
commander_rust = { git = "https://github.com/MSDimos/commander-rust/", branch = "pre-alpha" }
```

# Viewpoint

As I think, developers should devote more time to the realization of functions instead of leaning how to use `command line interface (CLI)`.
 So a crate of `CLI` should be easy to use.
Specifically, it should have the following advantages:

- Firstly, it should have less `API`s.
- Secondly, it should be intuitive enough to use. What u see is what u get.
- Thirdly, it should make full use of the advantages of programming language.

Inspired by [Rocket](https://rocket.rs/) and [commander.js](https://github.com/tj/commander.js/), the crate is born.

# Design concept

A `CLI` program consists of `Command`，`SubCommand`，`Options` and `Argument`

> `Options` is exactly `Option`, but `Rust` has used `Option` already, so I use it as replacement.

The relationships between them are:

1. One `CLI` has **ONLY ONE** `Command`.
2. One `Command` has **ZERO or MORE** `SubCommand`s.
3. `Command` and `SubCommand` have **ZERO or MORE** `Options`.
4. `SubCommand` and `Options` can accept **ZERO or MORE** `Argument`.

For instance, see examples below:

```shell
command <require_argument> <optional_argument> --option
command sub_command <require_argument> <optional_argument> --option
```

# attribute macros
## `#[option]`
Defining options using `#[option]`. Syntax shows below:
```rust
#[option([-s], --long-name <arg1> [arg2], "description about this option")]
```

Note, options of `command` are global options, it means `sub-command` can access them.

```rust
#[option(--global-option)]
#[command(test)]
fn test() {
    // input
    // paht/of/example test --global-option
    assert!(!opts.contains_key("local-option"));
    assert!(opts.contains_key("global-option"));
}

#[option(--local-option)]
#[sub_command(test_sub)]
fn test_sub(opts: Opts) {
    // input:
    // path/of/example test test_sub --global-option --local-option
    assert!(opts.contains_key("global-option"));
    assert!(opts.contains_key("local-option"))
}
```

Options without arguments are also called `flag` or `switch` (Ha, not `Nintendo Switch`).

### restriction of `#[option]`
All options should be defined above `command` or `sub_command`.  
All options defined below `command` or `sub_command` will be ignored. See example below:
```rust
// valid
#[option(--display, "display something")]
#[option(-v, --version, "display version")]
#[sub_command(cmd_name, "this is a sub_command")]
fn sub_cmd_fn() {} 


// these below are all invalid

#[sub_command(cmd_name, "this is a sub_command")]
#[option(--display, "display something")]
#[option(-v, --version, "display version")]
fn sub_cmd_fn1() {} 

#[option(--display, "display something")]
#[sub_command(cmd_name, "this is a sub_command")]
#[option(-v, --version, "display version")]
fn sub_cmd_fn2() {} 
```

## `#[default_options]`

```rust
#[default_options]
#[sub_command(test)]
fn test() {}

// is equal to
#[option(-v, --version, "print version information")]
#[option(-h, --help, "print help information")]
#[sub_command(test)]
fn test() {}
```

### restriction of `#[default_options]`

Once you use `#[default_options]`,
other options of this `sub_command` or `command` can't use short names `-v` `-h` or long names `--version` `--help`.
See example below:

```rust
#[default_options]
// Error, `-v` is reserved keyword which is used by `#[default_options]`
#[option(-v, --verbose, "display verbose information")]
// Error, `--help` is reserved keyword which is used by `#[default_options]`
#[option(--help, "need help")]
#[sub_command(test)]
fn test() {}
```

## `#[command]` and `#[sub_command]`

They have the similar syntax which are shown below:
```rust
// sub_command
#[sub_command(sub_cmd_name <arg1> [args], "this is a sub-command")]

// command without app version
// in this case, environment variable `CARGO_PKG_VERSION` (i.e., std::env!("CARGO_PKG_VERSION")) will be used as app version
#[command(cmd_name <arg1> [args], "this is a sub-command")]
// command with app version
#[command("0.0.1-pre-alpha", cmd_name <arg1> [args], "this is a sub-command")]
```

### restriction of `#[command]` or `#[sub_command]`
`#[command]` is only, but `#[sub_command]` s are not.

# procedural macros

## `execute!()`

Run the cli app. If you don't call it, the cli app will not run.

Syntax is shown as example below:

```rust
// Note, these `*_fn_name*` are names of function instead of names of `sub_command` or `command`
execute!(command_fn_name, [sub_command_fn_name1, sub_command_fn_name2, ...])
// if no sub_command needed, provide a `[]`
execute!(command_fn_name, [])
```

> Note: Because of restrictions of `Rust`, if you want to used procedural macro, you should add attribute `#![feature(proc_macro_hygiene)]`. See this [issue](https://github.com/rust-lang/rust/issues/54727)for more details.

### restriction of `execute!()`

Because of the internal mechanism, all functions which are used in `execute!()` should be at the same level of modules. It means the example below will raise error:

```rust
mod child_mod {
    #[sub_command(test_sub, "test1")]
	pub fn test1() {}
}

#[command(test_cmd, "test2")]
fn test2() {}

use child_mod::test1;

fn main() {
     // Error, cannot find function `_commander_rust_prefix_test1_commander_rust_suffix_` in this scope
    execute!(test2, [test1]);
}
```

# Extract arguments

## types of arguments

There are four types of arguments. Listed below:

- required single argument:`<arg>`
- required multiply arguments: `<..args>` or `<...args>`
- optional single argument:`[arg]`
- optional multiply arguments: `[..args]` or `[...args]`

> In fact, `required multiply arguments` is equal to `optional multiply arguments`. 

Note: there are several restrictions:

1. All `optional` arguments should be **after** all `required` arguments.

```rust
// valid
#[option(test <a> <b> [c] [d])]
// invalid
#[option(test <a> [b] <c> [d])]
```

2. There can be only one `multiply` argument, and it can only be used as the **last** parameter.

```rust
// valid
#[otpion(test <a> <..b>)]
// invalid
#[option(test <..a> <b>)]
```

## extract named arguments

See example below.

```rust
#[option(--user-name <name>, "login with username")]
#[option(--passwd <passwd>, "login with passwd")]
// all named arguments (e.g. here, <url>) should be used in function signature
#[command(login <url>, "login")]
fn login_fn(url: String) {}
```

> <name> is named argument of option `--user-name`, `<passwd>` is named argument of option `--passwd`. And `url` is a named argument of command `login`.

> Of course, you can customize the type of named arguments. Any type that implement the trait `FromArg` or `FromArgs` can be used as type of named arguments.
>
> What's different between `FromArg` and `FromArgs`?
>
> - `FromArg` is used for type of named arguments which are `single` arguments (e.g., `<arg>` or `[arg]`).
> - `FromArgs` is used for type of named arguments which are `multiply` arguments (e.g., `<..arg>` or `[..arg]`).
>
> There are several types that implement the trait `FromArg`:
>
> - `String` and `&str`
> - `Option<T: FromArg>`
> - `Result<T: FromArg, T::Error>`
> - `i8` `i16` `i32` `i64` `i128` `u8` `u16` `u32` `u64` `u128`
> - `&Arg`
> - `Path` and `PathBuf`
>
> There are several types that implement the trait `FromArgs`:
>
> - `String`
> - `Vec<T: FromArg> (not T: FromArgs)`
> - `Option<T: FromArgs>`
> - `Result<T: FromArgs, T::Error>`
> - `&Args`

How to implement the two traits above? Let me show u an example.

Now, I define an `command` with an argument.

```rust
#[command(download <pkg>, "download an package")]
fn connect(pkg: Pkg) {
    match down_load_pkg(&pkg.name, &pkg.version) {
        Ok(_) => println!("success"),
        Err(e) => eprintln!("{}", e),
    }
}
```

I don't want to use `String`, but I use a type named `Pkg`.
I want to decode user's input which is formatted like `react=16.13.1`.
Now, let's define the struct `Pkg`.

```rust
struct Version(u8, u8, u8);

struct Pkg {
    name: String,
    version: Version,
}
```

Now, the highlight is coming. Let's implement the trait `FromArg`, then we can use it.

```rust
impl<'a> FromArg<'a> for Pkg {
    type Error = ();

    // see document for more details about `Arg` and `Args`
    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        let splits: Vec<&str> = arg.split('=').collect();

        if splits.len() != 2 {
            Err(())
        } else {
            let name = splits[0];
            let vers: Vec<&str> = splits[1].split('.').collect();

            if vers.len() != 3 {
                Err(())
            } else {
                let mut vs = [0, 0, 0];

                for (idx, ver) in vers.into_iter().enumerate() {
                    if let Ok(v) = ver.parse::<u8>() {
                        vs[idx] = v;
                    } else {
                        return Err(());
                    }
                }

                Ok(Pkg {
                    name: name.to_string(),
                    version: Version(vs[0], vs[1], vs[2]),
                })
            }
        }
    }
}
```

Ha, it's done. Now you can use the cli app like:

```shell
$ /path/of/download react=16.13.1
```

But there are some bugs here.
Look the `line 8` `line 14` `line 22` in the code above.
It returned the `Err`. It means sometimes it will crash.
Try to input like this:

```shell
// Error, parse failed, can't parse input `react=16` as type `Pkg`
$ /path/of/download react
```

How to catch errors and handle them yourself ? 
It's easy, do u remember that there are several types which implement the trait `FromArg`? 
`Option<T: FromArg>` and `Result<T: FromArg, T::Error>` are two of them. 
So, change the signature of function:
```rust
#[command(download <pkg>, "download an package")]
// Or pkg: Result<Pkg, ()>, both are okay
fn connect(pkg: Option<Pkg>) {
	if let Some(pkg) = pkg {
        match down_load_pkg(&pkg.name, &pkg.version) {
            Ok(_) => println!("success"),
            Err(e) => eprintln!("{}", e),
        }
    } else {
        // if you want to do something, do it.
        eprintln!("can't parse package.");
    }
}
```

Now, If you input:

```shell
// customize error, can't parse package.
$ /path/of/download react
```

If you want to download multiply packages like:

```shell
$ /path/of/download react=16.13.1 react-redux=7.2.0
```

Change signature of function `download` like:

```rust
#[command(download <pkg>, "download an package")]
// Or pkgs: Vec<Result<Pkg, ()>>, both are okay
fn connect(pkgs: Vec<Option<Pkg>>) {
	if pkgs.is_empty() {
        eprintln!("no packages offered.");
    } else {
        for pkg in pkgs.into_iter() {
            if let Some(pkg) = pkg {
                match down_load_pkg(&pkg.name, &pkg.version) {
                    Ok(_) => printlnr!("success"),
                    Err(e) => eprintln!("{}", e),
                }
            } else {
                // if you want to do something, do it.
                eprintln!("can't parse package.");
            }
        }
    }
}
```

# Extract options

## `Opts` and `GlobalOpts`

I offer u two types to get options. One is `Opts`,
the other one is `GlobalOpts`. 
By names, you should be able to know the difference between them.

```rust
#[option(-f, --force, "force to install even if this package has already installed")]
#[option(-g, --global, "install as a global package")]
#[sub_command(install <pkg>, "install a package")]
fn install_fn(pkg: Result<Pkg, ()>, opts: Opts, global_opts: GlobalOpts) {
   	if opts.contains_key("force") {
        // do something here
    }
    
    if global_opts.contains_key("verbose") {
        // do something here
    }
}
```

## extract arguments of options

Like arguments of `command`, arguments of `option`s have implemented the trait `FromArg` or `FromArgs`. See example below:

```rust
#[option(--fruit <fruit>)]
#[commmand(eat)]
fn eat(opts: Opts) {
    // try to get option
    if let Some(Mixed::Single(fruit)) = opts.get("fruit") {
        // "apple" is default value
        let fruit = String::from_arg(fruit).unwrap_or("apple".to_string());
        
        println!("I eat a(n) {}", fruit);
    }
}
```

> Code above used type `Mixed`. Why it? See example below:
>
> ```rust
> #[option(--test <a> <b> <..c>)]
> ```
>
> As you can see, `<a>` and `<b>` are both `sigle` arguments. But `<..c>` is multiply argument. 
>
> If you want to get named arguments of `--test`  by using api `get`, it will return value of type `Result<Mixed, ()>`. 
>
> In this case, for `single` arguments, `Mixed` is `Mixed::Signle` which only contains one input value. For `multiply` arguments, `Mixed` is `Mixed::Multiply` which contains more input value. 
>
> You can see document or source code for more details.

## advanced usage of options

Repeat: All types of named arguments should implement the trait `FromArg`(for `single` argument) or `FromArgs`(for `multiply` arguments).

If you offer non-named arguments, the types of them should implement the trait `FromApp`.

> See document for more details about struct `App`.

`Opts` and `GlobalOpts` have already implemented the trait `FromApp`. There are several types that implement the trait `FromApp`.

- `Application` and `&Application`(alias `App` and `&App`)
- `&Command`
- `Result<T: FromApp, T::Error>`
- `Option<T: FromApp>`

> See document for more details about struct `Command`. It contains all information about you cli app.
>
> If you want to show help or version information, there are three `api`s of `Command` you can use:
>
> 1. `cmd.println()` -- print help information of command
> 2. `cmd.println_sub("sub_name")` -- print help information of the specified sub-command
> 3. `cmd.println_version()` -- print version information
>
> ```rust
> #[command(test)]
> fn npms_fn(cmd: &Command) {
>     cmd.println();
> }
> ```

How to implement the trait `FromApp`. See example below:

```rust
enum DangerousThing {
    Cephalosporin,
    Wine,
    None,
}

// by implementing the trait `FromApp`, you can do many multiple custom options types
// e.g. here, mutually exclusive options
struct MutexThing(DangerousThing);

impl<'a> FromApp<'a> for MutexThing {
    type Error = String;

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        // You can use `<T as FromApp>::from_app(app)` to convert app to `T`
        if let Ok(opts) = GlobalOpts::from_app(app) {
            if opts.contains_key("cephalosporin") && opts.contains_key("drink-wine") {
                Err(String::from("DANGER!!! DO NOT DO IT! DO NOT take cephalosporin while drinking wine!"))
            } else if opts.contains_key("cephalosporin") {
                Ok(MutexThing(DangerousThing::Cephalosporin))
            } else {
                Ok(MutexThing(DangerousThing::Wine))
            }
        } else {
            Ok(MutexThing(DangerousThing::None))
        }
    }
}

// WARN: DO NOT take cephalosporin while drinking wine! It's fatal behavior!!!!!!!!
#[option(--cephalosporin, "take cephalosporin")]
#[option(--drink-wine, "drink wine")]
#[default_options]
#[command("0.0.1-fruits-eater", eat <food>, "eat food")]
// Here, do u see it?
// mutex_thing is not named argument, so it should implement the trait `FromApp`.
fn eat_fn(food: String, mutex_thing: Result<MutexThing, String>) {
    match food.as_str() {
        "apple" | "banana" | "pear" | "watermelon" | "orange" => println!("I eat a(n) {}, it's delicious!", food),
        _ => println!("I dislike {}", food),
    }

    match mutex_thing {
        Ok(MutexThing(DangerousThing::Cephalosporin)) => println!("DO NOT drink wine recently!"),
        Ok(MutexThing(DangerousThing::Wine)) => println!("DO NOT take cephalosporin recently!"),
        Ok(MutexThing(DangerousThing::None)) => println!("I want to eat more!"),
        Err(note) => println!("{}", note),
    }
}

```



# Conclusion

1. There are three traits you may will use:

|    Name    | description                                                  |
| :--------: | ------------------------------------------------------------ |
| `FromArg`  | single named arguments should implement it, e.g., `<arg>` or `[arg]` |
| `FromArgs` | multiply named arguments should implement it, e.g., `<..args>` or `[..args]` |
| `FromApp`  | non-named arguments of function signature(if it exists) should implement it. |

2. You can use `Opts` and `GlobalOpts` to get options.

2. You can use `&Command` to print help and version information by yourself.
3. Run cli app by calling macro `execute!()`.

# Examples

You can check examples in folder `examples` of this crate for full usage of `commander_rust`.

# Contribution

Because of something that happened to me, I stopped maintaining the previous version of this project for a long time. 

After all that, I have time to maintain the project. 
I am sorry for those people who opened issues, because of refactoring of the project, I can't and needn't to respond them any more.
Now, starting from scratch, any useful contribution is welcome.

If you find bug and fix it, please create an `Merge Request`.

If you have a good idea and implement it, please create an `Merge Request`.

If you have any questions, please open an `issue`.

# TODO list

- [ ] i18n support?

- [ ] sub-sub*n-sub-commands support?

- [ ] cross modules support?

