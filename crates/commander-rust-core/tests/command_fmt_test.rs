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
    print!("{}", command);
    assert_eq!(
        "\u{1b}[1;3mDESCRIPTION\u{1b}[0m:\n    e^{ix} = cos(x) + isin(x)\n\n\u{1b}[1;3mUSAGE\u{1b}[0m:\n    duck <..Makka_Pakka> [sub_commands]\n\n\n\u{1b}[1;3mSUB_COMMANDS\u{1b}[0m:\n    sub        hello babies\n    help       Print help information\n    version    Print version information\n\n",
        format!("{}", command),
    );

}