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

#[cfg(feature = "cn")]
pub const ARG_DUPLICATE_DEFINITION: Msg = "参数重复定义";
#[cfg(feature = "en")]
pub const ARG_DUPLICATE_DEFINITION: Msg = "arguments duplicate definition";

#[cfg(feature = "cn")]
pub const ORDER_ERROR: Msg = "参数顺序错误。所有【必选】必须出现在所有【可选】之前，只有最后一个参数能为【不定数量类型】";
#[cfg(feature = "en")]
pub const ORDER_ERROR: Msg = "The parameter order is incorrect. All [required] must appear before all [optional], only the last parameter can be [indefinite quantity type]";

#[cfg(feature = "cn")]
pub const ENTRY_ONLY_MAIN: Msg = "#[init]只能被用于main函数";
#[cfg(feature = "en")]
pub const ENTRY_ONLY_MAIN: Msg = "#[init] can be used for fn main only";

#[cfg(feature = "cn")]
pub const DONT_MATCH: Msg = "#[command]定义的指令名必须与其对应的处理函数名一样";
#[cfg(feature = "en")]
pub const DONT_MATCH: Msg = "the name of sub-command defined by #[command] should be same as the name of it's function";

#[cfg(feature = "cn")]
pub const OPT_DUPLICATE_DEFINITION: Msg = "选项重复定义";
#[cfg(feature = "en")]
pub const OPT_DUPLICATE_DEFINITION: Msg = "options duplicate definition";

#[cfg(feature = "cn")]
pub const INVALID_ARGUMENTS: Msg = "非法参数，无法处理。请提交issue并附带你的代码";
#[cfg(feature = "en")]
pub const INVALID_ARGUMENTS: Msg = "invalid argument, please open issue with your code";

#[cfg(feature = "cn")]
pub const NO_SUB_CMD_NAMES_MAIN: Msg = "[main]不能用作子命令的名字";
#[cfg(feature = "en")]
pub const NO_SUB_CMD_NAMES_MAIN: Msg = "sub-command can't be named as [main]";