# why this crate?
As I think, developers should devote more time to the realization of functions instead of leaning how to use `command line interface (CLI)`. So a crate of `CLI` should be easy to use.
Specifically, it should have the following advantages:

- Firstly, it should not have more than 10 `API`s.
- Secondly, it should be intuitive enough to use. What u see is what u get.
- Thirdly, it should make full use of the advantages of programming language.



# Design concept

A `CLI` program consists of `Command`，`SubCommand`，`Options`，`Argument`

> `Options` is exactly `Option`, but `Rust` has used it already, so using it as a replacement.

The relationships between them are:

1. One `CLI` consists of **ONLY ONE** `Command`.
2. One `Command` consists of **ZERO or MORE** `SubCommand`s.
3. `Command` and `SubCommand` both consist of **ZERO or MORE** `Options`.
4. `SubCommand` and `Options` can accept **ZERO or MORE** `Argument`.

For instance, see examples below:

```shell
command <require_argument> <optional_argument> --option
command sub_command <require_argument> <optional_argument> --option
```

# attribute macros
## option
Define option using `#[option]`. Syntax shows below:
```rust
#[option([-s], --long-name <arg1> [arg2], "description about this option")]
```

Options of a sub_command are local options of the specified sub_command. In addition, options of command are global options.

Options without arguments are also called `switch` (Ha, not `Nintendo Switch`).

### restriction
All options should be defined above command and sub_command. See example below:
```rust
// valid
#[option(--display, "display something")]
#[option(-v, --version, "display version")]
#[sub_command(cmd_name, "this is a sub_command")]
fn sub_cmd_fn() {} 


// these are all invalid

#[sub_command(cmd_name, "this is a sub_command")]
#[option(--display, "display something")]
#[option(-v, --version, "display version")]
fn sub_cmd_fn1() {} 


#[option(--display, "display something")]
#[sub_command(cmd_name, "this is a sub_command")]
#[option(-v, --version, "display version")]
fn sub_cmd_fn2() {} 
```

## command and sub_command
They have the same syntax which are shown below:
```rust
// sub_command
#[sub_command(sub_cmd_name <arg1> [args], "this is a sub-command")]
//command
#[command(sub_cmd_name <arg1> [args], "this is a sub-command")]
```

Arguments are optional.

### restriction
Definition of `#[command]` is only. `#[sub_command]` doesn't have this restriction.

# procedural macros

## register_subs
Register all sub_commands which are defined with `#[sub_command]`.

## register_cmd
Register the only command which is defined with `#[command]`.

# TODO list

1. i18n support?
2. sub-sub*n-sub-commands support?