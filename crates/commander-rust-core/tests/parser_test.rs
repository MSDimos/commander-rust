use commander_rust_core::parser::{Segment, SegmentWrapper};
use std::ffi::OsString;
use commander_rust_core::Command;

#[test]
fn empty_input() {
    let args_os: Vec<OsString> = vec![];
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test, "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_ok());

    if let Ok(((cmd, sub), local_opts, global_opts)) = output {
        assert!(cmd.is_none());
        assert!(sub.is_none());
        assert!(local_opts.is_empty());
        assert!(global_opts.is_empty());
    }
}

#[test]
fn one_input() {
    let args_os: Vec<OsString> = vec!["arg"]
        .iter()
        .map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test <arg>, "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_ok());

    if let Ok(((cmd, sub), local_opts, global_opts)) = output {
        assert_eq!(cmd, Some(Segment::Command(None, vec![Segment::Raw("arg".to_string())])));
        assert!(sub.is_none());
        assert!(local_opts.is_empty());
        assert!(global_opts.is_empty());
    }

    let args_os: Vec<OsString> = vec!["arg"]
        .iter()
        .map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test [arg], "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_ok());

    if let Ok(((cmd, sub), local_opts, global_opts)) = output {
        assert_eq!(cmd, Some(Segment::Command(None, vec![Segment::Raw("arg".to_string())])));
        assert!(sub.is_none());
        assert!(local_opts.is_empty());
        assert!(global_opts.is_empty());
    }

    let args_os: Vec<OsString> = vec![];
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test [arg], "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_ok());

    if let Ok(((cmd, sub), local_opts, global_opts)) = output {
        assert!(cmd.is_none());
        assert!(sub.is_none());
        assert!(local_opts.is_empty());
        assert!(global_opts.is_empty());
    }

    let args_os: Vec<OsString> = vec![];
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test <arg>, "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_err());
}

#[test]
fn multiply_inputs() {
    let args_os: Vec<OsString> = vec!["a", "b", "c"]
        .iter()
        .map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test <arg1> <arg2> [arg3], "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_ok());

    if let Ok(((cmd, sub), local_opts, global_opts)) = output {
        assert_eq!(
            cmd,
            Some(Segment::Command(
                None,
                vec![
                    Segment::Raw("a".to_string()),
                    Segment::Raw("b".to_string()),
                    Segment::Raw("c".to_string())
                ])
            )
        );
        assert!(sub.is_none());
        assert!(local_opts.is_empty());
        assert!(global_opts.is_empty());
    }


    let args_os: Vec<OsString> = vec!["a", "b"]
        .iter()
        .map(|s| OsString::from(s)).collect();
    let segments = Segment::from_vec(args_os);
    let mut segment_wrapper = SegmentWrapper(segments);
    let cmd = Command::from(r#"test <arg1> <arg2> [arg3], "test something""#);

    let output = segment_wrapper.parse_test(&cmd);

    assert!(output.is_ok());

    if let Ok(((cmd, sub), local_opts, global_opts)) = output {
        assert_eq!(
            cmd,
            Some(Segment::Command(
                None,
                vec![
                    Segment::Raw("a".to_string()),
                    Segment::Raw("b".to_string()),
                ])
            )
        );
        assert!(sub.is_none());
        assert!(local_opts.is_empty());
        assert!(global_opts.is_empty());
    }
}

