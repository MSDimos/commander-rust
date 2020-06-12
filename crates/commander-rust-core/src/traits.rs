use crate::{ Argument, SubCommand, Options };

pub trait GetArgs {
    fn get_args(&self) -> &Vec<Argument>;
}

pub trait GetOpts {
    fn get_opts(&self) -> &Vec<Options>;
}

pub trait PushSubCommand {
    fn push_sub_command(&mut self, _: SubCommand);
}

pub trait PushOptions {
    fn push_option(&mut self, _: Options);
}

pub trait PushArgument {
    fn push_argument(&mut self, _: Argument);
}

pub trait ValidateArgs {
    fn validate_args(&self) -> bool;
}

pub trait ContainsOpt {
    fn contains_option(&self, opt: &str) -> bool;
}

pub trait GetOpt {
    fn get_opt(&self, opt: &str) -> Option<&Options>;
}