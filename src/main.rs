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
use board::{Board, Color, Loc};
use dice::Dice;
use player::Player;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("Hello, world!");
    let mut test_player = autoplayers::XMLTestPlayer {
        color: Color::Red, //This is meaningless
        name: "Lloyd".to_string(),
        stream: if args.len() > 1 {
            TcpStream::connect("172.217.6.110:80").expect("Could not connect")
        } else {
            TcpStream::connect("127.0.0.1:8000").expect("Could not connect to the server")
        },
    };
    // TODO make the test_player able to take in moves
    // Add GUI thing
    let test_dice: Dice = Dice { rolls: vec![3, 5] };
    let test_board: Board = Board::from(map!{
            Color::Red => [Loc::Spot { index: Board::get_entrance(&Color::Red)+1 },
                              Loc::Spot { index: Board::get_entrance(&Color::Red)+1 },
                               Loc::Spot { index: Board::get_entrance(&Color::Red) },
                              Loc::Nest,]
        });
    let move_vector = test_player.do_move(test_board, test_dice);
    println!("Moves that we have {:#?}", move_vector);
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
