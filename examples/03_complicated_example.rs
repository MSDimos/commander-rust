// a package manager simulation
#![feature(proc_macro_hygiene)]

use commander_rust::{sub_command, default_options, command, option, execute, Arg, Opts, Command};
use commander_rust::traits::FromArg;
use commander_rust_core::converters::GlobalOpts;

#[derive(Debug)]
struct Version(u8, u8, u8);

#[derive(Debug)]
struct Pkg {
    name: String,
    version: Version,
}

impl<'a> FromArg<'a> for Pkg {
    type Error = ();

    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        let splits: Vec<&str> = arg.split('=').collect();

        if splits.len() != 2 {
            Err(())
        } else {
            let name = splits[0];
            let vers: Vec<&str> = splits[1].split('.').collect();

            if vers.len() != 3 {
                Err(())
            } else {
                let mut vs = [0, 0, 0];

                for (idx, ver) in vers.into_iter().enumerate() {
                    if let Ok(v) = ver.parse::<u8>() {
                        vs[idx] = v;
                    } else {
                        return Err(());
                    }
                }

                Ok(Pkg {
                    name: name.to_string(),
                    version: Version(vs[0], vs[1], vs[2]),
                })
            }
        }
    }
}

#[default_options]
#[option(-f, --force, "force to install even if this package has already installed")]
#[option(-g, --global, "install as a global package")]
#[sub_command(install <pkg>, "install a package")]
fn install_fn(pkg: Result<Pkg, ()>, opts: Opts, global_opts: GlobalOpts) {
    if let Ok(pkg) = pkg {
        let node_pkg = format!("{}@{}.{}.{}", pkg.name, pkg.version.0, pkg.version.1, pkg.version.2);
        println!("try to install {}", node_pkg);
        println!("installing...");
        let mut cmd = std::process::Command::new("npm");

        cmd.arg("install");
        cmd.arg(&node_pkg);

        if global_opts.contains_key("verbose") {
            cmd.arg("--verbose");
        }

        if opts.contains_key("force") {
            cmd.arg("--force");
        }

        if opts.contains_key("global") {
            cmd.arg("--global");
        }

        if cmd.status().is_ok() {
            println!("install completed.");
        } else {
            eprintln!("fail to install.");
        }
    } else {
        println!("can't parse the package");
    }
}


#[default_options]
#[sub_command(update [target], "update a package or a project")]
fn update_fn(target: Option<String>, global_opts: GlobalOpts) {
    let mut cmd = std::process::Command::new("npm");

    cmd.arg("install");

    if let Some(target) = target {
        println!("update package {}.", target);
        cmd.arg(target);
    } else {
        println!("update entire project.");
    }

    if global_opts.contains_key("verbose") {
        cmd.arg("--verbose");
    }

    if cmd.status().is_ok() {
        println!("update completed.");
    } else {
        eprintln!("fail to update.");
    }
}

#[default_options]
#[option(-g, --global, "remove global packages")]
#[option(--g)]
#[sub_command(uninstall <..pkgs>, "uninstall packages")]
fn uninstall_fn(pkgs: Vec<String>, opts: Opts, global_opts: GlobalOpts) {
    let mut status = vec![false; pkgs.len()];

    for pkg in pkgs.iter() {
        let mut cmd = std::process::Command::new("npm");

        cmd.arg("uninstall");
        cmd.arg(pkg);

        if global_opts.contains_key("verbose") {
            cmd.arg("--verbose");
        }

        if opts.contains_key("global") {
            cmd.arg("--global");
        }

        status.push(cmd.status().is_ok());
    }

    for (idx, pkg) in pkgs.into_iter().enumerate() {
        if status[idx] {
            println!("uninstall {} successfully", pkg);
        } else {
            println!("uninstall {} unsuccessfully", pkg);
        }
    }
}

#[default_options]
#[option(--verbose, "output verbose messages on internal operations")]
// if you doesn't offer version (e.g., #[command("0.0.1-pre-alpha", npms, "node manager simulation")])
// commander_rust will use environment variable `CARGO_PKG_VERSION` (i.e., std::env!("CARGO_PKG_VERSION")) as version
#[option(-g, --global)]
#[command(npms, "node package manager simulation")]
fn npms_fn(cmd: &Command) {
    // show help information
    // if you want to show help or version information, there are three `api` you can use
    // 1. cmd.println() -- print help information of command
    // 2. cmd.println_sub("sub_name") -- print help information of the specified sub-command
    // 3. cmd.println_version() -- print version information
    cmd.println()
}

fn main() {
    execute!(npms_fn, [install_fn, update_fn, uninstall_fn]);
}