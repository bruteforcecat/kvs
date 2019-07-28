extern crate clap;
use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("KVS")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("Value")
                        .help("A string value")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of a string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the value of a string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("Value")
                        .help("A string value")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", _) => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        ("get", _) => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        ("rm", _) => {
            eprintln!("unimplemented");
            process::exit(1)
        }
        _ => unreachable!(),
    }
}
