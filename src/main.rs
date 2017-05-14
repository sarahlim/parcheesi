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

use std::thread;

fn main() {
    println!("Hello, world!");
    //    deserialize::parse_start_game(parse::xml_start_game(&board::Color::Red));
    thread::spawn(move || { networkgame::start_server(); });
    networkplayer::player_send();
    networkplayer::player_send();
    loop {


    }
}
