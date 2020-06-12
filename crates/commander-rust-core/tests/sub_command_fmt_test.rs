use commander_rust_core::{Options, Argument, SubCommand};
use commander_rust_core::traits::{PushOptions, PushArgument};

#[test]
fn sub_command_fmt_test() {
    let sub_cmd = SubCommand::from(r#"main_command -> sub_command"#);
    assert_eq!(
        format!("\nUSAGE:\n    main_command sub_command\n\n\n"),
        format!("{}", sub_cmd),
    );

    let mut sub_cmd = SubCommand::new("main-command".to_string(), "sub-command".to_string(), Some("hello babies".to_string()));

    sub_cmd.push_argument(Argument::from("<Makka_Pakka>"));
    sub_cmd.push_argument(Argument::from("[Upsy_Daisy]"));
    assert_eq!(
        format!("DESCRIPTION:\n    hello babies\n\nUSAGE:\n    main-command sub-command <Makka_Pakka> [Upsy_Daisy]\n\n\n"),
        format!("{}", sub_cmd),
    );

    let mut sub_cmd = SubCommand::new("main-command".to_string(), "sub-command".to_string(), Some("hello babies".to_string()));

    sub_cmd.push_argument(Argument::from("<Makka_Pakka>"));
    sub_cmd.push_argument(Argument::from("[Upsy_Daisy]"));

    sub_cmd.push_option(Options::from(r#"-t, --test <a> <b> [..c], "This is using for testing""#));
    sub_cmd.push_option(Options::from(r#"--all <a> <b> [..c], "This is using for testing""#));
    sub_cmd.push_option(Options::from(r#"--pass <a> <b> [..c]"#));
    assert_eq!(
        format!(
r#"DESCRIPTION:
    hello babies

USAGE:
    main-command sub-command <Makka_Pakka> [Upsy_Daisy] [--options]

OPTIONS:
    -t, --test <a> <b> [..c]    This is using for testing
        --all <a> <b> [..c]     This is using for testing
        --pass <a> <b> [..c]

"#),
        format!("{}", sub_cmd),
    );
}