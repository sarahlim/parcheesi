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

use std::{thread,time};
use std::net::{TcpStream};
use networkplayer::NetworkPlayer;

fn main() {
    println!("Hello, world!");
    //    deserialize::parse_start_game(parse::xml_start_game(&board::Color::Red));
    thread::spawn(move || { networkgame::start_server(); });
<<<<<<< HEAD
    //thread::spawn(move || { networkplayer::player_send("Test Player 1".to_string()); });  
    let mut test = autoplayers::XMLTestPlayer {
        color: board::Color::Red,
        name: "JOHN".to_string(),
        stream: TcpStream::connect("127.0.0.1:8000").expect("Could not connect"),            
    };
    let string = NetworkPlayer::receive(&mut test);
    println!("I received {}",string);
    NetworkPlayer::send(&mut test, "Me".to_string());
    

    loop {

    }
        
=======
    networkplayer::player_send();
    networkplayer::player_send();

    loop {}
>>>>>>> a3e9d9e28fb6ab51ca06790472d8b4a7d29ce2e4
}

