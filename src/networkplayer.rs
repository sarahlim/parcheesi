use std::net::{Shutdown, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader,BufWriter};
use super::player::Player;



pub fn player_send(xml: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8000")
        .expect("Couldn't connect to the server...");
<<<<<<< HEAD
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    for line in reader.lines() {
        println!("Player got {}", line.unwrap());
        
=======
        stream.write(b"TEST MESSAGE");
        let mut message = Vec::new();
        stream.read_to_end(&mut message);
        println!("{:#?}", &message);
>>>>>>> a3e9d9e28fb6ab51ca06790472d8b4a7d29ce2e4
    }
    println!("Out of read loop");

}

pub trait NetworkPlayer : Player {
    fn connect(&mut self) -> ();// When instantiating players, just call connect to avoid immutable stuff?
    fn send(&mut self, msg: String) -> ();
    fn receive(&mut self) -> String;
}
// Where to split up the stream?
