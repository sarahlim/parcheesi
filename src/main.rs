#[macro_use]
mod macros;

extern crate quick_xml;

mod board;
mod game;
mod dice;
mod constants;
mod player;
mod autoplayers;
mod serialize;
mod deserialize;
mod networkplayer;
mod networkgame;

use std::net::TcpStream;
use networkplayer::NetworkPlayer;
use board::Color;

fn main() {
    println!("Hello, world!");
    let mut test_player = autoplayers::XMLTestPlayer {
        color: Color::Red,
        name: "Johann".to_string(),
        stream: TcpStream::connect("127.0.0.1:8000").expect("Could not connect to the server"),
    };
    let response: String = test_player.receive();
    println!("{}", response);
    let test_string: String = "<name>".to_string() + &test_player.name +
                              "</name> \n";
    test_player.send(test_string);
    loop {
        let moves = test_player.receive();
        println!("{}", moves);
    }
}
