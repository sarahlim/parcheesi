use std::net::{Shutdown, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use super::player::Player;


pub trait NetworkPlayer: Player {
    fn connect(&mut self) -> (); // When instantiating players, just call connect to avoid immutable stuff?
    fn send(&mut self, msg: String) -> ();
    fn receive(&mut self) -> String;
}
// Where to split up the stream?
