use super::player::Player;
use super::constants::*;
use super::board::{Board, Pawn, Color, Path, Loc, PawnLocs};
use super::game::{Move, MoveType};
use super::dice::Dice;

pub struct MoveFirstPawnPlayer {
    pub color: Color,
}

pub struct MoveLastPawnPlayer {
    pub color: Color,
}

pub struct XMLTestPlayer {
    pub color: Color,
    pub name: String,
}

impl Player for XMLTestPlayer {
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        vec![]
    }

    fn start_game(&self) -> () {
        //self.name; // Send this over the wire
    }
}

//fn get_moves_from_loc(dice: &Dice, index: usize) -> Vec<Move> {
//
//}

// Function to expose an iterator of the dice rolls, for now jank patch with dices.rolls.iter()

impl Player for MoveFirstPawnPlayer {
    fn start_game(&self) -> () {}

    /// Always try to move the furthest pawn.
    /// If none of the pawns can be moved with any of the mini-moves,
    /// return an empty vector of moves.
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let pawn_locs: PawnLocs = board.get_pawns_by_color(&self.color); // call sort player locs here, that vector of pairs removes need to call enumerated below
        let mut sorted_pawn_locs: Vec<(usize, Loc)> =
            Board::sort_player_locs(&self.color, pawn_locs); // cannot use board.etc bc for functions bc it is a method and doesn't have a self reference.
        // TODO:: Move this into library
        sorted_pawn_locs.reverse();
        println!("Sorted pawn locs are {:#?}", sorted_pawn_locs);
        'outer: for &(pawn_id, loc) in sorted_pawn_locs.iter() {
            'inner: for &mini_move in dice.rolls.iter() {
                println!("Die Roll is {:#?} Id is  {:#?} Loc is {:#?} {:#?}",
                         mini_move,
                         pawn_id,
                         loc,
                         &self.color);
                let m = Move {
                    pawn: Pawn {
                        color: self.color,
                        id: pawn_id,
                    },
                    m_type: match loc {
                        Loc::Nest => MoveType::EnterPiece,
                        Loc::Home => continue,
                        Loc::Spot { index } => {
                            if Board::is_home_row(self.color, loc) {
                                MoveType::MoveHome {
                                    start: index,
                                    distance: mini_move,
                                }
                            } else {
                                MoveType::MoveMain {
                                    start: index,
                                    distance: mini_move,
                                }
                            }
                        }
                    },
                };
                // is valid move check should be done here
                if Board::is_valid_move(&board, &dice, &m) {
                    moves.push(m);
                    break 'outer;
                } else {
                    println!("invalid");
                }
            }
        }
        println!("{:#?}", moves);
        moves
    }
}

impl Player for MoveLastPawnPlayer {
    fn start_game(&self) -> () {}


    /// Always try to move the furthest pawn.
    /// If none of the pawns can be moved with any of the mini-moves,
    /// return an empty vector of moves.
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let pawn_locs: PawnLocs = board.get_pawns_by_color(&self.color); // call sort player locs here, that vector of pairs removes need to call enumerated below
        let mut sorted_pawn_locs: Vec<(usize, Loc)> =
            Board::sort_player_locs(&self.color, pawn_locs); // cannot use board.etc bc for functions bc it is a method and doesn't have a self reference.
        // TODO:: Move this into library
        println!("Sorted pawn locs are {:#?}", sorted_pawn_locs);
        'outer: for &(pawn_id, loc) in sorted_pawn_locs.iter() {
            'inner: for &mini_move in dice.rolls.iter() {
                println!("Die Roll is {:#?} Id is  {:#?} Loc is {:#?} {:#?}",
                         mini_move,
                         pawn_id,
                         loc,
                         &self.color);
                let m = Move {
                    pawn: Pawn {
                        color: self.color,
                        id: pawn_id,
                    },
                    m_type: match loc {
                        Loc::Nest => MoveType::EnterPiece,
                        Loc::Home => continue,
                        Loc::Spot { index } => {
                            if Board::is_home_row(self.color, loc) {
                                MoveType::MoveHome {
                                    start: index,
                                    distance: mini_move,
                                }
                            } else {
                                MoveType::MoveMain {
                                    start: index,
                                    distance: mini_move,
                                }
                            }
                        }
                    },
                };
                // is valid move check should be done here
                if Board::is_valid_move(&board, &dice, &m) {
                    moves.push(m);
                    break 'outer;
                } else {
                    println!("invalid");
                }
            }
        }
        println!("{:#?}", moves);
        moves
    }
}

mod test {
    use super::*;


    #[test]
    fn do_move_basic() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Spot {
                index: 58,
            },
                             Loc::Nest,
                             Loc::Nest,
                             Loc::Home,
            ]
        });
        let test_dice = Dice {
            rolls: vec![5, 5],
            used: vec![],
        };
        let expected_move = Move {
            m_type: MoveType::MoveMain {
                start: 58,
                distance: 5,
            },
            pawn: Pawn {
                id: 0,
                color: Color::Green,
            },
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == Some(expected_move));
    }
    // Chooses furthest pawn
    /*
[1, 2]
Green: 67, 19, Home, 36
Blue: 18, Home, Nest, 13
// Expect: MoveMain { start: 36, distance: 1 }
*/






    #[test]
    fn do_move_choose_farthest_pawn() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Spot {
                index: 67,
            },
                             Loc::Spot {
                                 index: 19,
                             },
                             Loc::Home,
                             Loc::Spot {
                                 index: 36,
                             },
            ],
            Color::Blue => [Loc::Spot {
                index: 18,
            },
                            Loc::Home,
                            Loc::Nest,
                            Loc::Spot {
                                index: 13,
                            }
            ]                    
        });
        let test_dice = Dice {
            rolls: vec![1, 2],
            used: vec![],
        };
        let expected_move = Move {
            m_type: MoveType::MoveMain {
                start: 36,
                distance: 1,
            },
            pawn: Pawn {
                id: 3,
                color: Color::Green,
            },
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == Some(expected_move));
    }
    /*   
// Chooses second pawn if first pawn is blockaded
[3, 2]
Green: Nest, 34, 47, 19
Red: 49, 49
Blue: 50
// Expect: MoveMain { start: 34, distance: 3 }
    
     */
    #[test]
    fn do_move_choose_second_pawn_if_first_blockaded() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Nest,
                             Loc::Spot {
                                 index: 34,
                             },
                             Loc::Spot {
                                 index: 47,
                             },
                             Loc::Spot {
                                 index: 19,
                             },
            ],
            Color::Red => [Loc::Spot {
                index: 49,
            },
                           Loc::Spot {
                               index: 49,
                           },
                           Loc::Nest,
                           Loc::Nest,],
            Color::Blue => [Loc::Spot {
                index: 50,
            },
                            Loc::Nest,
                            Loc::Nest,
                            Loc::Nest]
        });
        let test_dice = Dice {
            rolls: vec![3, 2],
            used: vec![],
        };
        let expected_move = Move {
            m_type: MoveType::MoveMain {
                start: 34,
                distance: 3,
            },
            pawn: Pawn {
                id: 1,
                color: Color::Green,
            },
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == Some(expected_move));
    }
    /*
// Chooses third pawn if first pawn would overshoot home and second is blockaded
[3, 2]
Green: Nest, 19, 406, 47
Red: 49, 49
Blue: 50
// Expect: MoveMain { start: 19, distance: 3 }
     */
    #[test]
    fn do_move_choose_second_pawn_if_first_overshoot_second_blockaded() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Nest,
                             Loc::Spot {
                                 index: 19,
                             },
                             Loc::Spot {
                                 index: 406,
                             },
                             Loc::Spot {
                                 index: 47,
                             },
            ],
            Color::Red => [Loc::Spot {
                index: 49,
            },
                           Loc::Spot {
                               index: 49,
                           },
                           Loc::Nest,
                           Loc::Nest,],
            Color::Blue => [Loc::Spot {
                index: 50,
            },
                            Loc::Nest,
                            Loc::Nest,
                            Loc::Nest]
        });
        let test_dice = Dice {
            rolls: vec![3, 2],
            used: vec![],
        };
        let expected_move = Move {
            m_type: MoveType::MoveMain {
                start: 19,
                distance: 3,
            },
            pawn: Pawn {
                id: 1,
                color: Color::Green,
            },
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == Some(expected_move));
    }

    /*
// Chooses first pawn, leading to a bop
[3, 2]
Green: Nest, 34, 47, 19
Red: 49
Blue: 50
// Expect: MoveMain { start: 47, distance: 2 } 
     */
    #[test]
    fn do_move_choose_first_pawn_and_bop() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Nest,
                             Loc::Spot {
                                 index: 34,
                             },
                             Loc::Spot {
                                 index: 47,
                             },
                             Loc::Spot {
                                 index: 19,
                             },
            ],
            Color::Red => [Loc::Spot {
                index: 49,
            },
                           Loc::Nest,
                           Loc::Nest,
                           Loc::Nest,],
            Color::Blue => [Loc::Spot {
                index: 50,
            },
                            Loc::Nest,
                            Loc::Nest,
                            Loc::Nest]
        });
        let test_dice = Dice {
            rolls: vec![3, 2],
            used: vec![],
        };
        let expected_move = Move {
            m_type: MoveType::MoveMain {
                start: 47,
                distance: 2,
            },
            pawn: Pawn {
                id: 2,
                color: Color::Green,
            },
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == Some(expected_move));
    }
    /*

// Enter, if no other pawns can be moved
[3, 2]
Green: Nest, 19, 406, 47
Red: 49, 49
Blue: 50, 21, 
Yellow: 22, 22
// Expect: EnterPiece
    // I changed the test so that 21 was also unable to be used, else 19 could move there
 */

    #[test]
    fn do_move_enter_if_no_others_can_move() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Nest,
                             Loc::Spot {
                                 index: 19,
                             },
                             Loc::Spot {
                                 index: 406,
                             },
                             Loc::Spot {
                                 index: 47,
                             },
            ],
            Color::Red => [Loc::Spot {
                index: 49,
            },
                           Loc::Spot {
                               index: 49,
                           },
                           Loc::Nest,
                           Loc::Nest,],
            Color::Blue => [Loc::Spot {
                index: 50,
            },
                            Loc::Spot {
                                index: 21,
                            },
                            Loc::Nest,
                            Loc::Nest],
            Color::Yellow => [Loc::Spot {
                index: 22,
            },
                              Loc::Spot {
                                  index: 22,
                              },
                              Loc::Nest,
                              Loc::Nest]
                              
        });
        let test_dice = Dice {
            rolls: vec![3, 2],
            used: vec![],
        };
        let expected_move = Move {
            m_type: MoveType::EnterPiece,
            pawn: Pawn {
                id: 0,
                color: Color::Green,
            },
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == Some(expected_move));
    }
    /*
// Return empty array if no moves are valid
[3, 3]
Green: Nest, 19, 406, 47
Blue: 50 //TODOis 50 a safety square, could be trouble if yes
Yellow: 22, 22
// Expect: []
*/
    #[test]
    fn do_move_no_possible_moves() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from(map!{
            Color::Green => [Loc::Nest,
                             Loc::Spot {
                                 index: 19,
                             },
                             Loc::Spot {
                                 index: 406,
                             },
                             Loc::Spot {
                                 index: 47,
                             },
            ],            
            Color::Blue => [Loc::Spot {
                index: 50,
            },
                            Loc::Nest,
                            Loc::Nest,
                            Loc::Nest],
            Color::Yellow => [Loc::Spot {
                index: 22,
            },
                              Loc::Spot {
                                  index: 22,
                              },
                              Loc::Nest,
                              Loc::Nest]
                              
        });
        let test_dice = Dice {
            rolls: vec![3, 3],
            used: vec![],
        };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == None);
    }



}
