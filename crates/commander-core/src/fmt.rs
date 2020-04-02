#![allow(unused_mut, dead_code)]

use std::fmt::{ Formatter, Result, Display };
use crate::{ Command, Argument, ArgumentType, Application };

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.ty {
            ArgumentType::RequiredSingle => write!(f, "<{}>", self.name),
            ArgumentType::RequiredMultiple => write!(f, "<{}...>", self.name),
            ArgumentType::OptionalSingle => write!(f, "[{}]", self.name),
            ArgumentType::OptionalMultiple => write!(f, "[{}...]", self.name),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut max_len = 0;
        let mut arg_formats = String::new();
        let mut lens = vec![];

        for opt in &self.opts {
            let arg_len = opt.arg.as_ref().map_or(0, |a| format!("{:}", a).len());
            let used_space = opt.long.len() + opt.short.len() + arg_len;

            if  used_space > max_len {
                max_len = used_space;
            }

            lens.insert(0, used_space);
        }

        for arg in &self.args {
            arg_formats.push_str(&format!("{:} ", arg));
        }

        if self.opts.len() > 0 {
            write!(f, "Usage: {} {}[options]\n\n", self.name, arg_formats)?;
        } else {
            write!(f, "Usage: {} {}\n\n", self.name, arg_formats)?;
        }

        write!(f, "{}\n\n", self.desc.clone().unwrap_or_default())?;

        if !self.opts.is_empty() {
            write!(f, "Private options: \n")?;

            for opt in &self.opts {
                let used_space = lens.pop().unwrap_or_default();
                let arg_format = opt.arg.as_ref().map_or(String::new(), |a| format!("{:}", a));

                write!(f, "  {}", format!("-{}, --{} {} {}", opt.short, opt.long, arg_format, " ".repeat(max_len - used_space)))?;
                write!(f, "  {}\n", opt.desc.clone().unwrap_or_default())?;
            }
        }

        write!(f, "\n")
    }
}


impl <Out> Display for Application<Out> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut max_len = 0;
        let mut lens = vec![];

        for opt in &self.opts {
            let arg_len = opt.arg.as_ref().map_or(0, |a| format!("{:}", a).len());
            let used_space = opt.long.len() + opt.short.len() + arg_len;

            if  used_space > max_len {
                max_len = used_space;
            }

            lens.insert(0, used_space);
        }

        write!(f, "Usage: {}", self.name)?;

        if self.cmds.len() > 0 {
            write!(f, " <command>")?;
        }

        if self.opts.len() > 0 {
            write!(f, " [options]")?;
        }

        if self.direct_args.len() > 0 {
            write!(f, " OR {} ", self.name)?;

            for arg in self.direct_args.iter() {
                write!(f, "{:} ", arg)?;
            }
            write!(f, "[options]")?;
        }

        write!(f, "\n\n{}\n\n", self.desc)?;

        if !self.opts.is_empty() {
            write!(f, "Public options: \n")?;

            for opt in &self.opts {
                let used_space = lens.pop().unwrap_or_default();
                let arg_format = opt.arg.as_ref().map_or(String::new(), |a| format!("{:}", a));

                write!(f, "  {}", format!("-{}, --{} {} {}", opt.short, opt.long, arg_format, " ".repeat(max_len - used_space)))?;
                write!(f, "  {}\n", opt.desc.clone().unwrap_or_default())?;
            }
        }

        if !self.cmds.is_empty() {
            write!(f, "\nCommands:\n")?;
            max_len = 0;

            for cmd in &self.cmds {
                let mut used_space = cmd.name.len() + 13;

                for arg in &cmd.args {
                    used_space += format!("{:}", arg).len() + 1;
                }

                if  used_space > max_len {
                    max_len = used_space;
                }

                lens.insert(0, used_space);
            }

            for cmd in &self.cmds {
                let used_space = lens.pop().unwrap_or_default();

                write!(f, "  {} ", cmd.name)?;

                for arg in &cmd.args {
                    write!(f, "{:} ", arg)?;
                }

                if cmd.opts.len() > 0 {
                    write!(f, "[options]")?;
                }

                write!(f, "{}  {}\n", " ".repeat(max_len - used_space), cmd.desc.clone().unwrap_or_default())?;
            }
        }

        if !self.cmds.is_empty() {
            write!(f, "\nSee '{} <command> --help' for more information on a specific command\n", self.name)
        } else {
            write!(f, "\n")
        }
    }
}