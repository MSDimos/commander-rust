pub mod parser;
pub mod traits;
pub mod errors;
pub mod converters;

#[cfg(feature = "test")]
use regex::Regex;

use traits::{GetArgs, ValidateArgs, PushOptions};
use std::collections::HashSet;
use std::fmt;
use traits::{PushSubCommand, PushArgument};
use std::option::Option::Some;
use crate::traits::{GetOpts, GetOpt };
use colored::Colorize;

/// Note: These `struct`s are different from `struct`s with same names
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgumentType {
    RequiredSingle,
    OptionalSingle,
    RequiredMultiple,
    OptionalMultiple,
}

impl ArgumentType {
    pub(crate) fn is_multiply(&self) -> bool {
        match self {
            &ArgumentType::RequiredMultiple | &ArgumentType::OptionalMultiple => true,
            _ => false,
        }
    }

    pub(crate) fn is_required(&self) -> bool {
        match self {
            &ArgumentType::RequiredSingle | &ArgumentType::RequiredMultiple => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    pub name: String,
    pub ty: ArgumentType,
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.ty {
            ArgumentType::RequiredSingle => format!("<{}>", self.name),
            ArgumentType::RequiredMultiple => format!("<..{}>", self.name),
            ArgumentType::OptionalSingle => format!("[{}]", self.name),
            ArgumentType::OptionalMultiple => format!("[..{}]", self.name),
        };

        write!(f, "{}", s)
    }
}

#[cfg(feature = "test")]
impl From<String> for Argument {
    fn from(s: String) -> Self {
        let rs_re = Regex::new(r"^<(?P<name>(\w|_)+)>$").unwrap().captures(&s);
        let rm_re = Regex::new(r"^<\.\.\.?(?P<name>(\w|_)+)>$").unwrap().captures(&s);
        let os_re = Regex::new(r"^\[(?P<name>(\w|_)+)\]$").unwrap().captures(&s);
        let om_re = Regex::new(r"^\[\.\.\.?(?P<name>(\w|_)+)\]$").unwrap().captures(&s);

        if let Some(rs_re) = rs_re {
            Argument {
                name: rs_re["name"].to_string(),
                ty: ArgumentType::RequiredSingle,
            }
        } else if let Some(rm_re) = rm_re {
            Argument {
                name: rm_re["name"].to_string(),
                ty: ArgumentType::RequiredMultiple,
            }
        } else if let Some(os_re) = os_re {
            Argument {
                name: os_re["name"].to_string(),
                ty: ArgumentType::OptionalSingle,
            }
        } else if let Some(om_re) = om_re {
            Argument {
                name: om_re["name"].to_string(),
                ty: ArgumentType::OptionalMultiple,
            }
        } else {
            Argument {
                name: String::new(),
                ty: ArgumentType::OptionalSingle,
            }
        }
    }
}

#[cfg(feature = "test")]
impl From<&str> for Argument {
    fn from(s: &str) -> Self {
        Argument::from(s.to_string())
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Options {
    pub short: Option<String>,
    pub long: String,
    opt_args: Vec<Argument>,
    pub desc: Option<String>,
}

impl Options {
    pub fn new(short: Option<String>, long: String, desc: Option<String>) -> Self {
        Options {
            short,
            long,
            opt_args: vec![],
            desc,
        }
    }
}

impl GetArgs for Options {
    fn get_args(&self) -> &Vec<Argument> {
        &self.opt_args
    }
}

#[cfg(feature = "test")]
impl From<String> for Options {
    fn from(s: String) -> Self {
        let short_pat = r"\-(?P<short>[[:alpha:]])";
        let long_pat = r"\-\-(?P<long>(-?[[:word:]])+)";
        let desc_pat = r#""(?P<desc>.*)""#;
        let pattern = format!("({}, )?{}( (?P<args>[^,]+))?(, {})?", short_pat, long_pat, desc_pat);
        let re = Regex::new(&pattern).unwrap();
        let cap = re.captures(&s).unwrap();
        let desc = if cap.name("desc").is_some() {
            Some(cap["desc"].to_string())
        } else { None };
        let short = if cap.name("short").is_some() {
            Some(cap["short"].to_string())
        } else { None };
        let mut options = Options::new(short, cap["long"].to_string(), desc);

        if cap.name("args").is_some() {
            let args_s = &s[cap.name("args").unwrap().range()];
            let args: Vec<&str> = args_s.split_terminator(' ').collect();

            for arg_s in args {
                let arg = Argument::from(arg_s);

                if !arg.name.is_empty() {
                    options.push_argument(arg);
                }
            }
        }

        options
    }
}

#[cfg(feature = "test")]
impl From<&str> for Options {
    fn from(s: &str) -> Self {
        Options::from(s.to_string())
    }
}

impl PartialEq<Options> for Options {
    fn eq(&self, other: &Options) -> bool {
        (self.short.is_some() && self.short == other.short) || self.long == other.long
    }
}

impl PushArgument for Options {
    fn push_argument(&mut self, argument: Argument) {
        self.opt_args.push(argument);

        if !self.validate_args() {
            self.opt_args.pop();
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubCommand {
    // only using for display information
    pub belong: String,
    pub name: String,
    cmd_args: Vec<Argument>,
    options: Vec<Options>,
    pub desc: Option<String>,
}

impl SubCommand {
    pub fn new(belong: String, name: String, desc: Option<String>) -> Self {
        SubCommand {
            belong,
            name,
            cmd_args: vec![],
            options: vec![],
            desc,
        }
    }

    pub fn println(&self) {
        println!("{}", self);
    }
}

#[cfg(feature = "test")]
impl From<String> for SubCommand {
    fn from(str: String) -> Self {
        // pattern: [main -> sub <a> <b> [c], ]["hello world!"]
        let re = Regex::new(r#"^((?P<belong>(\w|_)+) -> (?P<name>(\w|_)+)( (?P<args>[^,]+))?)?(, )?("(?P<desc>.*)")?$"#).unwrap();
        let cap = re.captures(&str).unwrap();
        let mut belong = String::new();
        let mut name = String::new();
        let mut desc = None;

        if cap.name("belong").is_some() {
            belong = cap["belong"].to_string();
            name = cap["name"].to_string();
        }

        if cap.name("desc").is_some() {
            desc = Some(cap["desc"].to_string());
        }
        let mut sub_command = SubCommand::new(belong, name, desc);

        if cap.name("args").is_some() {
            let args_s = &str[cap.name("args").unwrap().range()];
            let args: Vec<&str> = args_s.split_terminator(' ').collect();

            for arg_s in args {
                let arg = Argument::from(arg_s);

                if !arg.name.is_empty() {
                    sub_command.push_argument(arg);
                }
            }
        }

        sub_command
    }
}

#[cfg(feature = "test")]
impl From<&str> for SubCommand {
    fn from(s: &str) -> Self {
        SubCommand::from(s.to_string())
    }
}

impl fmt::Display for SubCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tab = String::from("    ");

        if let Some(desc) = &self.desc {
            writeln!(f, "{}:", "DESCRIPTION".bold().italic()).unwrap();
            writeln!(f, "{tab}{}", desc, tab = tab).unwrap();
        }

        let mut args = String::new();

        if !self.cmd_args.is_empty() {
            for arg in self.cmd_args.iter() {
                args.push_str(&format!(" {}", arg));
            }
        }

        writeln!(f, "\n{}:", "USAGE".bold().italic()).unwrap();

        if self.options.is_empty() {
            writeln!(f, "{tab}{} {}{}\n", self.belong, self.name, args, tab = tab).unwrap();
        } else {
            writeln!(f, "{tab}{} {}{} [--options]\n", self.belong, self.name, args, tab = tab).unwrap();
        }

        if !self.options.is_empty() {
            writeln!(f, "{}:", "OPTIONS".bold().italic()).unwrap();

            let mut width = 0;
            let mut opts_str = vec![];

            for opt in &self.options {
                let opt_name_str = if let Some(short_name) = &opt.short {
                    format!("-{}, --{}", short_name, opt.long)
                } else {
                    format!("{tab}--{}", opt.long, tab = tab)
                };
                let opt_args_str = if !opt.opt_args.is_empty() {
                    let mut opt_args = String::new();

                    for arg in opt.opt_args.iter() {
                        opt_args.push_str(&format!(" {}", arg));
                    }

                    opt_args
                } else {
                    String::new()
                };

                let opt_str = format!("{}{}", opt_name_str, opt_args_str);

                width = width.max(opt_str.len());
                opts_str.push(format!("{}{}", opt_name_str, opt_args_str));
            }

            for (idx, opt_str) in opts_str.into_iter().enumerate() {
                if let Some(opt_desc) = &self.options[idx].desc {
                    writeln!(f, "{tab}{:<width$}{tab}{}", opt_str, opt_desc, width = width, tab = tab).unwrap();
                } else {
                    writeln!(f, "{tab}{:<width$}", opt_str, width = width, tab = tab).unwrap();
                }
            }
        }

        writeln!(f)
    }
}

impl GetArgs for SubCommand {
    fn get_args(&self) -> &Vec<Argument> {
        &self.cmd_args
    }
}

impl GetOpts for SubCommand {
    fn get_opts(&self) -> &Vec<Options> { &self.options }
}

impl PushOptions for SubCommand {
    fn push_option(&mut self, option: Options) {
        if !option.validate_args() {
            return;
        }

        let mut dup = false;

        for opt in self.options.iter() {
            if opt == &option {
                dup = true;
                break;
            }
        }

        if !dup {
            self.options.push(option);
        }
    }
}

impl PushArgument for SubCommand {
    fn push_argument(&mut self, argument: Argument) {
        self.cmd_args.push(argument);

        if !self.validate_args() {
            self.cmd_args.pop();
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Command {
    pub name: String,
    sub_cmds: Vec<SubCommand>,
    cmd_args: Vec<Argument>,
    options: Vec<Options>,
    pub desc: Option<String>,
    pub version: String,
}


impl Command {
    pub fn new(name: String, desc: Option<String>) -> Self {
        Command {
            name,
            cmd_args: vec![],
            sub_cmds: vec![],
            options: vec![],
            desc,
            version: String::from(std::env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn get_sub_cmd<'a>(&'a self, sub_name: &str) -> Option<&'a SubCommand> {
        for sub_cmd in &self.sub_cmds {
            if sub_name == sub_cmd.name {
                return Some(sub_cmd);
            }
        }

        None
    }

    pub fn println(&self) {
        println!("{}", self);
    }

    pub fn println_version(&self) {
        println!("{}", self.version);
    }

    pub fn println_sub<T: ToString>(&self, key: T) {
        if let Some(sub_cmd) = self.get_sub_cmd(&key.to_string()) {
            sub_cmd.println();
        }
    }
}

#[cfg(feature = "test")]
impl From<String> for Command {
    fn from(s: String) -> Self {
        let re = Regex::new(r#"^("(?P<version>.*)", )?(?P<name>(\w|_)+)( (?P<args>[^,]+))?(, ("(?P<desc>.*)"))?$"#).unwrap();
        let cap = re.captures(&s).unwrap();
        let name = cap["name"].to_string();
        let desc = if cap.name("desc").is_some() {
            Some(cap["desc"].to_string())
        } else {
            None
        };
        let mut cmd = Command::new(name, desc);

        if cap.name("version").is_some() {
            cmd.version = cap["version"].to_string();
        }

        if cap.name("args").is_some() {
            let args_s = &s[cap.name("args").unwrap().range()];
            let args: Vec<&str> = args_s.split_terminator(' ').collect();

            for arg_s in args {
                let arg = Argument::from(arg_s);

                if !arg.name.is_empty() {
                    cmd.push_argument(arg);
                }
            }
        }

        cmd
    }
}

#[cfg(feature = "test")]
impl From<&str> for Command {
    fn from(s: &str) -> Self {
        Command::from(s.to_string())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tab = String::from("    ");

        if let Some(desc) = &self.desc {
            writeln!(f, "{}:", "DESCRIPTION".bold().italic()).unwrap();
            writeln!(f, "{tab}{}", desc, tab = tab).unwrap();
        }

        writeln!(f, "\n{}:", "USAGE".bold().italic()).unwrap();

        let mut args = String::new();

        if !self.cmd_args.is_empty() {
            for arg in self.cmd_args.iter() {
                args.push_str(&format!(" {}", arg));
            }
        }

        let opt_fmt = if !self.options.is_empty() {
            " [--global-options]".to_string()
        } else {
            String::new()
        };
        let sub_cmd_fmt = if !self.sub_cmds.is_empty() {
            if self.sub_cmds.iter().any(|sub_cmd| !sub_cmd.options.is_empty()) {
                " [sub_commands] [--options]".to_string()
            } else {
                " [sub_commands]".to_string()
            }
        } else {
            String::new()
        };

        writeln!(f, "{tab}{}{}{}{}\n", self.name, args, opt_fmt, sub_cmd_fmt, tab = tab).unwrap();

        if !self.options.is_empty() {
            writeln!(f, "{}:", "OPTIONS".bold().italic()).unwrap();

            let mut width = 0;
            let mut opts_str = vec![];

            for opt in &self.options {
                let opt_name_str = if let Some(short_name) = &opt.short {
                    format!("-{}, --{}", short_name, opt.long)
                } else {
                    format!("{tab}--{}", opt.long, tab = tab)
                };
                let opt_args_str = if !opt.opt_args.is_empty() {
                    let mut opt_args = String::new();

                    for arg in opt.opt_args.iter() {
                        opt_args.push_str(&format!(" {}", arg));
                    }

                    opt_args
                } else {
                    String::new()
                };

                let opt_str = format!("{}{}", opt_name_str, opt_args_str);

                width = width.max(opt_str.len());
                opts_str.push(format!("{}{}", opt_name_str, opt_args_str));
            }

            for (idx, opt_str) in opts_str.into_iter().enumerate() {
                if let Some(opt_desc) = &self.options[idx].desc {
                    writeln!(f, "{tab}{:<width$}{tab}{}", opt_str, opt_desc, width = width, tab = tab).unwrap();
                } else {
                    writeln!(f, "{tab}{:<width$}", opt_str, width = width, tab = tab).unwrap();
                }
            }
        }

        if !self.sub_cmds.is_empty() {
            let mut width = 0;

            for sub_cmd in self.sub_cmds.iter() {
                width = width.max(sub_cmd.name.len() + 4);
            }

            writeln!(f, "\n{}:", "SUB_COMMANDS".bold().italic()).unwrap();

            for sub_cmd in self.sub_cmds.iter() {
                writeln!(
                    f,
                    "{tab}{:<width$}{}",
                    sub_cmd.name,
                    sub_cmd.desc.as_ref().unwrap_or(&String::new()),
                    tab = tab, width = width
                ).unwrap();
            }
        }


        writeln!(f)
    }
}

impl PushSubCommand for Command {
    fn push_sub_command(&mut self, sub_command: SubCommand) {
        let mut dup = false;

        for sub_cmd in self.sub_cmds.iter() {
            if sub_cmd == &sub_command {
                dup = true;
                break;
            }
        }

        if !dup {
            self.sub_cmds.push(sub_command);
        }
    }
}

impl PushOptions for Command {
    fn push_option(&mut self, option: Options) {
        if !option.validate_args() {
            return;
        }

        let mut dup = false;

        for opt in self.options.iter() {
            if opt == &option {
                dup = true;
                break;
            }
        }

        if !dup {
            self.options.push(option);
        }
    }
}

impl PushArgument for Command {
    fn push_argument(&mut self, argument: Argument) {
        self.cmd_args.push(argument);

        if !self.validate_args() {
            self.cmd_args.pop();
        }
    }
}

impl GetArgs for Command {
    fn get_args(&self) -> &Vec<Argument> {
        &self.cmd_args
    }
}

impl GetOpts for Command {
    fn get_opts(&self) -> &Vec<Options> {
        &self.options
    }
}

impl<T: GetArgs> ValidateArgs for T {
    fn validate_args(&self) -> bool {
        let mut opt_start = false;
        let mut names = HashSet::new();
        let args = self.get_args();

        for (idx, arg) in args.iter().enumerate() {
            if !arg.ty.is_required() {
                opt_start = true;
            }

            if idx != args.len() - 1 && arg.ty.is_multiply() {
                // only last argument could be optional, error
                return false;
            }


            if arg.ty.is_required() && opt_start {
                // all optional arguments should follow all required arguments, error
                // for instance, <a> <b> [c] [d] is valid, but <a> [b] <c> [d] is invalid
                return false;
            }

            let name = arg.name.to_string();

            if names.contains(&name) {
                return false;
            } else {
                names.insert(name);
            }
        }

        true
    }
}

impl<T: GetOpts> GetOpt for T {
    fn get_long_opt(&self, opt_name: &str) -> Option<&Options> {
        for opt in self.get_opts().iter() {
            if opt_name == opt.long {
                return Some(opt);
            }
        }

        None
    }

    fn get_short_opt(&self, opt_name: &str) -> Option<&Options> {
        for opt in self.get_opts().iter() {
            if let Some(short) = &opt.short {
                if short == opt_name {
                    return Some(opt);
                }
            }
        }

        None
    }
}
