use colored::Colorize;
use std::ffi::OsString;
use crate::Command;
use crate::traits::{GetArgs, ValidateArgs, GetOpt};
use crate::errors::{raise_error, UNKNOWN_OPT, UNKNOWN_SUB, INTERNAL_ERROR, MISMATCHED_ARGS};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Segment {
    Short(String, Vec<Segment>),
    Long(String, Vec<Segment>),
    DoubleSub,
    Raw(String),
    // if first element is None, arguments belong to command
    // if it's not, arguments belong to the only sub_command
    Command(Option<String>, Vec<Segment>),
    None,
}

impl Segment {
    /// Parse according to the standard rules.
    /// There are several errors:
    /// 1. --long-option=
    /// 2. -short-opts
    /// 3. ------ // many -, but -- is valid
    /// 4. --long-option-
    pub fn from_vec(args_os: Vec<OsString>) -> Vec<Segment> {
        let mut segments = vec![];
        let mut opts_end = false;

        for arg_os in args_os.into_iter() {
            let arg_os: String = arg_os
                .into_string()
                .expect("parse cli arguments failed, this is most likely caused by a character encoding problem");

            if !opts_end {
                if arg_os.starts_with("--") && !arg_os.starts_with("---") {
                    if Self::is_long(&arg_os) {
                        if let Some(eq_idx) = arg_os.find('=') {
                            let key = &arg_os[2..eq_idx];
                            let value = &arg_os[(eq_idx + 1)..];

                            segments.push(
                                Segment::Long(
                                    key.to_string(),
                                    vec![Segment::Raw(value.to_string())],
                                )
                            );
                        } else if arg_os[2..].split('-')
                            .collect::<Vec<&str>>()
                            .iter()
                            .all(|cs| Self::is_lit_word(cs)) {
                            segments.push(Segment::Long(arg_os[2..].to_string(), vec![]));
                        } else {
                            segments.push(Segment::Raw(arg_os));
                        }
                    } else if Self::is_double_sub(&arg_os) {
                        opts_end = true;
                        segments.push(Segment::DoubleSub)
                    } else {
                        segments.push(Segment::Raw(arg_os.clone()));
                    }
                } else if arg_os.starts_with('-') {
                    if Self::is_short(&arg_os) {
                        let chrs: Vec<char> = arg_os[1..].chars().collect();

                        for chr in chrs {
                            let mut key = String::new();

                            key.push(chr);
                            segments.push(Segment::Short(key, vec![]));
                        }
                    } else {
                        segments.push(Segment::Raw(arg_os));
                    }
                } else if arg_os.is_empty() {
                    continue;
                } else {
                    segments.push(Segment::Raw(arg_os));
                }
            } else {
                segments.push(Segment::Raw(arg_os));
            }
        }

        segments
    }

    // lit word is a bit different from word, it consists of '_', alphabet and number
    pub fn is_lit_word(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c == '_' || char::is_alphanumeric(c))
    }

    pub fn is_short(s: &str) -> bool {
        s.len() >= 2 && s.starts_with('-') && Self::is_lit_word(&s[1..])
    }

    pub fn is_long(s: &str) -> bool {
        if s.len() >= 3 && s.starts_with("--") {
            // --abc or --abc-def-ghi
            if Self::is_lit_word(&s[2..]) || s[2..]
                .split('-')
                .collect::<Vec<&str>>()
                .iter()
                .all(|cs| Self::is_lit_word(cs)) {
                true
            } else {
                // --abc-def=value
                if s[2..].contains('=') {
                    let parts: Vec<&str> = s[2..].split('=').collect();

                    !s[2..].ends_with('=') && parts.len() >= 2 && parts[0]
                        .split_terminator('-')
                        .collect::<Vec<&str>>()
                        .iter()
                        .all(|cs| Self::is_lit_word(cs))
                } else {
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn is_double_sub(s: &str) -> bool {
        s == "--"
    }
}

#[derive(Debug)]
pub enum TerminatorKind {
    GlobalHelp,
    GlobalVersion,
    Help(String),
    Version(String),
    Other,
}

#[derive(Debug)]
pub enum TerminatorType {
    Version,
    Help,
    None,
}

pub type InputCmdArgs = Option<Segment>;
pub type InputSubArgs = Option<Segment>;
pub type InputGlobalOpts = Vec<Segment>;
pub type InputLocalOpts = Vec<Segment>;
pub type ParserResult = Result<((InputCmdArgs, InputSubArgs), InputLocalOpts, InputGlobalOpts), TerminatorKind>;

/// Note: Vec<Segment> doesn't contain the first element from `env::arg_os()`,
/// it's usually the absolute path of cli, e.g., `/usr/bin/bash`.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SegmentWrapper(pub Vec<Segment>);

impl SegmentWrapper {
    fn remove_cmd(&mut self, cmd: &Command) -> (Option<Segment>, Option<Segment>) {
        // argument after -- will be parsed as arguments of command or sub-command
        let mut raws = self.remove_raws();

        if !self.is_empty() {
            if let Segment::Command(_, _) = &self.0[0] {
                if let Segment::Command(name, mut args) = self.0.remove(0) {
                    if let Some(name) = name {
                        args.push(Segment::Raw(name));
                    }

                    let mut i = 0;
                    let mut sub = None;
                    let mut iter = args.iter();

                    while let Some(Segment::Raw(raw_str)) = iter.next() {
                        if cmd.get_sub_cmd(&raw_str).is_some() {
                            sub = Some(raw_str.clone());
                            args.remove(i);
                            break;
                        }

                        i += 1;
                    }

                    let mut cmd_args = vec![];
                    let mut sub_cmd_args = vec![];
                    let mut sub_name = String::new();

                    if let Some(sub_name_str) = sub {
                        for _ in 0..i {
                            cmd_args.push(args.remove(0));
                        }

                        sub_name = sub_name_str;
                        sub_cmd_args.append(&mut args);
                        sub_cmd_args.append(&mut raws);
                    } else {
                        cmd_args.append(&mut args);
                        cmd_args.append(&mut raws);
                    }

                    // three possibilities
                    // 1. None, Command => No command arguments but sub-command is offered
                    // 2. Command, Command => both command and sub-command are offered
                    // 3. Command, None => command is offered but no sub-command
                    // None, None is impossible because that `args` will not be empty
                    return if cmd_args.is_empty() {
                        (None, Some(Segment::Command(Some(sub_name), sub_cmd_args)))
                    } else if sub_cmd_args.is_empty() && sub_name.is_empty() {
                        (Some(Segment::Command(None, cmd_args)), None)
                    } else {
                        (Some(Segment::Command(None, cmd_args)), Some(Segment::Command(Some(sub_name), sub_cmd_args)))
                    };
                }
            } else if !raws.is_empty() {
                return (Some(Segment::Command(None, raws)), None);
            }
        } else if !raws.is_empty() {
            return (Some(Segment::Command(None, raws)), None);
        }

        (None, None)
    }

    fn remove_options(&mut self) -> Vec<Segment> {
        let mut options = vec![];
        let mut i = 0;

        while i < self.len() {
            match &self.0[i] {
                Segment::Long(_, _) | Segment::Short(_, _) => options.push(self.0.remove(i)),
                _ => i += 1,
            }
        }

        options
    }

    fn remove_global_options(&mut self, cmd: &Command) -> Vec<Segment> {
        let mut input_global_opts = vec![];
        let mut i = 0;

        while i < self.len() {
            match &self.0[i] {
                Segment::Short(name, _) => {
                    if cmd.get_short_opt(name).is_some() {
                        input_global_opts.push(self.0.remove(i));
                    } else {
                        i += 1;
                    }
                }
                Segment::Long(name, _) => {
                    if cmd.get_long_opt(name).is_some() {
                        input_global_opts.push(self.0.remove(i));
                    } else {
                        i += 1;
                    }
                }
                Segment::DoubleSub => break,
                _ => i += 1,
            }
        }

        input_global_opts
    }

    fn remove_raws(&mut self) -> Vec<Segment> {
        let mut raws = vec![];
        let mut i = 0;
        let len = self.len();

        while i < len {
            match &self.0[i] {
                Segment::DoubleSub => break,
                _ => i += 1,
            }
        }

        for _ in i..len {
            if let Segment::Raw(raw_s) = self.0.remove(i) {
                raws.push(Segment::Raw(raw_s));
            }
        }

        raws
    }

    /// divide arguments of options, for example:
    /// --long a b c -s d e f => divided as [[long, [a, b, c], [s, [d, e, f]] (actually not this, but another struct)
    /// --long=a b c -s d e f => [[long, [a, b, c], [s, [d, e, f]]
    fn divide_option_arguments(&mut self) {
        let mut left = 0;
        let mut right = 0;

        while left <= right && right < self.len() {
            match &mut self.0[right] {
                Segment::Short(_, _) | Segment::Long(_, _) => {
                    left = right;
                    right += 1;
                }
                // all input arguments after -- are raw arguments
                &mut Segment::DoubleSub => break,
                _ => {
                    // if left == right,
                    // it means that they are neither `Segment::Short` nor `Segment::Long`
                    if left == right && right < self.len() {
                        left += 1;
                        right += 1;
                    } else {
                        let r = self.0.remove(right);
                        match &mut self.0[left] {
                            Segment::Short(_, args) | Segment::Long(_, args) => args.push(r),
                            _ => continue,
                        }
                    }
                }
            }
        }
    }

    /// if sub-command or command accept arguments, divide their arguments
    /// all input argument before the first option are parsed as arguments of command or sub-command
    fn divide_cmd_arguments(&mut self) {
        if !self.0.is_empty() {
            let mut start = 0;
            let mut end = self.len();
            let mut cmd_args = vec![];

            // stop once encounter a short option or long option
            for i in 0..self.len() {
                match &self.0[i] {
                    Segment::Short(_, _) | Segment::Long(_, _) | Segment::DoubleSub => {
                        end = i;
                        break;
                    }
                    _ => continue,
                }
            }

            if let Segment::Command(_, _) = &mut self.0[0] {
                start = 1;
            }

            for _ in start..end {
                cmd_args.push(self.0.remove(start));
            }

            if start == 0 && !cmd_args.is_empty() {
                self.0.insert(0, Segment::Command(None, cmd_args));
            } else if let Segment::Command(_, args) = &mut self.0[0] {
                args.append(&mut cmd_args);
            }
        }
    }

    fn check_options<T: GetOpt>(options: &[Segment], cmd: &T) -> Result<(), String> {
        let mut all = true;
        let mut error = String::new();

        for opt in options {
            match opt {
                Segment::Short(name, _) => {
                    if cmd.get_short_opt(name).is_none() {
                        all = false;
                        error = format!("{} `{}`", UNKNOWN_OPT, format!("-{}", name).bold());
                        break;
                    }
                }
                Segment::Long(name, _) => {
                    if cmd.get_long_opt(name).is_none() {
                        all = false;
                        error = format!("{} `{}`", UNKNOWN_OPT, format!("--{}", name).bold());
                        break;
                    }
                }
                _ => continue,
            }
        }

        if all { Ok(()) } else { Err(error) }
    }

    // check arguments of command or sub-command, option
    fn check_arguments<T: GetArgs>(target: &Segment, source: &T) -> Result<(), String> {
        // if it's false, panic is necessary because it will not work at all
        // In theory it will never be false, because you can’t construct an invalid arguments group
        // but who can ensure anything in the world?
        if !source.validate_args() {
            return Err(format!("{} Position: {} {}", INTERNAL_ERROR, file!(), line!()));
        }

        let (mut min, mut max) = (0, 0);

        for arg in source.get_args() {
            if !arg.ty.is_multiply() {
                if arg.ty.is_required() {
                    // if it's <a>, min and max plus one
                    min += 1;
                    max += 1;
                } else {
                    // if it's [a], only max plus one
                    max += 1;
                }
            } else {
                // if it's <..a> or [..a], max is infinity
                // in fact, there is no difference between <..a> and [..a]
                // so it means that only accept up to `usize::max_value()` (depends on your cpu and system) parameters
                max = usize::max_value();
            }
        }
        let mut def_args_fmt = String::new();
        let mut input_args_fmt = String::new();
        let mut error = false;

        match target {
            Segment::Short(_, args)
            | Segment::Long(_, args)
            | Segment::Command(_, args) => {
                if args.len() < min || args.len() > max {
                    def_args_fmt = {
                        let tmp = source.get_args();
                        let mut str = String::new();

                        for arg in tmp.iter() {
                            str = format!("{}{} ", str, arg);
                        }

                        str.trim().to_string()
                    };
                    input_args_fmt = {
                        let mut str = String::new();

                        for arg in args.iter() {
                            let tmp = if let Segment::Raw(str) = arg { str.as_str() } else { "" };
                            if str.is_empty() {
                                str = tmp.to_string();
                            } else {
                                str = format!("{}, {}", str, tmp);
                            }
                        }

                        str.trim().to_string()
                    };
                    error = true;
                }
            }
            _ => {}
        }

        if error {
            let name = match target {
                Segment::Short(name, _) => format!("`{}`", format!("-{}", name).bold()),
                Segment::Long(name, _) => format!("`{}`", format!("--{}", name).bold()),
                Segment::Command(name, _) => {
                    if let Some(name) = name {
                        format!("sub-command `{}`", name.bold())
                    } else {
                        "command".to_string()
                    }
                }
                _ => "which".to_string(),
            };

            // two conditions, beautificate the error text
            // 1. lack of input arguments
            // 2. extra input arguments
            return if input_args_fmt.is_empty() {
                Err(format!(
                    "{} Arguments of {} are defined as `{}`, but you input nothing.",
                    MISMATCHED_ARGS, name, def_args_fmt.bold(),
                ))
            } else if def_args_fmt.is_empty() {
                Err(format!(
                    "{} {} doesn't accept any argument, but you input `{}`.",
                    MISMATCHED_ARGS, name, input_args_fmt.bold(),
                ))
            } else {
                Err(format!(
                    "{} Arguments of {} are defined as `{}`, but you input `{}`.",
                    MISMATCHED_ARGS, name, def_args_fmt.bold(), input_args_fmt.bold(),
                ))
            };
        }

        Ok(())
    }

    fn check_gol_option_arguments<T: GetOpt>(ins: &T, options: &[Segment]) -> Result<(), String> {
        for opt in options.iter() {
            match opt {
                Segment::Long(name, _) => {
                    if let Some(def_opt) = ins.get_long_opt(name) {
                        if let Err(err) = Self::check_arguments(opt, def_opt) {
                            return Err(err);
                        }
                    }
                }
                Segment::Short(name, _) => {
                    if let Some(def_opt) = ins.get_short_opt(name) {
                        if let Err(err) = Self::check_arguments(opt, def_opt) {
                            return Err(err);
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(())
    }

    fn parse(&mut self, cmd: &Command) -> ParserResult {
        let mut success = true;

        let tmp = if !self.0.is_empty() {
            // if self.0 is non-empty, do operations below
            // because some operations need to index at 0 which may raise errors

            // `divide_option_arguments`, `divide_cmd_arguments`
            // `remove_cmd`, `remove_global_options`, `remove_options`, `remove_raws`
            // it's best to call them one by one

            // divide arguments of options firstly in case some errors
            self.divide_option_arguments();
            self.divide_cmd_arguments();

            let (cmd_segs, sub_segs) = self.remove_cmd(cmd);
            let global_options = self.remove_global_options(cmd);
            let local_options = self.remove_options();

            // begin to validate input arguments, two steps

            // step-1: validate that all options are valid(if it's registered then it's valid)

            // validate local options and global options respectively
            // global options is always valid, because it's construct through the `Command` instance
            // u can assume `Command` is valid
            if let Some(sub_segs) = &sub_segs {
                if let Segment::Command(Some(sub_cmd_name), _) = sub_segs {
                    if let Some(sub_cmd) = cmd.get_sub_cmd(&sub_cmd_name) {
                        // if the sub-command offered is one of the sub-commands registered
                        // try to check whether all local-options belong to the sub-command offered or not
                        if let Err(err) = Self::check_options(&local_options, sub_cmd) {
                            success = raise_error(err);
                        }
                    } else {
                        // unreachable branch
                        success = raise_error(format!("{} `{}`", UNKNOWN_SUB, sub_cmd_name.bold()));
                    }
                }
            } else {
                // if no sub-command is offered but the local options are not empty
                // it means that these local options are unknown
                if let Some(local_opt) = local_options.get(0) {
                    if let Segment::Short(name, _) = local_opt {
                        success = raise_error(format!("{} `{}`", UNKNOWN_OPT, format!("-{}", name).bold()));
                    } else if let Segment::Long(name, _) = local_opt {
                        success = raise_error(format!("{} `{}`", UNKNOWN_OPT, format!("--{}", name).bold()));
                    }
                }
            }

            // check global options
            // In fact, this step will never raise error
            // because that `global_option` is parsed from `cmd`, so they are compatible
            if let Err(err) = Self::check_options(&global_options, cmd) {
                success = raise_error(err);
            }

            // step-2: validate that if arguments defined and arguments inputted are equivalent

            // check arguments of command if it offered
            if let Some(cmd_segs) = &cmd_segs {
                if let Segment::Command(none, _) = cmd_segs {
                    // command shouldn't have a name
                    if none.is_none() {
                        if let Err(err) = Self::check_arguments(cmd_segs, cmd) {
                            success = raise_error(err);
                        }
                    }
                }
            }

            // check arguments of sub-command if it offered
            if let Some(sub_segs) = &sub_segs {
                if let Segment::Command(sub_name, _) = sub_segs {
                    if let Some(sub_name) = sub_name {
                        if let Some(sub_cmd) = cmd.get_sub_cmd(sub_name) {
                            if let Err(err) = Self::check_arguments(sub_segs, sub_cmd) {
                                success = raise_error(err);
                            }

                            // check arguments of local options of specific sub-command
                            if let Err(err) = Self::check_gol_option_arguments(sub_cmd, &local_options) {
                                success = raise_error(err);
                            }
                        }
                    }
                }
            }

            // check arguments of global options
            if let Err(err) = Self::check_gol_option_arguments(cmd, &global_options) {
                success = raise_error(err);
            }

            ((cmd_segs, sub_segs), local_options, global_options)
        } else {
            // check if user doesn't input any arguments (sel.0 is empty)
            // but at this situation, command might accept arguments
            // if do not do any checking, it will raise some `Rust` runtime errors which are difficult to understand and debug
            // so do checking through constructing an empty `Segment::Command`
            if let Err(err) = Self::check_arguments(&Segment::Command(None, vec![]), cmd) {
                success = raise_error(err);
            }

            ((None, None), vec![], vec![])
        };

        if success { Ok(tmp) } else { Err(TerminatorKind::Other) }
    }

    #[cfg(feature = "test")]
    pub fn parse_test(&mut self, cmd: &Command) -> ParserResult {
        self.parse(cmd)
    }

    pub fn get_terminator(&self) -> TerminatorType {
        for seg in self.0.iter() {
            match seg {
                Segment::Short(name, _) => {
                    if name == "v" {
                        return TerminatorType::Version;
                    } else if name == "h" {
                        return TerminatorType::Help;
                    }
                }
                Segment::Long(name, _) => {
                    if name == "version" {
                        return TerminatorType::Version;
                    } else if name == "help" {
                        return TerminatorType::Help;
                    }
                }
                _ => {}
            }
        }

        TerminatorType::None
    }

    // return value is only using for testing
    pub fn parse_cli(cmd: &Command) -> ParserResult {
        // first element is useless.collect();
        let args_os: Vec<OsString> = std::env::args_os().skip(1).collect();
        let segments = Segment::from_vec(args_os);
        let mut segment_wrapper = SegmentWrapper(segments);
        let terminator = segment_wrapper.get_terminator();
        let first_sub = if segment_wrapper.is_empty() {
            None
        } else if let Segment::Raw(may_sub_name) = &segment_wrapper.0[0] {
            cmd.get_sub_cmd(may_sub_name)
        } else { None };

        // if any terminator inputted and defined (e.g., v, version, h, help)
        // it will not parse, because they have special callbacks
        match terminator {
            TerminatorType::Help => {
                if let Some(sub) = first_sub {
                    if sub.get_long_opt("help").is_some() {
                        return Err(TerminatorKind::Help(sub.name.clone()));
                    }
                }

                if cmd.get_long_opt("help").is_some() {
                    return Err(TerminatorKind::GlobalHelp);
                }
            }
            TerminatorType::Version => {
                if let Some(sub) = first_sub {
                    if sub.get_long_opt("version").is_some() {
                        return Err(TerminatorKind::Version(sub.name.clone()));
                    }
                }

                if cmd.get_long_opt("version").is_some() {
                    return Err(TerminatorKind::GlobalVersion);
                }
            }
            TerminatorType::None => {}
        }

        segment_wrapper.parse(cmd)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

