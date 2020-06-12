use commander_rust_core::{Command, Options, SubCommand};
use commander_rust_core::traits::{PushOptions, PushSubCommand};
use std::ffi::OsString;
use commander_rust_core::parser::{SegmentWrapper, Segment};

#[test]
fn validation_strategy_test() {
    let mut command = Command::from(r#"peogle <user> <passwd>"#);
    let mut sub_cmd = SubCommand::from(r#"peogle -> search <..conditions>"#);

    command.push_option(Options::from(r#"-d, --database <database_name>"#));

    sub_cmd.push_option(Options::from(r#"-a, --age <from> <to>, "age range of people""#));
    sub_cmd.push_option(Options::from(r#"-n, --name <..names>"#));
    sub_cmd.push_option(Options::from(r#"--sex <sex>"#));
    command.push_sub_command(sub_cmd);

    let args_os: Vec<OsString> = vec![
        // "/path/peogle", // ignore it
        "dimos",
        "123456",
        "search",
        "name",
        "age",
        "sex",
        "--name",
        "Jack",
        "Rose",
        "Smith",
        "--age",
        "20",
        "40",
        "--sex",
        "all",
        "--",
        "a",
        "b",
        "c"
    ]
        .iter()
        .map(|s| OsString::from(s)).collect();
    let mut segments = SegmentWrapper(Segment::from_vec(args_os));
    let tmp = segments.parse_test(&command);

    if let Ok(((cmd, sub), local_opts, global_opts)) = tmp {
        assert_eq!(
            Some(Segment::Command(
                None,
                vec![
                    Segment::Raw("dimos".to_string()),
                    Segment::Raw("123456".to_string()),
                ],
            )),
            cmd
        );
        assert_eq!(
            Some(Segment::Command(
                Some("search".to_string()),
                vec![
                    Segment::Raw("name".to_string()),
                    Segment::Raw("age".to_string()),
                    Segment::Raw("sex".to_string()),
                    Segment::Raw("a".to_string()),
                    Segment::Raw("b".to_string()),
                    Segment::Raw("c".to_string()),
                ]
            )),
            sub,
        );

        assert_eq!(
            vec![
                Segment::Long("name".to_string(), vec![
                    Segment::Raw("Jack".to_string()),
                    Segment::Raw("Rose".to_string()),
                    Segment::Raw("Smith".to_string()),
                ]),
                Segment::Long("age".to_string(), vec![
                    Segment::Raw("20".to_string()),
                    Segment::Raw("40".to_string()),
                ]),
                Segment::Long("sex".to_string(), vec![
                    Segment::Raw("all".to_string()),
                ])
            ],
            local_opts,
        );

        assert!(global_opts.is_empty());
    } else {
        assert!(false, "parse arguments failed");
    }
}