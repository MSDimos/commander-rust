#![feature(proc_macro_hygiene)]
use commander_rust::{ option, sub_command, command, register, run };

#[allow(dead_code)]
#[option(-m, --movie <movie_name>, "play a movie")]
#[option(--music <music_name>, "play a music")]
#[sub_command(play, "play media source")]
fn play_fn() {
    println!("running fn `play`");
}

#[allow(dead_code)]
#[option(-u, --upload <file_name>, "upload a file")]
#[option(-d, --download <file_name>, "download a file")]
#[sub_command(transfer, "transfer file between server and client")]
fn transfer_fn() {
    println!("running fn `transfer`");
}

#[allow(dead_code)]
#[option(-v, --version, "print version information")]
#[option(-h, --help, "print help information")]
#[option(--nocapture, "display everything in test mode")]
#[command(media, "media device emulator")]
fn media_fn() {
    println!("running fn `media`");
}

fn main() {
    register!(media_fn, [play_fn, transfer_fn]);
    run!();
}