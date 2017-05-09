use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

pub fn start_server() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

        fn handle_client(mut stream: TcpStream) {
            let mut line = String::new();
            stream.read_to_string(&mut line);
        }

    for stream in listener.incoming() {
        
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                       handle_client(stream)
                    });
                }
                Err(e) => { panic!("No connection"); }
            }
        }
}
