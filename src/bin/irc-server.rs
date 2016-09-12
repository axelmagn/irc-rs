#[macro_use]
extern crate log;
extern crate env_logger;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use docopt::Docopt;
use std::collections::HashMap;
use std::fmt;
use std::io::{BufReader, LineWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;


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

#[derive(Clone)]
struct Message {
    name: String,
    text: String,
}

#[derive(Clone)]
enum Event {
    Msg(Message),
    Quit,
}

#[derive(Clone)]
struct User {
    name: String,
    sender: Sender<Event>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{}: {}", self.name, self.text)
    }
}

type SubscribersList = Arc<Mutex<HashMap<Uuid, User>>>;

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

    // create a list to hold the subscribers.
    // this is the list of people to send stuff to.
    let subscribers: SubscribersList = Arc::new(Mutex::new(HashMap::new()));

    // create a thread that will pass messages along to subscribers
    let (chan_tx, chan_rx) = channel::<Event>();
    {
        let subscribers = subscribers.clone();
        thread::spawn(move|| {
            for event in chan_rx {
                // let event = event.unwrap();
                let subscribers = subscribers.lock().unwrap();
                for sub in subscribers.values() { 
                    sub.sender.send(event.clone()).unwrap(); 
                }
            }
        })
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // clone channel resources
                let (subscribers, chan_tx) = (subscribers.clone(), chan_tx.clone());
                // create handler thread
                thread::spawn(move|| {
                    // handle the stream 
                    handle_stream(stream, subscribers, chan_tx);
                });
            },
            Err(e) => {
                error!("ERROR (server)\t{}", e);
            }
        }
    }
}


/// Client Stream Handler
fn handle_stream(stream: TcpStream, subscribers: SubscribersList, chan_tx: Sender<Event>) {
    // log the connection
    let name = format!("{}", stream.peer_addr().unwrap());
    info!("CONNECTED\t{}", name);

    // get a channel for the subscriber
    let (sub_tx, sub_rx) = channel::<Event>();
    // generate a unique key to keep track of the user
    let key = Uuid::new_v4();
    // generate a user structure for subscribers
    let user = User{
        name: name.clone(),
        sender: sub_tx.clone(),
    };

    // add the channel to the list
    {
        let mut subscribers = subscribers.lock().unwrap();
        subscribers.insert(key, user);
    };

    let stream_writer = stream.try_clone().unwrap();

    // spawn a listener that passes messages along to client
    let sub_writer =  thread::spawn(move|| {
        let mut writer = LineWriter::new(stream_writer);
        for event in sub_rx {
            match event {
                Event::Msg(m) => {
                    write!(writer, "{}\n", m).unwrap();
                },
                Event::Quit => { break; },
            }
        }
    });

    // pass each line along as a mesage
    BufReader::new(stream)
              .lines()
              .map(|line| line.unwrap())
              .map(|line| Event::Msg(Message{ name: name.clone(), text: line }))
              .map(|msg| chan_tx.send(msg))
              .last();


    // deregister and shutdown listener
    {
        let mut subscribers = subscribers.lock().unwrap();
        subscribers.remove(&key);
    }
    sub_tx.send(Event::Quit).unwrap();
    sub_writer.join().unwrap();


    info!("DISCONNECTED\t{}", name);
}
