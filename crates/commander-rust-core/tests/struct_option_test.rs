use commander_rust_core::{Options, Argument, ArgumentType};
use commander_rust_core::traits::GetArgs;

#[test]
fn options_test() {
    let option = Options::from(r#"-a, --all-numbers <a> [b] [...c], "hello world!""#);
    assert_eq!(option.short, Some(String::from("a")));
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![
        Argument {
            name: String::from("a"),
            ty: ArgumentType::RequiredSingle,
        },
        Argument {
            name: String::from("b"),
            ty: ArgumentType::OptionalSingle,
        },
        Argument {
            name: String::from("c"),
            ty: ArgumentType::OptionalMultiple,
        }
    ]);
    assert_eq!(option.desc, Some(String::from("hello world!")));

    let option = Options::from(r#"-a, --all-numbers <a> [b] [...c]"#);
    assert_eq!(option.short, Some(String::from("a")));
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![
        Argument {
            name: String::from("a"),
            ty: ArgumentType::RequiredSingle,
        },
        Argument {
            name: String::from("b"),
            ty: ArgumentType::OptionalSingle,
        },
        Argument {
            name: String::from("c"),
            ty: ArgumentType::OptionalMultiple,
        }
    ]);
    assert_eq!(option.desc, None);

    let option = Options::from(r#"-a, --all-numbers <a>, "hello world!""#);
    assert_eq!(option.short, Some(String::from("a")));
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![
        Argument {
            name: String::from("a"),
            ty: ArgumentType::RequiredSingle,
        },
    ]);
    assert_eq!(option.desc, Some(String::from("hello world!")));

    let option = Options::from(r#"-a, --all-numbers <ab> [cd], "hello world!""#);
    assert_eq!(option.short, Some(String::from("a")));
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![
        Argument {
            name: String::from("ab"),
            ty: ArgumentType::RequiredSingle,
        },
        Argument {
            name: String::from("cd"),
            ty: ArgumentType::OptionalSingle,
        },
    ]);
    assert_eq!(option.desc, Some(String::from("hello world!")));

    let option = Options::from(r#"-a, --all-numbers, "hello world!""#);
    assert_eq!(option.short, Some(String::from("a")));
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![]);
    assert_eq!(option.desc, Some(String::from("hello world!")));

    let option = Options::from(r#"-a, --all-numbers"#);
    assert_eq!(option.short, Some(String::from("a")));
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![]);
    assert_eq!(option.desc, None);

    let option = Options::from(r#"--all-numbers <a> [b] [...c], "hello world!""#);
    assert_eq!(option.short, None);
    assert_eq!(option.long, String::from("all-numbers"));
    assert_eq!(option.get_args(), &vec![
        Argument {
            name: String::from("a"),
            ty: ArgumentType::RequiredSingle,
        },
        Argument {
            name: String::from("b"),
            ty: ArgumentType::OptionalSingle,
        },
        Argument {
            name: String::from("c"),
            ty: ArgumentType::OptionalMultiple,
        }
    ]);
    assert_eq!(option.desc, Some(String::from("hello world!")));
}