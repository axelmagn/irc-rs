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
    //
    // 1. args is not actually a vector; it's an Iterator.  Much of the rust standard library prefers
    //    Iterator or ExactSizeIterator because they can be easily collected into whatever data
    //    structure is preferable to the programmer.
    //
    // 2. Even though rust is statically typed, we usually don't have to specify a type when
    //    defining variables.  This is because rust provides "type inference", where it will try to
    //    derive the type at compile time, and only bother you if it can't figure out what type a
    //    variable should be.  There are a few things that are *not* type inferred though: function
    //    signatures and static variables.  This is a conscious design decision made in order to
    //    support more maintainable code.
    let args = env::args();
    let host: String = args.nth(1);

    // for now, just print the host and exit
    println!("HOST:\t{}", host);
}
