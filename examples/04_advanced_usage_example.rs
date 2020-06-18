#![feature(proc_macro_hygiene)]

use commander_rust::{ execute, option, command, sub_command, default_options };
use commander_rust::{ Application, GlobalOpts, Command, Arg };
use commander_rust::traits::{ FromApp, FromArg };

enum DangerousThing {
    Cephalosporin,
    Wine,
    None,
}

// by implementing the trait `FromApp`, you can do many multiple custom options types
// e.g. here, mutually exclusive options
struct MutexThing(DangerousThing);

impl<'a> FromApp<'a> for MutexThing {
    type Error = String;

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        if let Ok(opts) = GlobalOpts::from_app(app) {
            if opts.contains_key("cephalosporin") && opts.contains_key("drink-wine") {
                Err(String::from("DANGER!!! DO NOT DO IT! DO NOT take cephalosporin while drinking wine!"))
            } else if opts.contains_key("cephalosporin") {
                Ok(MutexThing(DangerousThing::Cephalosporin))
            } else if opts.contains_key("drink-wine") {
                Ok(MutexThing(DangerousThing::Wine))
            } else {
                Ok(MutexThing(DangerousThing::None))
            }
        } else {
            Ok(MutexThing(DangerousThing::None))
        }
    }
}

enum Food {
    Noodles,
    Beef,
    Fish,
}

impl<'a> FromArg<'a> for Food {
    type Error = ();

    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        match arg.as_str() {
            "noodles" => Ok(Food::Noodles),
            "beef" => Ok(Food::Beef),
            "fish" => Ok(Food::Fish),
            _ => Err(()),
        }
    }
}

// WARN: DO NOT take cephalosporin while drinking wine! It's fatal behavior!!!!!!!!
#[option(--cephalosporin, "take cephalosporin")]
#[option(--drink-wine, "drink wine")]
#[default_options]
#[command("0.0.1-fruits-eater", eat [food], "eat food")]
fn eat_fn(food: Option<Food>, mutex_thing: Result<MutexThing, String>) {
    match food {
        Some(Food::Noodles) | Some(Food::Beef) | Some(Food::Fish) => println!("I eat it, I like it!"),
        _ => println!("I dislike it."),
    }

    match mutex_thing {
        Ok(MutexThing(DangerousThing::Cephalosporin)) => println!("DO NOT drink wine recently!"),
        Ok(MutexThing(DangerousThing::Wine)) => println!("DO NOT take cephalosporin recently!"),
        Ok(MutexThing(DangerousThing::None)) => {},
        Err(note) => println!("{}", note),
    }
}

#[sub_command(help, "display help information")]
fn help_fn(cmd: &Command) {
    cmd.println();
}

#[sub_command(version, "display version information")]
fn version_fn(cmd: &Command) {
    cmd.println_version();
}

fn main() {
    execute!(eat_fn, [help_fn, version_fn]);
}