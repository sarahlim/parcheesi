use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;


// Mapping between colors and TCP Streams will be needed

pub fn start_server() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    fn handle_client(mut stream: TcpStream) {
        let mut reader = BufReader::new(stream);

        for line in reader.by_ref().lines() {
            let x = line.unwrap();
            if x == "" {
                break;
            } else {
                println!("{}", &x);
            }
        }
    }

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                panic!("No connection");
            }
        }
    }

}
