#![feature(proc_macro_hygiene)]

use commander_rust::{ option, sub_command, command };
use commander_rust::{ Options, SubCommand, };
use commander_rust::traits::{ PushOptions };

#[test]
#[option(-m, --movie <movie_name>, "play a movie")]
#[option(--music <music_name>, "play a music")]
#[sub_command(play, "play media source")]
fn play() {
    assert_eq!(
        Options::from(r#"-m, --movie <movie_name>, "play a movie""#),
        _commander_rust_prefix_play_movie_commander_rust_suffix_(),
    );

    assert_eq!(
        Options::from(r#"--music <music_name>, "play a music""#),
        _commander_rust_prefix_play_music_commander_rust_suffix_(),
    );

    let mut sub_command = SubCommand::new("".to_string(), "play".to_string(), Some("play media source".to_string()));

    sub_command.push_option(Options::from(r#"-m, --movie <movie_name>, "play a movie""#));
    sub_command.push_option(Options::from(r#"--music <music_name>, "play a music""#));
    assert_eq!(sub_command, _commander_rust_prefix_play_commander_rust_suffix_());
}

#[test]
#[option(-u, --upload <file_name>, "upload a file")]
#[option(-d, --download <file_name>, "download a file")]
#[sub_command(transfer, "transfer file between server and client")]
fn transfer() {
    assert_eq!(
        Options::from(r#"-u, --upload <file_name>, "upload a file""#),
        _commander_rust_prefix_transfer_upload_commander_rust_suffix_(),
    );

    assert_eq!(
        Options::from(r#"-d, --download <file_name>, "download a file""#),
        _commander_rust_prefix_transfer_download_commander_rust_suffix_(),
    );

    // let mut sub_command = SubCommand::from(r#"media -> transfer, "transfer file between server and client""#);
    let mut sub_command = SubCommand::new("".to_string(), "transfer".to_string(), Some("transfer file between server and client".to_string()));

    sub_command.push_option(Options::from(r#"-u, --upload <file_name>, "upload a file""#));
    sub_command.push_option(Options::from(r#"-d, --download <file_name>, "download a file""#));
    assert_eq!(sub_command, _commander_rust_prefix_transfer_commander_rust_suffix_());
}

#[test]
#[option(-v, --version, "print version information")]
#[option(-h, --help, "print help information")]
#[option(--nocapture, "display everything in test mode")]
#[command(media, "media device emulator")]
fn media_emulator() {
    assert_eq!(
        Options::from(r#"-v, --version, "print version information""#),
        _commander_rust_prefix_media_emulator_version_commander_rust_suffix_(),
    );

    assert_eq!(
        Options::from(r#"-h, --help, "print help information""#),
        _commander_rust_prefix_media_emulator_help_commander_rust_suffix_(),
    );

    assert_eq!(
        Options::from(r#"--nocapture, "display everything in test mode""#),
        _commander_rust_prefix_media_emulator_nocapture_commander_rust_suffix_(),
    );
}
