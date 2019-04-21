use commander_core::{ Instance, normalize, Raw };

#[test]
fn parse_instance() {
    let _args = vec!["", "rmdir", "/home/test", "/home/test2", "-rfo", "/home/output", "--display=no", "--format", "%s%s"];
    let args: Vec<String> = _args.clone().into_iter().map(|s| String::from(s)).collect();
    let instance = normalize(args.clone());
    let expect = vec![
        Instance {
            name: String::from("rmdir"),
            args: vec![String::from("/home/test"), String::from("/home/test2")],
        },
        Instance {
            name: String::from("r"),
            args: vec![],
        },
        Instance {
            name: String::from("f"),
            args: vec![],
        },
        Instance {
            name: String::from("o"),
            args: vec![String::from("/home/output")],
        },
        Instance {
            name: String::from("display"),
            args: vec![String::from("no")],
        },
        Instance {
            name: String::from("format"),
            args: vec![String::from("%s%s")],
        },
    ];

    assert_eq!(expect, instance);
}

#[test]
fn parse_raw() {
    let v = vec!["true", "false", "0", "0.1", "100", "9999999999999999", "hello world!"];
    let raw = Raw::new(v.iter().map(|&s| String::from(s)).collect());

    assert_eq!(0, raw.clone().into());
    assert_eq!(vec![0, 0, 0, 0, 100, 0, 0], {
        let v: Vec<i32> = raw.clone().into();
        v
    });
    assert_eq!(vec![0.0, 0.0, 0.0, 0.1, 100.0, 9999999999999999.0, 0.0], {
        let v: Vec<f32> = raw.clone().into();
        v
    });
    assert_eq!(true, raw.clone().into());
    assert_eq!(vec![true, false, false ,false, false, false, false], {
        let v: Vec<bool> = raw.clone().into();
        v
    });
    assert_eq!(vec!["true", "false", "0", "0.1", "100", "9999999999999999", "hello world!"], {
        let v: Vec<String> = raw.clone().into();
        v
    });
    assert_eq!(String::from("true"), {
        let s: String = raw.clone().into();
        s
    });
}