use std::net::{Shutdown, TcpStream};
use std::io::prelude::*;


pub fn player_send() {
    {
        let mut stream = TcpStream::connect("127.0.0.1:8000")
        .expect("Couldn't connect to the server...");
        stream.write(b"TEST MESSAGE");
        let mut message = Vec::new();
        stream.read_to_end(&mut message);
        println!("{:#?}",&message);            
    }

}
