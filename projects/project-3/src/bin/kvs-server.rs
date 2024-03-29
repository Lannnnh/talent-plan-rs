// use std::io::{Read, Write};
// use std::net::{Shutdown, TcpListener, TcpStream};
// use std::thread;
use clap::{App, AppSettings};
use log::LevelFilter;
use log::{error, info, warn};

// fn handle_client(mut stream: TcpStream) {
//     let mut data = [0 as u8; 50]; // using 50 byte buffer
//     while match stream.read(&mut data) {
//         Ok(size) => {
//             // echo everything!
//             stream.write(&data[0..size]).unwrap();
//             true
//         }
//         Err(_) => {
//             println!(
//                 "An error occurred, terminating connection with {}",
//                 stream.peer_addr().unwrap()
//             );
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }

fn main() {
    // let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // // accept connections and process them, spawning a new thread for each one
    // println!("Server listening on port 3333");
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             println!("New connection: {}", stream.peer_addr().unwrap());
    //             thread::spawn(move || {
    //                 // connection succeeded
    //                 handle_client(stream)
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             /* connection failed */
    //         }
    //     }
    // }
    // // close the socket server
    // drop(listener);

    // env_logger 默认输出到标准错误 stderr
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .args_from_usage(
            "-a, --addr=[ip:port] 'kv-server listening address'
            -e, --engine=[type] 'kv-server engine type'",
        )
        .get_matches();

    let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
    let engine = matches.value_of("engine").unwrap_or("kvs");

    info!("kv-server: {}", env!("CARGO_PKG_VERSION"));
    info!("listening address: {}", addr);
    info!("using engine: {}", engine);
}
