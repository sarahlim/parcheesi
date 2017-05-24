use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write, BufReader, BufWriter, BufRead};
use std::thread;
use super::serialize;
use super::board::Color;


// Mapping between colors and TCP Streams will be needed

pub fn start_server() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    fn handle_client(mut stream: TcpStream) {
        println!("Client Connected");

        let mut writer = BufWriter::new(&stream);
        writer
            .write_all("Red\n".as_bytes())
            .expect("could not write");
        writer
            .flush()
            .expect("could not flush");

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .expect("could not read");
        println!("Server received {}", response);

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
