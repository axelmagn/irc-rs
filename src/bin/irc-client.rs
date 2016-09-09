//! The IRC Client

// Use the `env` module from the static library
use std::env;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;

// declare a usage string in static memory.
const USAGE: &'static str = "Usage:
    irc-client <HOST>

Connect to an irc host.
";

const PROMPT: &'static str = "> ";

fn main() {
    // parse arguments to retrieve host.
    let host = env::args().nth(1).expect(USAGE);
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
