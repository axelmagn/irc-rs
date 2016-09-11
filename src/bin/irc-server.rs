#[macro_use]
extern crate log;
extern crate env_logger;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::BufReader;


fn handle_stream(stream: TcpStream) {
    // log the connection
    let name = format!("{}", stream.peer_addr().unwrap());
    info!("CONNECTED\t{}", name);

    // log each line
    for line in BufReader::new(stream).lines() {
        line.map(|line| info!("{}:\t{}", name, line))
            .unwrap_or_else(|e| error!("ERROR ({})\t{}", name, e));
    }
}

fn main() {
    // set up logging
    env_logger::init().unwrap();

    // start listening on tcp
    let listen_host = "127.0.0.1:3000";
    let listener = TcpListener::bind(listen_host).unwrap();
    info!("LISTENING\t{}", listen_host);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // create handler thread
                thread::spawn(move|| {
                    // handle the stream somehow
                    handle_stream(stream);
                });
            },
            Err(e) => {
                error!("ERROR (server)\t{:?}", e);
            }
        }
    }
}
