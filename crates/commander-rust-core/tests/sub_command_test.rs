use commander_rust_core::{SubCommand, Argument};
use commander_rust_core::traits::GetArgs;

#[test]
fn sub_command_test() {
    let sub_command = SubCommand::from(r#"main -> sub <a_b> [c_d] <e_f>, "hello world!""#);

    assert_eq!("main".to_string(), sub_command.belong);
    assert_eq!("sub".to_string(), sub_command.name);
    assert_eq!(
        &vec![
            Argument::from("<a_b>"),
            Argument::from("[c_d]"),
        ],
        sub_command.get_args(),
    );
    assert_eq!(Some("hello world!".to_string()), sub_command.desc);

    let sub_command = SubCommand::from(r#"main -> sub <a_b> [c_d] <e_f>, "hello world!""#);

    assert_eq!("main".to_string(), sub_command.belong);
    assert_eq!("sub".to_string(), sub_command.name);
    assert_eq!(
        &vec![
            Argument::from("<a_b>"),
            Argument::from("[c_d]"),
        ],
        sub_command.get_args(),
    );
    assert_eq!(Some("hello world!".to_string()), sub_command.desc);

    let sub_command = SubCommand::from(r#"main -> sub <a_b> [c_d] <e_f>"#);

    assert_eq!("main".to_string(), sub_command.belong);
    assert_eq!("sub".to_string(), sub_command.name);
    assert_eq!(
        &vec![
            Argument::from("<a_b>"),
            Argument::from("[c_d]"),
        ],
        sub_command.get_args(),
    );
    assert_eq!(None, sub_command.desc);


    let sub_command = SubCommand::from(r#"main -> sub"#);

    assert_eq!("main".to_string(), sub_command.belong);
    assert_eq!("sub".to_string(), sub_command.name);
    assert!(sub_command.get_args().is_empty());
    assert_eq!(None, sub_command.desc);
}