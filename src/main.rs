mod models;
mod fcastmanager;
mod command;
mod state;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use serde_json::Value;

use models::{VolumeUpdateMessage, PlaybackUpdateMessage};
use fcastmanager::main_handler;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:46899").unwrap(); //TODO: config
    println!("Server listening on port 46899"); //TODO: config
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    main_handler(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
