//! The IRC Client

// Use the `env` module from the static library
use std::env;

// declare a usage string in static memory.
const USAGE: &'static str = "Usage:
    irc-client <HOST>

Connect to an irc host.
";

fn main() {
    // parse arguments to retrieve host.
    let host = env::args().nth(1).expect("No host was specified!");

    // for now, just print the host and exit
    println!("HOST:\t{}", host);
}
