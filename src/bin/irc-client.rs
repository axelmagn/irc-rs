//! The IRC Client
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate docopt;
extern crate rustc_serialize;

// Use the `env` module from the static library
use docopt::Docopt;
use std::io::{BufReader, LineWriter};
use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use std::net::Shutdown;
use std::thread;


// declare a usage string in static memory.
const USAGE: &'static str = "Usage:
    irc-client [options] <host>

Connect to an irc host.

Options:
    -h --help       Show this help message
";

/// Program Arguments
#[derive(RustcDecodable, Debug)]
struct Args {
    arg_host: String,
}


// const PROMPT: &'static str = "> ";

fn main() {
    // set up logging
    env_logger::init().unwrap();

    // parse args
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    // show host
    let host = args.arg_host.as_str();
    info!("HOST:\t{}", host);

    // connect to server
    let stream = TcpStream::connect(host)
                           .expect("Could not connect to host.");

    let reader = stream.try_clone().expect("Could not clone stream");
    let writer = stream.try_clone().expect("Could not clone stream");

    // handle reader
    let reader_child = thread::spawn(move|| {
        // wrap raw stream in a buffered reader
        BufReader::new(reader)
                  // break into lines
                  .lines()
                  // print each line, or log an error if it fails
                  .map(|line| match line {
                      Ok(l)     => { println!("{}", l); },
                      Err(e)    => { error!("ERROR\t{}", e); },
                  })
                  // keep reading from the stream until it is shutdown
                  .last();
    });

    // handle writer
    let writer_child = thread::spawn(move|| {
        // wrap raw stream in a buffered writer that flushes on each newline
        let mut writer = LineWriter::new(writer);
        io::stdin()
            // read bytes one by one from stdin
           .bytes()
           // feed each byte into the stream
           .map(|b| b.and_then(|b| writer.write(&[b])))
           // log any write errors
           .map(|r| match r {
               Ok(_)    => () ,
               Err(e)   => { error!("ERROR\t{}", e); }
           })
           // read until the end of stdin
           .last();
        // shutdown the stream, which will also signals the end of the reader iterator
        stream
            .shutdown(Shutdown::Both)
            .expect("Could not shutdown stream");
    });

    // wait to join child threads
    writer_child.join().unwrap();
    reader_child.join().unwrap();
}
