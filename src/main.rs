#[macro_use]
mod macros;

extern crate quick_xml;

mod board;
mod game;
mod dice;
mod constants;
mod player;
mod gametree;
mod autoplayers;
mod serialize;
mod deserialize;
mod networkplayer;
mod networkgame;

use std::net::TcpStream;
use networkplayer::NetworkPlayer;
use board::{Board,Color};
use dice::Dice;
use player::Player;

fn main() {
    println!("Hello, world!");
    let mut test_player = autoplayers::XMLTestPlayer {
        color: Color::Blue, //This is meaningless
        name: "Lloyd".to_string(),
        stream: TcpStream::connect("127.0.0.1:8000").expect("Could not connect to the server"),
    };
    test_player.receive();
    loop {
        test_player.receive();
    }
    //let name: String = test_player.start_game();
    // Probably should change this just to be void
    //let color: String = test_player.receive();
    //println!("Player got {}", color);
    //let assigned_color: Color = deserialize::deserialize_start_game(color);
    //test_player.color = assigned_color;
    //println!("I am {}", test_player.color);
    //test_player.send("<name>Yo YO yo it is I </name>\n".to_string());
    //println!("Do we get here");
    //loop {
      //  let moves = test_player.receive();
      //  println!("Player received: {}", moves);
    //}
}
