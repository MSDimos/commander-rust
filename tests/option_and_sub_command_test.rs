#![allow(warnings)]

use commander_rust::{sub_command, option};
use commander_rust::{ Argument, Options, SubCommand };

#[test]
#[option(-v, --version, "Print version information")]
#[option(-h, --help, "Print help information")]
#[sub_command(basic, "basic functions u want to use")]
fn basic() {
    assert_eq!(
        Options::from(r#"-v, --version, "Print version information""#),
        _commander_rust_prefix_basic_version_commander_rust_suffix_(),
    );

    assert_eq!(
        Options::from(r#"-h, --help, "Print help information""#),
        _commander_rust_prefix_basic_help_commander_rust_suffix_(),
    );

    assert_eq!(
        Options::from(r#"-h, --help, "Print help information""#),
        _commander_rust_prefix_basic_help_commander_rust_suffix_(),
    );
}

#[test]
#[option(-a, --ak47, "Da da da da~")]
#[option(-m, --m4a1, "Biu biu biu biu~")]
#[option(--barrett-m82a1, "Bang bang bang!")]
#[sub_command(shoot, "Are u ready?")]
fn shoot() {
    use commander_rust::traits::PushOptions;

    let mut sub_command = SubCommand::new(String::new(), "shoot".to_string(), Some("Are u ready?".to_string()));

    sub_command.push_option(Options::from(r#"-a, --ak47, "Da da da da~""#));
    sub_command.push_option(Options::from(r#"-m, --m4a1, "Biu biu biu biu~""#));
    sub_command.push_option(Options::from(r#"--barrett-m82a1, "Bang bang bang!""#));
    assert_eq!(sub_command, _commander_rust_prefix_shoot_commander_rust_suffix_());
}
