#[macro_use]
extern crate log;
extern crate env_logger;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;


const USAGE: &'static str = "
Usage: irc-server [options] 

Starts an IRC server. If no socket address is specified, it will listen on 0.0.0.0:3000.

Options:
    -h --help               Show this message.
    -l --listen=<addr>      Socket address to listen on [default: 0.0.0.0:3000].
";

/// Program Arguments
#[derive(RustcDecodable, Debug)]
struct Args {
    flag_listen: String,
}


fn main() {
    // set up logging
    env_logger::init().unwrap();

    // parse args
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    debug!("ARGS\t{:?}", args);

    // start listening on tcp
    let listen_host: String = args.flag_listen;
    let listener = TcpListener::bind(listen_host.as_str())
                               .expect("Invalid TCP Host");
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


/// Client Stream Handler
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
