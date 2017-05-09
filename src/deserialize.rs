#![allow(dead_code, unused_variables, unused_imports, unused_mut)]


use std::collections::BTreeMap;
use super::player::Player;
use super::autoplayers::XMLTestPlayer;
use super::dice::Dice;
use super::board::{Color, Board, Pawn, Loc, MoveResult};
use super::game::{Move, MoveType};
use super::constants::*;
use super::parse;
use super::quick_xml::reader::Reader;
use super::quick_xml::events::Event;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// This function will receive a string about a new game starting. It will
pub fn deserialize_start_game(request: String) -> Color {
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
            }
            Ok(Event::Text(e)) => {
                txt.push(e.unescape_and_decode(&reader)
                             .unwrap())
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!(),
            _ => (),
        }
        buf.clear();
    }
    match txt.pop()
              .unwrap()
              .as_ref() {
        "Red" => Color::Red,
        "Blue" => Color::Blue,
        "Yellow" => Color::Yellow,
        "Green" => Color::Green,
        _ => panic!("That's not a color"),
    }
}

pub fn deserialize_moves(xml: String) -> Vec<Move> {
    let string_vec: Vec<String> = move_string_to_vec_string(xml);
    let result = vec_string_to_vec_move(string_vec);
    println!("{:#?}", result);
    result
}

/// This function will take in an xml string and return
/// a vector of strings corresponding to the moves.
pub fn move_string_to_vec_string(xml: String) -> Vec<String> {
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    // let mut pos_vec = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"moves" => println!("test"),
                    b"enter-piece" => {
                        txt.push(e.unescape_and_decode(&reader)
                                     .unwrap())
                    }
                    b"move-piece-home" => {
                        txt.push(e.unescape_and_decode(&reader)
                                     .unwrap())
                    }
                    b"move-piece-main" => {
                        txt.push(e.unescape_and_decode(&reader)
                                     .unwrap())
                    }
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => {
                txt.push(e.unescape_and_decode(&reader)
                             .unwrap())
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!(),
            _ => (),
        }
        buf.clear();
    }
    println!("{:#?}", txt);
    txt
}

/// This function will take a vector of strings and dispatch on the terms to build up moves
/// The vector of strings will have the type of move and the necessary components to build
/// up the move.
pub fn vec_string_to_vec_move(vec_string: Vec<String>) -> Vec<Move> {
    let mut vec_move: Vec<Move> = Vec::new();
    let mut it = vec_string.iter();
    loop {
        match it.next() {
            Some(x) => {
                match x.as_ref() {
                    "enter-piece" => {
                        let curr_move: Move = Move {
                            m_type: MoveType::EnterPiece,
                            pawn: Pawn {
                                color: string_to_color(it.next()
                                                           .unwrap()
                                                           .to_string()),
                                id: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(), //WoW!
                            },
                        };
                        vec_move.push(curr_move);
                    }
                    "move-piece-home" => {
                        let curr_move: Move = Move {
                            pawn: Pawn {
                                color: string_to_color(it.next()
                                                           .unwrap()
                                                           .to_string()),
                                id: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
                            },
                            m_type: MoveType::MoveHome {
                                start: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
                                distance: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
                            },
                        };
                        vec_move.push(curr_move);
                    }
                    "move-piece-main" => {
                        let curr_move: Move = Move {
                            pawn: Pawn {
                                color: string_to_color(it.next()
                                                           .unwrap()
                                                           .to_string()),
                                id: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
                            },
                            m_type: MoveType::MoveMain {
                                start: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
                                distance: it.next()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
                            },
                        };
                        vec_move.push(curr_move);
                    }
                    _ => panic!("XML MOVE NOT RECOGNIZED"),
                }
            }

            None => break,
        }
    }
    vec_move
}

pub fn deserialize_board(xml: String) -> Board {
    let mut vec_string: Vec<String> = xml_board_to_vec_string(xml);

    Board::new()

}

pub fn build_pawn_from_strings(color: String, id: String) -> Pawn {
    let pawn: Pawn = Pawn {
        color: string_to_color(color),
        id: id.parse::<usize>()
            .unwrap(),
    };
    pawn
}

//Todo implement Loc enum by look at strings

pub fn vec_string_to_board(vec_string: Vec<String>) -> Board {
    //loop
    Board::new()
}

pub fn xml_board_to_vec_string(xml: String) -> Vec<String> {
    // Board is a BTreeMap from Color to PawnLocs
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    println!("{:#?}", xml);
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"board" => (),
                    _ => {
                        txt.push(e.unescape_and_decode(&reader)
                                     .unwrap())
                    }
                }
            }
            Ok(Event::Text(e)) => {
                txt.push(e.unescape_and_decode(&reader)
                             .unwrap())
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!(),
            _ => (),
        }
        buf.clear();
    }
    println!("{:#?}", txt);
    txt


}

pub fn string_to_color(string: String) -> Color {
    match string.as_ref() {
        "Red" => Color::Red,
        "Blue" => Color::Blue,
        "Yellow" => Color::Yellow,
        "Green" => Color::Green,
        _ => panic!("string to color"),             
    }
}

mod tests {
    use super::*;
    use parse;

    #[test]
    /// Parse then unparse and check if results are the same
    fn move_vector_test() {
        let m_1: Move = Move {
            m_type: MoveType::EnterPiece,
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            },
        };
        let m_2: Move = Move {
            m_type: MoveType::MoveHome {
                start: 101,
                distance: 3,
            },
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            },
        };
        let m_3: Move = Move {
            m_type: MoveType::MoveMain {
                start: 12,
                distance: 3,
            },
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            },
        };

        let m_vec: Vec<Move> = vec![m_1.clone(), m_2.clone(), m_3.clone()];
        let xml = parse::xml_moves(&m_vec);
        let test: Vec<Move> = deserialize_moves(xml);
        assert!(m_vec == test);
    }

    #[test]
    /// Parse the board
    fn deserialize_board_test() {
        deserialize_board(Board::new().xmlify());
        assert!(false);
    }

    #[test]
    /// Parse real game board
    fn deserialize_board_basic_test() {
        let board: Board = Board::from(map!{
            Color::Red => [Loc::Home, Loc::Spot { index: 103 }, Loc::Spot{ index: 30 }, Loc::Spot{ index: 29}]
        });
        deserialize_board(board.xmlify());
        assert!(false);

    }

}
