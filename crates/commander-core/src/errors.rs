use std::process::exit;

pub fn error(msg: &str) {
    eprintln!("{}: {}", ERR_RED, msg);
    exit(500);
}

type Msg = &'static str;

#[cfg(feature = "cn")]
pub const ERR_RED: Msg = "\x1b[0;31m 编译错误 \x1b[0m";
#[cfg(feature = "en")]
pub const ERR_RED: Msg = "\x1b[0;31m Compile time error \x1b[0m";

//#[cfg(feature = "cn")]
//pub const ARG_DONT_MATCH: Msg = "参数不匹配，请检查输入的参数";
//#[cfg(feature = "en")]
//pub const ARG_DONT_MATCH: Msg = "arguments don't match, check your inputs";
