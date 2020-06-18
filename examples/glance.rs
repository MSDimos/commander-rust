#![feature(proc_macro_hygiene)]
#![allow(dead_code)]
#![allow(unused_variables)]
use commander_rust::{ command, sub_command, option, default_options, execute };
use commander_rust::traits::{ FromArg };
use commander_rust::{ Arg, Opts, GlobalOpts, Mixed };
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::process::exit;

struct Address {
    ipv4: Ipv4Addr,
    port: u16,
}

impl<'a> FromArg<'a> for Address {
    type Error = ();

    fn from_arg(arg: &'a Arg) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = arg.split_terminator(':').collect();

        if parts.len() != 2 { Err(()) } else {
            let ipv4 = if let Ok(ipv4) = Ipv4Addr::from_str(parts[0]) { ipv4 } else {
                return Err(());
            };

            let port = if let Ok(port) = u16::from_str(parts[1]) { port } else {
                return Err(());
            };

            Ok(Address {
                ipv4,
                port,
            })
        }
    }
}

#[default_options]
#[option(--https, "use https instead of http")]
#[sub_command(connect <address>, "connect to the address")]
fn connect(address: Option<Address>, opts: Opts, global_opts: GlobalOpts) {
    if let Some(address) = address {
        // try to get global option named `proxy`
        if let Some(proxy) = global_opts.get("proxy") {
            // try to get named argument `proxy_address` from global option `proxy`
            if let Some(Mixed::Single(arg)) = proxy.get("proxy_address") {
                // convert`proxy_address` to value whose type is `Address`
                if let Ok(proxy_address) = Address::from_arg(arg) {
                    // use a proxy to connect to the address
                    if opts.contains_key("https") {
                        // use https
                    } else {
                        // use http
                    }
                } else {
                    eprintln!("invalid proxy address");
                    exit(1);
                }
            }
        }
    } else {
        eprintln!("invalid address");
        exit(1);
    }
}

#[default_options]
#[sub_command(disconnect <address>, "disconnect the connection")]
fn disconnect(address: Option<Address>) {
    if let Some(address) = address {
        // disconnect
    } else {
        eprintln!("invalid address");
        exit(1);
    }
}

#[default_options]
#[option(--proxy <proxy_address>, "use proxy to connect")]
#[command(net, "network tool")]
fn net() { /* .. */}

fn main() {
    execute!(net, [connect, disconnect]);
}