use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_stream(stream: TcpStream) {
    // TODO: do something
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:194").unwrap();

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
                // TODO: log the error
            }
        }
    }
}
