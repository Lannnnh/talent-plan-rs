use clap::{App, AppSettings, Arg, SubCommand};
use kvs::kv::KvStore;
// use std::fs;
use std::env;
use std::path::Path;
use std::process::exit;
use tempfile::TempDir;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let current_dir = env::current_dir().unwrap();
    let mut kvstore = KvStore::open(&current_dir).unwrap();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap().to_string();
            let value = _matches.value_of("VALUE").unwrap().to_string();

            kvstore.set(&key, &value).unwrap();
        }
        ("get", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap().to_string();

            match kvstore.get(&key) {
                Ok(value) => match value {
                    Some(v) => println!("{}", v),
                    None => println!("Key not found"),
                },
                Err(_) => {
                    println!("Key not found")
                }
            }
        }
        ("rm", Some(_matches)) => {
            if let Some(k) = _matches.value_of("KEY") {
                let key = k.to_string();

                match kvstore.remove(&key) {
                    Err(_) => {
                        println!("Key not found");
                        exit(1);
                    }
                    _ => {}
                }
            }
        }
        _ => unreachable!(),
    }
}
