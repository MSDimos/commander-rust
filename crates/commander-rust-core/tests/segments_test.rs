use commander_rust_core::parser::Segment;
use std::ffi::OsString;

#[test]
fn match_test() {
    // `word` test
    assert!(Segment::is_lit_word("hello"));
    assert!(Segment::is_lit_word("i18n"));
    assert!(Segment::is_lit_word("123"));
    assert!(!Segment::is_lit_word("1+2=3"));

    // `short` test
    assert!(Segment::is_short("-a"));
    assert!(Segment::is_short("-abc"));
    assert!(!Segment::is_short("-abc=hhh"));
    assert!(!Segment::is_short("-abc-def"));
    assert!(!Segment::is_short("--abc"));

    // `long` test
    assert!(Segment::is_long("--long"));
    assert!(Segment::is_long("--long-option"));
    assert!(Segment::is_long("--long=value"));
    assert!(Segment::is_long(r#"--long="hello world!""#));
    assert!(!Segment::is_long("--long="));

    // `--` test
    assert!(Segment::is_double_sub("--"));
    assert!(!Segment::is_double_sub("++"));
}

#[test]
fn parse_test() {
    let args_os: Vec<OsString> = vec!["cli", "sub_command"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Raw(String::from("cli")),
            Segment::Raw(String::from("sub_command"))
        ],
        segments,
    );

    let args_os: Vec<OsString> = vec!["cli", "-a", "hello world!"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Raw(String::from("cli")),
            Segment::Short("a".to_string(), vec![]),
            Segment::Raw(String::from("hello world!"))
        ],
        segments,
    );

    let args_os: Vec<OsString> = vec!["cli", "--long-options", "hello world!"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Raw(String::from("cli")),
            Segment::Long(String::from("long-options"), vec![]),
            Segment::Raw(String::from("hello world!"))
        ],
        segments,
    );

    let args_os: Vec<OsString> = vec!["cli", "--long-options=/path/to/output", "hello world!"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Raw(String::from("cli")),
            Segment::Long(String::from("long-options"), vec![Segment::Raw(String::from("/path/to/output"))]),
            Segment::Raw(String::from("hello world!"))
        ],
        segments,
    );

    let args_os: Vec<OsString> = vec!["cli", "--js-expr=let a = 123;", "hello world!"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Raw(String::from("cli")),
            Segment::Long(String::from("js-expr"), vec![Segment::Raw(String::from("let a = 123;"))]),
            Segment::Raw(String::from("hello world!"))
        ],
        segments,
    );

    let args_os: Vec<OsString> = vec!["cli", "--", "-abc", "--long=abc", "*&%asd"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Raw(String::from("cli")),
            Segment::DoubleSub,
            Segment::Raw(String::from("-abc")),
            Segment::Raw(String::from("--long=abc")),
            Segment::Raw(String::from("*&%asd")),
        ],
        segments,
    );

    let args_os: Vec<OsString> = vec!["--long=", "-abc-cd", "--=", "---"].iter().map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    assert_eq!(
        vec![
            Segment::Error(String::from("--long=")),
            Segment::Error(String::from("-abc-cd")),
            Segment::Error(String::from("--=")),
            Segment::Error(String::from("---")),
        ],
        segments,
    );
}

