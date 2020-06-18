#![feature(int_error_matching)]

use commander_rust_core::converters::{Arg, Args, Application};
use commander_rust_core::traits::{PushOptions, PushSubCommand};
use commander_rust_core::converters::{FromArgs, FromArg, Mixed};
use std::num::ParseIntError;
use std::path::PathBuf;
use commander_rust_core::parser::{Segment, SegmentWrapper};
use std::ffi::OsString;
use commander_rust_core::{Command, Options, SubCommand};
use std::collections::HashMap;

macro_rules! test_ints {
    ($src: expr => $($ty: ty = $val: expr; )*) => {
        $({
            let a: Result<$ty, ParseIntError> = <$ty>::from_arg(&$src);
            assert_eq!(a, Ok($val));

            let oa: Option<$ty> = Option::from_arg(&$src).unwrap();
            assert_eq!(oa, Some($val));
        })*

    };
}

macro_rules! test_ints_vec {
    ($src: expr => $($ty: ty = $val: expr; )*) => {
        $({
            let va: Vec<$ty> = Vec::from_args(&$src).unwrap();
            assert_eq!(va, $val);
        })*
    };
}

#[test]
fn conversion_test() {
    let args = Args(vec![
        "1024".to_string(),
        "2048".to_string(),
        "9086".to_string()
    ]
        .into_iter()
        .map(|n| Arg(n))
        .collect()
    );
    let arg = Arg("123".to_string());

    test_ints! {arg =>
        u8 = 123;
        u16 = 123;
        u32 = 123;
        u64 = 123;
        u128 = 123;
    }
    ;

    test_ints! {arg =>
        i8 = 123;
        i16 = 123;
        i32 = 123;
        i64 = 123;
        i128 = 123;
    }
    ;

    test_ints_vec! {args =>
        u16 = vec![1024, 2048, 9086];
        u32 = vec![1024, 2048, 9086];
        u64 = vec![1024, 2048, 9086];
        u128 = vec![1024, 2048, 9086];
    }
    ;

    test_ints_vec! {args =>
        i16 = vec![1024, 2048, 9086];
        i32 = vec![1024, 2048, 9086];
        i64 = vec![1024, 2048, 9086];
        i128 = vec![1024, 2048, 9086];
    }
    ;

    let err_u8 = u8::from_arg(&Arg("256".to_string()));
    assert!(err_u8.is_err());

    let str = String::from_arg(&Arg("hello world!".to_string()));
    assert_eq!(Ok("hello world!".to_string()), str);

    let path_buf = PathBuf::from_arg(&Arg("+-*".to_string()));
    assert_eq!(Ok(PathBuf::from("+-*")), path_buf);
}


#[test]
fn customize_arg_conversion_test() {
    #[derive(Eq, PartialEq, Debug)]
    struct MyU8 {
        num: u8,
    }
    ;

    impl<'a> FromArg<'a> for MyU8 {
        type Error = ();

        fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
            match u8::from_arg(arg) {
                Ok(num) => Ok(MyU8 { num }),
                Err(_) => Err(()),
            }
        }
    }

    assert_eq!(
        Ok(MyU8 {
            num: 127,
        }),
        MyU8::from_arg(&Arg("127".to_string()))
    );
}

#[test]
fn customize_args_conversion_test() {
    #[derive(Eq, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    impl<'a> FromArgs<'a> for Person {
        type Error = ();

        fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
            if args.len() != 2 {
                Err(())
            } else {
                let name = String::from_arg(&args[0]);
                let age = u8::from_arg(&args[1]);

                if name.is_ok() && age.is_ok() {
                    Ok(Person {
                        name: name.unwrap(),
                        age: age.unwrap(),
                    })
                } else {
                    Err(())
                }
            }
        }
    }

    assert_eq!(
        Ok(Person {
            name: "Jack".to_string(),
            age: 46,
        }),
        Person::from_args(&Args(vec![Arg("Jack".to_string()), Arg("46".to_string())])),
    );
}


#[test]
fn converter_test() {
    let mut command = Command::from(r#"net"#);
    let mut sub_cmd = SubCommand::from(r#"net -> send <..content>"#);

    command.push_option(Options::from(r#"-s, --ssl, "using ssl"#));

    sub_cmd.push_option(Options::from(r#"-m, --method <method>, "method to use""#));
    sub_cmd.push_option(Options::from(r#"-u, --url <url>, "url of target""#));
    sub_cmd.push_option(Options::from(r#"-c, --content-type <content_type>, "content-type of sending message""#));
    sub_cmd.push_option(Options::from(r#"--max-size <size>, "max size(b) of message""#));
    sub_cmd.push_option(Options::from(r#"--headers [..headers], "headers of request""#));
    command.push_sub_command(sub_cmd);

    let args_os: Vec<OsString> = vec![
        // "/path/of/cli", // ignore it
        "send",
        "--method",
        "post",
        "--url",
        "https://www.example.com",
        "--content-type",
        "text/plain",
        "--headers",
        "Accept=*/*",
        "Accept-Language=zh-CN; en-US",
        "User-Agent=Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:77.0) Gecko/20100101 Firefox/77.0",
        "Connection=keep-alive",
        "--max-size",
        "1024",
        "--",
        "hello world!"
    ]
        .iter()
        .map(|s| OsString::from(s)).collect();
    let mut segments = SegmentWrapper(Segment::from_vec(args_os));
    let converter = Application::from_parser_result(&segments.parse_test(&command), &command).unwrap();

    #[derive(Eq, PartialEq, Debug)]
    struct Headers {
        accept: String,
        accept_language: String,
        user_agent: String,
        connection: String,
    }

    impl<'a> FromArgs<'a> for Headers {
        type Error = ();

        fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
            let mut map = HashMap::new();
            let keys = ["Accept", "Accept-Language", "User-Agent", "Connection"];

            for arg in args.iter() {
                if let Ok(s) = String::from_arg(arg) {
                    let mut s: Vec<String> = s.split_terminator('=').into_iter().map(|s| s.to_string()).collect();
                    if s.len() != 2 {
                        return Err(());
                    } else if keys.contains(&s[0].as_str()) {
                        map.insert(s.remove(0), s.remove(0));
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }

            if map.len() != keys.len() {
                Err(())
            } else {
                Ok(Headers {
                    accept: map.remove("Accept").unwrap(),
                    accept_language: map.remove("Accept-Language").unwrap(),
                    user_agent: map.remove("User-Agent").unwrap(),
                    connection: map.remove("Connection").unwrap(),
                })
            }
        }
    }

    assert!(converter.cmd_args.is_empty());

    if let Mixed::Multiply(args) = converter.sub_args.get("content").unwrap() {
        let contents: Vec<String> = Vec::from_args(args).unwrap_or(vec![]);
        assert_eq!(vec!["hello world!".to_string()], contents);
    } else {
        assert!(false);
    }

    assert_eq!(Some("send".to_string()), converter.sub_name);

    if let Some(map) = converter.get_opt("method") {
        if let Some(Mixed::Single(arg)) = map.get("method") {
            let tmp = String::from_arg(arg).unwrap_or(String::new());
            assert_eq!("post", tmp);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    if let Some(map) = converter.get_opt("url") {
        if let Some(Mixed::Single(arg)) = map.get("url") {
            let tmp = String::from_arg(arg).unwrap_or(String::new());
            assert_eq!("https://www.example.com", tmp);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    if let Some(map) = converter.get_opt("content-type") {
        if let Some(Mixed::Single(arg)) = map.get("content_type") {
            let tmp = String::from_arg(arg).unwrap_or(String::new());
            assert_eq!("text/plain", tmp);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    if let Some(map) = converter.get_opt("max-size") {
        if let Some(Mixed::Single(arg)) = map.get("size") {
            let tmp = u16::from_arg(arg).unwrap_or(0);
            assert_eq!(1024, tmp);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    if let Some(map) = converter.get_opt("headers") {
        if let Some(Mixed::Multiply(args)) = map.get("headers") {
            let tmp = <Headers as FromArgs>::from_args(args).unwrap();
            assert_eq!(Headers {
                accept: "*/*".to_string(),
                accept_language: "zh-CN; en-US".to_string(),
                user_agent: "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:77.0) Gecko/20100101 Firefox/77.0".to_string(),
                connection: "keep-alive".to_string(),
            }, tmp);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    assert!(!converter.contains_opt("ssl"));
    assert!(!converter.contains_opt("s"));
    assert!(converter.contains_opt("u"));
    assert!(converter.contains_opt("url"));
}