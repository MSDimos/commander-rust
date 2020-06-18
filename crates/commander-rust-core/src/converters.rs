use crate::parser::{ Segment, ParserResult, };
use crate::traits::{ GetArgs, GetOpt };
use crate::Command;
use std::ops::{ Deref, DerefMut };
use std::str::FromStr;
use std::num::ParseIntError;
use std::path::{ PathBuf, Path };
use std::fmt::Debug;
use std::fmt;
use std::collections::HashMap;

/// type conversion needed
pub trait FromArg<'a>: Sized {
    type Error: Debug;
    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error>;
}

pub trait FromArgs<'a>: Sized {
    type Error: Debug;
    fn from_args(args: &'a Args) -> Result<Self, Self::Error>;
}

#[derive(Clone, Debug)]
pub struct Arg(pub String);

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Arg {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Arg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> FromArg<'a> for &'a Arg {
    type Error = ();

    #[inline]
    fn from_arg(arg: &'a Arg) -> Result<&'a Arg, Self::Error> {
        Ok(arg)
    }
}

impl<'a> FromArg<'a> for String {
    type Error = ();

    #[inline]
    fn from_arg(arg: &'a Arg) -> Result<String, Self::Error> {
        Ok(arg.0.clone())
    }
}

impl<'a> FromArg<'a> for &'a str {
    type Error = ();

    #[inline]
    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        Ok(&arg.0)
    }
}

impl<'a> FromArg<'a> for PathBuf {
    type Error = ();

    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        Ok(PathBuf::from(arg.as_str()))
    }
}

impl<'a> FromArg<'a> for &'a Path {
    type Error = ();

    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        Ok(Path::new(arg.as_str()))
    }
}

macro_rules! impl_ints {
    ($($T: ty),+) => {
        $(
            impl<'a> FromArg<'a> for $T {
                type Error = ParseIntError;

                #[inline]
                fn from_arg(arg: &'a Arg) -> Result<$T, Self::Error> {
                    <$T as FromStr>::from_str(&arg)
                }
            }
        )*
    };
}

impl_ints![u8, u16, u32, u64, u128];
impl_ints![i8, i16, i32, i64, i128];

impl<'a, T: FromArg<'a>> FromArg<'a> for Result<T, T::Error> {
    type Error = T::Error;

    #[inline]
    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        match T::from_arg(arg) {
            Ok(val) => Ok(Ok(val)),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<'a, T: FromArg<'a>> FromArg<'a> for Option<T> {
    type Error = ();

    #[inline]
    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        match T::from_arg(arg) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Args(pub Vec<Arg>);

impl From<Segment> for Args {
    fn from(seg: Segment) -> Args {
        match seg {
            Segment::Short(_, args)
            | Segment::Long(_, args)
            | Segment::Command(_, args) => {
                let mut args_strs = vec![];

                for arg in args {
                    if let Segment::Raw(str) = arg {
                        args_strs.push(Arg(str));
                    }
                }

                Args(vec![])
            }
            _ => Args(vec![]),
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for arg in self.iter() {
            s = format!("{} {}", s, arg);
        }

        write!(f, "{}", s.trim())
    }
}

impl Deref for Args {
    type Target = Vec<Arg>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Args {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> FromArgs<'a> for String {
    type Error = ();

    fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
        Ok(format!("{}", args))
    }
}

impl<'a> FromArgs<'a> for &'a Args {
    type Error = ();

    fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
        Ok(args)
    }
}

impl<'a, T: FromArg<'a>> FromArgs<'a> for Vec<T> {
    type Error = T::Error;

    fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
        let mut result = vec![];

        for arg in args.iter() {
            match T::from_arg(arg) {
                Ok(val) => result.push(val),
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }
}


impl<'a, T: FromArgs<'a>> FromArgs<'a> for Option<T> {
    type Error = T::Error;

    #[inline]
    fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
        match T::from_args(args) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}

impl<'a, T: FromArgs<'a>> FromArgs<'a> for Result<T, T::Error> {
    type Error = T::Error;

    fn from_args(args: &'a Args) -> Result<Self, Self::Error> {
        match T::from_args(args) {
            Ok(val) => Ok(Ok(val)),
            Err(e) => Ok(Err(e)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mixed {
    Single(Arg),
    Multiply(Args),
}

#[derive(Debug, Clone, Default)]
pub struct Application {
    pub sub_name: Option<String>,
    pub sub_args: HashMap<String, Mixed>,
    pub cmd_args: HashMap<String, Mixed>,
    pub local_opts: HashMap<String, HashMap<String, Mixed>>,
    pub global_opts: HashMap<String, HashMap<String, Mixed>>,
    pub(crate) command: Command,
}

// alias it for using
pub type App = Application;

impl Application {
    fn extract_args<T: GetArgs>(args: &[Segment], def: &T) -> HashMap<String, Mixed> {
        let mut cmd_args = HashMap::new();

        // Since the arguments have been validated when entering this step,
        // the input arguments must match the defined arguments
        for (idx, cmd_arg) in def.get_args().iter().enumerate() {
            if !cmd_arg.ty.is_multiply() {
                if cmd_arg.ty.is_required() {
                    // for <arg>
                    if let Segment::Raw(s) = &args[idx] {
                        cmd_args.insert(cmd_arg.name.clone(), Mixed::Single(Arg(s.to_string())));
                    }
                } else {
                    // for [arg]
                    if !args.is_empty() && idx < args.len() {
                        // [arg] with input
                        if let Segment::Raw(s) = &args[idx] {
                            cmd_args.insert(cmd_arg.name.clone(), Mixed::Single(Arg(s.to_string())));
                        }
                    } else {
                        // [arg] without input
                        cmd_args.insert(cmd_arg.name.clone(), Mixed::Single(Arg(String::new())));
                    }
                }
            } else if idx < args.len() {
                // for <..args> or [..args]
                let mut mixed_args = vec![];

                for arg in args.iter().skip(idx) {
                    if let Segment::Raw(s) = arg {
                        mixed_args.push(Arg(s.to_string()));
                    }
                }

                cmd_args.insert(cmd_arg.name.clone(), Mixed::Multiply(Args(mixed_args)));
            }
        }

        cmd_args
    }

    fn extract_args_for_options<T: GetOpt>(opts: &[Segment], def: &T) -> HashMap<String, HashMap<String, Mixed>> {
        let mut mixed_opts = HashMap::new();

        for opt in opts.iter() {
            match opt {
                Segment::Long(name, args) => {
                    if let Some(opt) = def.get_long_opt(name) {
                        if let Some(short) = &opt.short {
                            mixed_opts.insert(short.to_string(), Self::extract_args(args, opt));
                        }
                        mixed_opts.insert(opt.long.to_string(), Self::extract_args(args, opt));
                    }
                },
                Segment::Short(name, args) => {
                    if let Some(opt) = def.get_short_opt(name) {
                        if let Some(short) = &opt.short {
                            mixed_opts.insert(short.to_string(), Self::extract_args(args, opt));
                        }
                        mixed_opts.insert(opt.long.to_string(), Self::extract_args(args, opt));
                    }
                }
                _ => {}
            }
        }

        mixed_opts
    }

    pub fn from_parser_result(parser_result: &ParserResult, cmd: &Command) -> Result<Self, String> {
        if let Ok(((in_cmd, in_sub), in_local_opts, in_global_opts)) = parser_result {
            let mut sub_name = None;
            let mut local_opts = HashMap::new();
            let cmd_args = if let Some(Segment::Command(_, args)) = in_cmd {
                Self::extract_args(args, cmd)
            } else { HashMap::new() };
            let sub_args = if let Some(Segment::Command(Some(name), args)) = in_sub {
                sub_name = Some(name.to_string());

                if let Some(sub_cmd) = cmd.get_sub_cmd(name) {
                    local_opts = Self::extract_args_for_options(&in_local_opts, sub_cmd);
                    Self::extract_args(args, sub_cmd)
                } else {
                    return Err(format!("can not find `{}`?", name));
                }
            } else { HashMap::new() };
            let global_opts = Self::extract_args_for_options(in_global_opts, cmd);

            return Ok(Application {
                sub_name,
                sub_args,
                cmd_args,
                local_opts,
                global_opts,
                command: cmd.clone(),
            });
        }

        Err("can't parse cli as specified format".to_string())
    }

    pub fn contains_opt<T: ToString>(&self, key: T) -> bool {
        let key = key.to_string();
        self.local_opts.contains_key(&key)
    }

    pub fn contains_global_opt<T: ToString>(&self, key: T) -> bool {
        self.global_opts.contains_key(&key.to_string())
    }

    pub fn get_opt<T: ToString>(&self, key: T) -> Option<&HashMap<String, Mixed>> {
        let key_s = key.to_string();

        if self.contains_opt(key) {
            if self.local_opts.contains_key(&key_s) {
                self.local_opts.get(&key_s)
            } else {
                self.global_opts.get(&key_s)
            }
        } else {
            None
        }
    }

    pub fn get_sub_arg<T: ToString>(&self, key: T) -> Option<&Mixed> {
        self.sub_args.get(&key.to_string())
    }

    pub fn get_cmd_arg<T: ToString>(&self, key: T) -> Option<&Mixed> {
        self.cmd_args.get(&key.to_string())
    }

    pub fn sub_name(&self) -> String {
        self.sub_name.clone().unwrap()
    }

    pub fn args_len(&self) -> usize {
        self.sub_args.len()
    }

    pub fn opts_len(&self) -> usize {
        self.local_opts.len() + self.global_opts.len()
    }
}

pub trait FromApp<'a>: Sized {
    type Error: Debug;
    fn from_app(app: &'a Application) -> Result<Self, Self::Error>;
}

impl<'a> FromApp<'a> for Application {
    type Error = ();

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        Ok(app.clone())
    }
}

impl<'a> FromApp<'a> for &'a Application {
    type Error = ();

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        Ok(&app)
    }
}

impl<'a> FromApp<'a> for &'a Command {
    type Error = ();

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        Ok(&app.command)
    }
}

#[derive(Debug)]
pub struct Opts(pub HashMap<String, HashMap<String, Mixed>>);

#[derive(Debug)]
pub struct GlobalOpts(pub HashMap<String, HashMap<String, Mixed>>);

impl Deref for Opts {
    type Target = HashMap<String, HashMap<String, Mixed>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Opts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for GlobalOpts {
    type Target = HashMap<String, HashMap<String, Mixed>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalOpts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> FromApp<'a> for Opts {
    type Error = ();

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        Ok(Opts(app.local_opts.clone()))
    }
}

impl<'a> FromApp<'a> for GlobalOpts {
    type Error = ();

    fn from_app(app: &'a Application) -> Result<Self, Self::Error> {
        Ok(GlobalOpts(app.global_opts.clone()))
    }
}

impl<'a, T: FromApp<'a>> FromApp<'a> for Result<T, T::Error> {
    type Error = T::Error;

    #[inline]
    fn from_app(app: &'a App) -> Result<Self, Self::Error> {
        match T::from_app(app) {
            Ok(val) => Ok(Ok(val)),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<'a, T: FromApp<'a>> FromApp<'a> for Option<T> {
    type Error = ();

    #[inline]
    fn from_app(app: &'a App) -> Result<Self, Self::Error> {
        match T::from_app(app) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}

