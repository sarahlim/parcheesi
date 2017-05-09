#![allow(dead_code, unused_variables, unused_imports, unused_mut)]


use std::collections::BTreeMap;
use super::player::Player;
use super::autoplayers::XMLTestPlayer;
use super::dice::Dice;
use super::board::{Color, Board, Pawn, Loc, MoveResult};
use super::game::{Move,MoveType};
use super::constants::*;
use super::quick_xml::reader::Reader;
use super::quick_xml::events::Event;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// This function will receive a string about a new game starting. It will 
pub fn parse_start_game(request: String) -> Color {
    let mut reader = Reader::from_str(&request);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"start-game" => println!("start game"),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break,
            Err(e) => panic!(),
            _ => (),
        }
        buf.clear();
    }
    match txt.pop().unwrap().as_ref() {
        "Red" => Color::Red,
        "Blue" => Color::Blue,
        "Yellow" => Color::Yellow,
        "Green" => Color::Green,
        _ => panic!("That's not a color"),
    }
}

