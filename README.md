# RusTCP
A Basic TCP server written in Rust

This project is meant to provide a starting point for building a TCP server and client that can be compiled into separate binaries. 

## Compiling
Use Cargo to compile both the client and the server binaries
```
cargo build
```
The binaries can be found at 
```
target/debug/client
target/debug/server
```

## Running
Start the server first
```
./target/debug/server
Server listening on port 8888
```

The connect using the client
```
./target/debug/client
Successfully connected to server at 0.0.0.0:8888
Sent Hello, awaiting reply...
Reply is ok!
Terminated.
```

## Options 
Use the `--help` flag to list out the available options on both the client and the server
Client:
```
./target/debug/client --help
logger client 0.1.0
TCP Server Client

USAGE:
    client [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <HOST>    The IP of the host server [default: 0.0.0.0]
    -p, --port <PORT>    The port the server is listening on [default: 8888]
```

Server:
```
./target/debug/server --help
logger server 0.1.0
TCP Server

USAGE:
    server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <PORT>    The port the server will listen on [default: 8888]
```
