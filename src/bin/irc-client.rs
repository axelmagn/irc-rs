//! The IRC Client
extern crate docopt;
extern crate rustc_serialize;

// Use the `env` module from the static library
use docopt::Docopt;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;

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


const PROMPT: &'static str = "> ";

fn main() {
    // parse arguments to retrieve host.

    // parse args
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    let host = args.arg_host;
    println!("HOST:\t{}", host);

    // echo repl
    let stdin = BufReader::new(io::stdin()).lines();
    let mut stdout = io::stdout();
    print!("{}", PROMPT);
    stdout.flush().unwrap();
    for line in stdin {
        let line = line.unwrap();
        println!("{}", line);
        print!("{}", PROMPT);
        stdout.flush().unwrap();
    }
}
