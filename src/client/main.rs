#[macro_use]
extern crate clap;

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use clap::{Arg, App};

const HOST: &str = "0.0.0.0";
const PORT: &str = "8888";

fn main() {
    let matches = App::new(format!("{} {}", crate_name!(), "client"))
        .version(crate_version!())
        .about("TCP Server Client")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("The IP of the host server")
            .default_value(HOST)
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("The port the server is listening on")
            .default_value(PORT)
            .takes_value(true))
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();

    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}:{}", host, port);

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}