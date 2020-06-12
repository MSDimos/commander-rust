use commander_rust_core::{Command, Argument, SubCommand};
use commander_rust_core::traits::{PushSubCommand, PushArgument};

#[test]
fn command_fmt_test() {
    let mut command = Command::from(r#"duck, "e^{ix} = cos(x) + isin(x)""#);
    let sub_cmd = SubCommand::from(r#"duck -> sub <flower> <sun>, "hello babies""#);

    command.push_argument(Argument::from("<...Makka_Pakka>"));
    command.push_argument(Argument::from("[Upsy_Daisy]"));

    command.push_sub_command(sub_cmd);
    command.push_sub_command(SubCommand::from(r#"duck -> help, "Print help information""#));
    command.push_sub_command(SubCommand::from(r#"duck -> version, "Print version information""#));
    assert_eq!(
r#"DESCRIPTION:
    e^{ix} = cos(x) + isin(x)

USAGE:
    duck <..Makka_Pakka> [sub_commands]


SUB_COMMANDS:
    sub        hello babies
    help       Print help information
    version    Print version information

"#,
        format!("{}", command),
    );
}