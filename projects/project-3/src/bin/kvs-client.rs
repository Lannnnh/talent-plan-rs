use clap::{App, AppSettings, Arg, SubCommand};
// use std::io::{Read, Write};
// use std::net::TcpStream;
// use std::str::from_utf8;

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

    // match TcpStream::connect("localhost:3333") {
    //     Ok(mut stream) => {
    //         println!("Successfully connected to server in port 3333");

    //         let msg = b"Hello!";

    //         stream.write(msg).unwrap();
    //         println!("Sent Hello, awaiting reply...");

    //         let mut data = [0 as u8; 6]; // using 6 byte buffer
    //         match stream.read_exact(&mut data) {
    //             Ok(_) => {
    //                 if &data == msg {
    //                     println!("Reply is ok!");
    //                 } else {
    //                     let text = from_utf8(&data).unwrap();
    //                     println!("Unexpected reply: {}", text);
    //                 }
    //             }
    //             Err(e) => {
    //                 println!("Failed to receive data: {}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to connect: {}", e);
    //     }
    // }

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplement!");
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplement!");
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplement!");
        }
        _ => unreachable!(),
    }
}
