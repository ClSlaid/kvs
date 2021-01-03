use std::process::exit;

use clap::{App, AppSettings, Arg, SubCommand};

use kvs::KvStore;

fn main() {
    let matches = App::new("kvs")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of a string key.")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the value of a string key.")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("command unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("command unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("command unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
