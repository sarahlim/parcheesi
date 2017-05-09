use std::net::{Shutdown,TcpStream};
use std::io::prelude::*;


pub fn player_send() {
    {
    let mut stream = TcpStream::connect("127.0.0.1:8000")
        .expect("Couldn't connect to the server...");
        stream.write(b"slut");
    }
    
}
