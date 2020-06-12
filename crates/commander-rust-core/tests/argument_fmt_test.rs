use commander_rust_core::Argument;

#[test]
fn argument_fmt_test() {
    assert_eq!(format!("{}", Argument::from("<num>")), String::from("<num>"));
    assert_eq!(format!("{}", Argument::from("[num]")), String::from("[num]"));
    assert_eq!(format!("{}", Argument::from("[num]")), String::from("[num]"));
    assert_eq!(format!("{}", Argument::from("<..num>")), String::from("<..num>"));
    assert_eq!(format!("{}", Argument::from("<...num>")), String::from("<..num>"));
    assert_eq!(format!("{}", Argument::from("[..num]")), String::from("[..num]"));
    assert_eq!(format!("{}", Argument::from("[...num]")), String::from("[..num]"));
}