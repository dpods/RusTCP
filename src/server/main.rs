#[macro_use]
extern crate clap;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use clap::{Arg, App};

const HOST: &str = "0.0.0.0";
const PORT: &str = "8888";

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let matches = App::new(format!("{} {}", crate_name!(), "server"))
        .version(crate_version!())
        .about("TCP Server")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("The port the server will listen on")
            .default_value(PORT)
            .takes_value(true))
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let listener = TcpListener::bind(format!("{}:{}", HOST, port)).unwrap();

    println!("Server listening on port {}", port);

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    // close the socket server
    drop(listener);
}