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

pub fn xml_do_move(dice: &Dice, board: &Board) {
    
}

pub fn xml_doubles_penalty() -> String {
    "<doubles-penalty></doubles-penalty>".to_string()
}

pub fn xml_void() -> String {
    "<void></void>".to_string()       
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
    fn xmlify_board() {
        let board: Board = Board::new();
        println!("{:#?}", board);
        println!("{:#?}",board.xmlify());
        assert!(board.xmlify() == "<board> <start> <pawn> <color> Red </color> <id> 0 </id> </pawn> <pawn> <color> Red </color> <id> 1 </id> </pawn> <pawn> <color> Red </color> <id> 2 </id> </pawn> <pawn> <color> Red </color> <id> 3 </id> </pawn> <pawn> <color> Green </color> <id> 0 </id> </pawn> <pawn> <color> Green </color> <id> 1 </id> </pawn> <pawn> <color> Green </color> <id> 2 </id> </pawn> <pawn> <color> Green </color> <id> 3 </id> </pawn> <pawn> <color> Blue </color> <id> 0 </id> </pawn> <pawn> <color> Blue </color> <id> 1 </id> </pawn> <pawn> <color> Blue </color> <id> 2 </id> </pawn> <pawn> <color> Blue </color> <id> 3 </id> </pawn> <pawn> <color> Yellow </color> <id> 0 </id> </pawn> <pawn> <color> Yellow </color> <id> 1 </id> </pawn> <pawn> <color> Yellow </color> <id> 2 </id> </pawn> <pawn> <color> Yellow </color> <id> 3 </id> </pawn> </start> <main> </main> <home-rows> </home-rows> <home> </home> </board>");
    }
}
