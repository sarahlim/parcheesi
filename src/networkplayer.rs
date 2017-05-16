use std::net::{Shutdown, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader,BufWriter};
use super::player::Player;



pub fn player_send(xml: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8000")
        .expect("Couldn't connect to the server...");
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    for line in reader.lines() {
        println!("Player got {}", line.unwrap());
        
    }
    println!("Out of read loop");

}

pub trait NetworkPlayer : Player {
    fn connect(&mut self) -> ();// When instantiating players, just call connect to avoid immutable stuff?
    fn send(&mut self, msg: String) -> ();
    fn receive(&mut self) -> String;
}
// Where to split up the stream?
