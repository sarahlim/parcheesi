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


   

/// Current thoughts on the xml things. If a something is a struct, implement
/// an xmlify method for easy access. In this file, all the xmlify calls will
/// get packaged together

/// This function will return a string? corresponding to the xml
/// stream to be sent for the start of the game
pub fn xml_start_game(&color: &Color) -> String {
    let color_string: String = Color::to_string(&color);
    let xml_response: String = "<start-game> ".to_string() + &color_string + " </start-game>";
    xml_response 
}


pub fn xml_start_game_response(player: &XMLTestPlayer) -> String {
    let xml_response: String = "<name> ".to_string() + &player.name.to_string() + " </name>";
    xml_response
}

pub fn xml_do_move(board: &Board, dice: &Dice) -> String {
    let xml_response: String = "<do-move> ".to_string() + &board.xmlify() + " " + &dice.xmlify() + " </do-move>";
    xml_response
}

pub fn xml_moves(move_vec: &Vec<Move>) -> String {
    let mut move_header: String = "<moves>".to_string();
    for moves in move_vec {
        move_header = move_header + " " + &moves.xmlify();
    }
    move_header + " </moves>"
}

pub fn xml_doubles_penalty() -> String {
    "<doubles-penalty> </doubles-penalty>".to_string()
}

pub fn xml_void() -> String {
    "<void> </void>".to_string()       
}





mod test {
    use super::*;
    
    #[test]
    fn xml_start_game_basic() {
        assert!(xml_start_game(&Color::Red) == "<start-game> Red </start-game>");
    }

    #[test]
    fn xml_start_game_response_basic() {
        let test_player: XMLTestPlayer = XMLTestPlayer {
            color: Color::Green,
            name: "Sven".to_string(),
        };
        assert!(xml_start_game_response(&test_player) == "<name> Sven </name>");
    }
    
    #[test]
    fn xmlify_pawn() {
        let pawn = Pawn {
            color: Color::Red,
            id: 2,
        };
        assert!(pawn.xmlify() == "<pawn> <color> Red </color> <id> 2 </id> </pawn>");
        
    }
    
    #[test]
    fn xmlify_dice() {
        let dice = Dice {
            rolls: vec![1,2],
            used: vec![],
        };
        assert!(dice.xmlify() == "<dice> <die> 1 </die> <die> 2 </die> </dice>")
    }

    #[test]
    fn xmlify_move_enter_piece() {
        let m: Move = Move {
            m_type: MoveType::EnterPiece,
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            },
        };
        println!("{}",m.xmlify());
        assert!(m.xmlify() == "<enter-piece> ".to_string() + &m.pawn.xmlify() + " </enter-piece>");
    }

    #[test]
    fn xmlify_move_piece_main() {
        let m: Move = Move {
            m_type: MoveType::MoveMain {
                start: 59,
                distance: 4
            },
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            },
        };
        println!("{}",m.xmlify());
        let expected: String =  "<move-piece-main> ".to_string() + &m.pawn.xmlify() + " <start> "+ &59.to_string() + " </start>" + " <distance> " + &4.to_string() + " </distance> </move-piece-main>";
        println!("{}", expected);
        assert!(m.xmlify() == expected);
    }

    #[test]
    fn xmlify_move_piece_home() {
      let m: Move = Move {
            m_type: MoveType::MoveHome {
                start: 59,
                distance: 4
            },
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            },
        };
        let expected: String =  "<move-piece-home> ".to_string() + &m.pawn.xmlify() + " <start> "+ &59.to_string() + " </start>" + " <distance> " + &4.to_string() + " </distance> </move-piece-home>";
        assert!(m.xmlify() == expected);
    }

    #[test]
    fn xmlify_board_nest() {
        let board: Board = Board::new();
        println!("{:#?}", board);
        println!("{:#?}",board.xmlify());
        assert!(board.xmlify() == "<board> <start> <pawn> <color> Red </color> <id> 0 </id> </pawn> <pawn> <color> Red </color> <id> 1 </id> </pawn> <pawn> <color> Red </color> <id> 2 </id> </pawn> <pawn> <color> Red </color> <id> 3 </id> </pawn> <pawn> <color> Green </color> <id> 0 </id> </pawn> <pawn> <color> Green </color> <id> 1 </id> </pawn> <pawn> <color> Green </color> <id> 2 </id> </pawn> <pawn> <color> Green </color> <id> 3 </id> </pawn> <pawn> <color> Blue </color> <id> 0 </id> </pawn> <pawn> <color> Blue </color> <id> 1 </id> </pawn> <pawn> <color> Blue </color> <id> 2 </id> </pawn> <pawn> <color> Blue </color> <id> 3 </id> </pawn> <pawn> <color> Yellow </color> <id> 0 </id> </pawn> <pawn> <color> Yellow </color> <id> 1 </id> </pawn> <pawn> <color> Yellow </color> <id> 2 </id> </pawn> <pawn> <color> Yellow </color> <id> 3 </id> </pawn> </start> <main> </main> <home-rows> </home-rows> <home> </home> </board>");
    }

    #[test]
    fn xmlify_board_real_game_do_move() {
        let board: Board = Board::from(map!{
            Color::Red => [Loc::Home, Loc::Spot { index: 103 }, Loc::Spot{ index: 30 }, Loc::Spot{ index: 29}]
        });
        let dice: Dice = Dice {
            rolls: vec![1,2],
            used: vec![],
        };

        let expected: String = "<board> <start> <pawn> <color> Green </color> <id> 0 </id> </pawn> <pawn> <color> Green </color> <id> 1 </id> </pawn> <pawn> <color> Green </color> <id> 2 </id> </pawn> <pawn> <color> Green </color> <id> 3 </id> </pawn> <pawn> <color> Blue </color> <id> 0 </id> </pawn> <pawn> <color> Blue </color> <id> 1 </id> </pawn> <pawn> <color> Blue </color> <id> 2 </id> </pawn> <pawn> <color> Blue </color> <id> 3 </id> </pawn> <pawn> <color> Yellow </color> <id> 0 </id> </pawn> <pawn> <color> Yellow </color> <id> 1 </id> </pawn> <pawn> <color> Yellow </color> <id> 2 </id> </pawn> <pawn> <color> Yellow </color> <id> 3 </id> </pawn> </start> <main> <piece-loc> <pawn> <color> Red </color> <id> 2 </id> </pawn> <loc> 30 </loc> </piece-loc> <piece-loc> <pawn> <color> Red </color> <id> 3 </id> </pawn> <loc> 29 </loc> </piece-loc> </main> <home-rows> <piece-loc> <pawn> <color> Red </color> <id> 1 </id> </pawn> <loc> 103 </loc> </piece-loc> </home-rows> <home> <pawn> <color> Red </color> <id> 0 </id> </pawn> </home> </board>".to_string();
        assert!(board.xmlify() == expected);
        // Tests the do move function, TODO separate into another test
        assert!(xml_do_move(&board,&dice) == "<do-move> ".to_string() + &expected + " " + &dice.xmlify() + " </do-move>");
        
    }

    #[test]
    fn xml_moves_() {
        let m_1: Move = Move {
            m_type: MoveType::EnterPiece,
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            }
        };
        let m_2: Move = Move {
            m_type: MoveType::MoveHome { start: 101, distance: 3},
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            }
         };
        let m_3: Move = Move {
            m_type: MoveType::MoveMain { start: 12, distance: 3 },
            pawn: Pawn {
                color: Color::Red,
                id: 2,
            }
        };

        let m_vec:Vec<Move> = vec![m_1.clone(),m_2.clone(),m_3.clone()];
        println!("{}",xml_moves(&m_vec));
        assert!(xml_moves(&m_vec) == "<moves> ".to_string() + &m_1.xmlify() + " " + &m_2.xmlify() + " " + &m_3.xmlify() + " </moves>");
    }
}
