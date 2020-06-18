use commander_rust_core::{Options, Argument, SubCommand};
use commander_rust_core::traits::{PushOptions, PushArgument};

#[test]
fn sub_command_fmt_test() {
    let sub_cmd = SubCommand::from(r#"main_command -> sub_command"#);
    assert_eq!(
        "\n\u{1b}[1;3mUSAGE\u{1b}[0m:\n    main_command sub_command\n\n\n",
        format!("{}", sub_cmd),
    );

    let mut sub_cmd = SubCommand::new("main-command".to_string(), "sub-command".to_string(), Some("hello babies".to_string()));

    sub_cmd.push_argument(Argument::from("<Makka_Pakka>"));
    sub_cmd.push_argument(Argument::from("[Upsy_Daisy]"));
    assert_eq!(
        "\u{1b}[1;3mDESCRIPTION\u{1b}[0m:\n    hello babies\n\n\u{1b}[1;3mUSAGE\u{1b}[0m:\n    main-command sub-command <Makka_Pakka> [Upsy_Daisy]\n\n\n",
        format!("{}", sub_cmd),
    );

    let mut sub_cmd = SubCommand::new("main-command".to_string(), "sub-command".to_string(), Some("hello babies".to_string()));

    sub_cmd.push_argument(Argument::from("<Makka_Pakka>"));
    sub_cmd.push_argument(Argument::from("[Upsy_Daisy]"));

    sub_cmd.push_option(Options::from(r#"-t, --test <a> <b> [..c], "This is using for testing""#));
    sub_cmd.push_option(Options::from(r#"--all <a> <b> [..c], "This is using for testing""#));
    sub_cmd.push_option(Options::from(r#"--pass <a> <b> [..c]"#));
    assert_eq!(
        "\u{1b}[1;3mDESCRIPTION\u{1b}[0m:\n    hello babies\n\n\u{1b}[1;3mUSAGE\u{1b}[0m:\n    main-command sub-command <Makka_Pakka> [Upsy_Daisy] [--options]\n\n\u{1b}[1;3mOPTIONS\u{1b}[0m:\n    -t, --test <a> <b> [..c]    This is using for testing\n        --all <a> <b> [..c]     This is using for testing\n        --pass <a> <b> [..c]\n\n",
        format!("{}", sub_cmd),
    );
}