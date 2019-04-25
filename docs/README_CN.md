# 为什么会有这个项目？

长久以来，在`Rust`中开发CLI程序并不是一件简单的事儿。 
由于`Rust`是一门静态语言，编译器需要在编译时知道所有的细节，这与CLI程序的动态性冲突了。
社区提供了很多的解决方法，诚然，它们都很优秀。但是它们都太复杂了。
人们往往并不希望在这个地方花费太多的时间，而是把大量的时间留给程序的功能性开发。

受到[commander.js](https://github.com/tj/commander.js) & [rocket.rs](https://rocket.rs)的启发，这个crate就此诞生了。

# 特性
+ API友好
+ 使用简单
+ 近似动态语言的支持
+ 低性能损耗
+ 自动实现`--version` & `--help`
+ 自动调用对应命令

# 限制

如果你想使用这个crate，你必须得保证你遵守以下的一些规则
- 使用`Rust 2018`（需要完整的proc macro支持）
- 使用`cargo`（`cargo`会在编译时注入一些环境变量，我们会使用到其中的一些）
- 熟悉`Rust`（因为它是为`Rust`而开发的）

作为参考，我的版本信息如下所示：
+ `cargo`: cargo 1.35.0-nightly (95b45eca1 2019-03-06)
+ `rustc`: rustc 1.35.0-nightly (e68bf8ae1 2019-03-11)
+ `Linux kernal`: 4.15.0-47-generic
+ `Ubuntu`: 16.04

# 用法

#### 下载`commander-rust`

有两种方法：从`Github`或者`crates.io`安装。 
两者的区别在于，`Github`保证是最新的，但是不保证其稳定性。 后者是稳定的，但不一定是最新的。

##### 从`Github`下载

```toml
[dependencies.commander_rust]
git = "https://github.com/MSDimos/commander_rust"
branch = "master"
```

#### 从`crates.io`下载

```toml
[dependencies]
commander_rust = "^1.0.0" # 指定其他任意你需要的版本
```

#### 使用它

提供了一个简单但是完整的例子，你可以通过它了解到所有。 例子中所呈现的一切，就是你所会用到的一切。相当简单！

```rust

// 必须，因为我们使用了`run！（）`，它是proc_macro宏
#![feature(proc_macro_hygiene)]

// 你只需要导入这五个东西
use commander_rust::{ Cli, command, option, entry, run };


// 什么是option？什么是command？
// 参考`commander.js`和`commander-rust`的文档。
// 注意，函数的参数类型并不是固定的，任意实现了`From<Raw>`的类型都可以。
// `Cli`并不是必须的参数，你可以省略它。
#[option(-s, --format <format>, "format output")]
#[option(-r, --recursive, "recursively")]
#[command(rmdir <dir> [otherDirs...], "remove files and directories")]
fn rmdir(dir: String, other_dirs: Option<Vec<String>>, cli: Cli) {
    // 如果编译器无法给出正确有效的错误提示
    // 考虑使用一个_rmdir函数去包裹以下所有代码，
    // 然后在这里调用它即可。
    // 参考这个issue：`https://github.com/dtolnay/syn/issues/622`
    let format = cli.get_or("format", String::new("%s"));
    
    if cli.has("recursive") {
        let quite: bool = cli.get_or("quite", false);
        
        if quite {
            // silently delete all files
            // just like `rm -rf /`
        } else {
            // tell the world I'm going to delete the files
        }
    } else {
        // drink a cup of coffee, relax.
    }
}

// 定义在这里的options是公共的，定义在`#[command]`之上的则是私有的。
#[option(-q, --quite <quite_or_not>, "dont display anything")]
#[entry]
fn main() {
     // 调用run！()，开始运行
     let app = run!();
     // 打印app功能同输入--help一样
     println!("app is {:#?}", app);
}
```

#### 试试

尝试调用一下`[cli的名字] --help`。

# 版本号&描述&cli名字？

他们都来自于你的项目的`Cargo.toml`里面。

```toml
# part of Cargo.toml
[package]
name = "example-test"
version = "0.1.0"
description = "Using for test"
```

# 错误

我无法保证它在所有情况下都能工作良好，实际上，由于一些客观原因的限制，测试并不能良好的进行。
如果你发现任何BUG，请向我反馈。感谢。

# 完整的例子

我提供了一个完整的例子用于展示`commander-rust`如何使用。
这个例子名为`hash`，他用于计算文件或字符串的MD5摘要。
查看`./examples/`以获得更多信息。

# 主页

我为其开发了一个主页，它基于`React`而开发。

# 规则

开发时，你需要遵守一些规则，才能正常工作：
1. 所有的`#[option]`都必须定义在`#[command]`或者`#[entry]`的上方！否则不能工作！
2. 不要重复定义`#[option]`，短命名和长命名都应该保持唯一！作为补偿，你可以定义相同的公共option和私有option。私有option的权重更高。
3. 私有option仅对对应的子命令可见，公共option对所有的子命令都有效
4. 不支持0子命令程序。

# 警告

这个crate在`Ubuntu 16.04`下工作良好，考虑到系统的差异性，我无法保证所有的系统都能正常工作。
如果你发现了问题，请向我反馈。

# 贡献

任何有用的贡献都是欢迎的！让我们一起完善`Rust`的生态！

# 协议

GPL-3.0协议覆盖。