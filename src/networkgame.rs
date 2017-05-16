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
        let mut writer = BufWriter::new(&stream);
        let mut line = String::new();
        writer.write_all(serialize::xml_start_game(&Color::Red).as_bytes());
        if let Err(e) = writer.flush() {
            panic!("AHHHHH");
        }
        let mut response = String::new();
        let mut reader = BufReader::new(&stream);
        for line in reader.by_ref().lines() {
            let mut response = line.unwrap();
            println!("Server received {}", response);
        }
        //let mut response = String::new();
        //reader.read_line(&mut response);
        //println!("Server received {}", response);
        //println!("Do I reach here");
        //writer.write(b"writing");
        //writer.write(b"writing");
        //writer.write(b"writing");

        
        
        
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
