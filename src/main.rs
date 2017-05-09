#[macro_use]
mod macros;


extern crate quick_xml;


mod board;
mod game;
mod dice;
mod constants;
mod player;
mod autoplayers;
mod parse;
mod deserialize;



fn main() {
    println!("Hello, world!");
    deserialize::parse_start_game(parse::xml_start_game(&board::Color::Red));
}



             
