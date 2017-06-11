use super::player::Player;
use super::board::{Board, Pawn, Color, Loc, PawnLocs, MoveResult};
use super::game::{Move, MoveType};
use super::dice::Dice;
use super::gametree::GameTree;
use super::networkplayer::NetworkPlayer;
use super::deserialize::XmlMessage;
use super::deserialize;
use super::serialize;
use std::net::TcpStream;
use std::io::{Write, BufReader, BufWriter, BufRead};

pub struct XMLTestPlayer {
    pub color: Color,
    pub name: String,
    pub stream: TcpStream,
}

impl Player for XMLTestPlayer {
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        // Create a vector to store the moves we'll eventually return.
        let mut moves: Vec<Move> = Vec::new();

        // Create temporary copies of the board and dice,
        // so we can build up a sequence of moves.
        let mut temp_board: Board = board.clone();
        let mut temp_dice: Dice = dice.clone();

        // Next, we want to loop until we've exhausted all possible
        // moves.

        // options is an iterator over all possible next moves.
        let mut options: GameTree = GameTree::new(board, dice, self.color);

        // In the future we will do something more intelligent but
        // for now just take the first legal move.
        loop {
            if let Some(chosen_move) = options.next() {
                println!("Our chosen move is {:#?}", chosen_move);
                let move_result: Result<MoveResult,
                                        &'static str> =
                    temp_board.handle_move(chosen_move);

                match move_result {
                    Ok(MoveResult(next_board, bonus)) => {
                        temp_board = next_board;
                        match chosen_move.m_type {
                            MoveType::EnterPiece => {
                                temp_dice = temp_dice.consume_entry_move();
                            }
                            MoveType::MoveMain { distance, .. } |
                            MoveType::MoveHome { distance, .. } => {
                                temp_dice = temp_dice
                                    .consume_normal_move(distance);
                            }
                        };

                        if let Some(amt) = bonus {
                            temp_dice = temp_dice.give_bonus(amt);
                        }
                        //println!("we have a valid move for vector {:#?}",chosen_move);
                        // Add the move to the vector.
                        moves.push(chosen_move);

                    }
                    Err(_) => unreachable!(),
                };
            } else {
                // No more options for moves.
                // println!("BREAK");
                break;
            }
            //println!("New Board, with dice, and color {:#?} {:#?} {}", temp_board.clone(), temp_dice.clone(), self.color);
            // Regenerate mini-moves given the new board.
            options = GameTree::new(temp_board.clone(),
                                    temp_dice.clone(),
                                    self.color);
        }
        moves
    }

    fn start_game(&self) -> String {
        self.send(serialize::xml_start_game_response(&self));
        self.name.to_string()
    }
}

impl NetworkPlayer for XMLTestPlayer {
    fn connect(&mut self) -> () {
        self.stream = TcpStream::connect("127.0.0.1:8000").expect("Couldn't connect to the server...");
    }

    fn send(&self, mut msg: String) -> () {
        println!("We send {}\n", msg);
        let mut writer = BufWriter::new(&self.stream);
        msg.push_str("\n");
        writer
            .write_all(msg.as_bytes())
            .expect("Player could not write");
        writer
            .flush()
            .expect("Player could not flush");
    }

    fn receive(&mut self) -> () {
        let mut reader = BufReader::new(&self.stream);
        let mut response: String = String::new();
        reader
            .read_line(&mut response)
            .expect("Player could not read");
        println!("Player received {}", response);
        let decision: XmlMessage =
            deserialize::deserialize_decision(response.clone());
        match decision {
            XmlMessage::StartGame => {
                self.color = deserialize::deserialize_start_game(response);
                self.start_game();
                ()
            }
            XmlMessage::DoMove => {
                // The deserialize method will return a tuple with the board and the dice, so we must decompose that
                // before we proceed
                println!("rec do move");
                let (board, dice) = deserialize::deserialize_do_move(response);
                let moves_vec = self.do_move(board, dice); //TODO move the write to do_move?
                // println!("Our move vec {:#?}", moves_vec);
                self.send(serialize::xml_moves(&moves_vec));
                ()
            }
            XmlMessage::DoublesPenalty => {
                self.send("<void> </void>".to_string());
                self.doubles_penalty()
            }
            XmlMessage::Error => panic!("Could not parse message"),        
        };
    }
}

/// MoveEndPawnPlayer tries to move pawns starting from the most (or least) advanced.
/// Since the only difference between move_first_pawn_player and move_last_pawn_player is whether
/// we iterate over pawns from front to back or the reverse, we define
/// a base MoveEndPawnPlayer, which takes in a boolean indicating
/// whether or not to reverse the pawn list in do_move.

pub struct MoveEndPawnPlayer {
    pub color: Color,
    pub name: String,
    should_reverse_path: bool,
}

impl MoveEndPawnPlayer {
    fn new(name: String,
           color: Color,
           should_reverse_path: bool)
           -> MoveEndPawnPlayer {
        MoveEndPawnPlayer {
            name: name,
            color: color,
            should_reverse_path: should_reverse_path,
        }
    }
}

impl Player for MoveEndPawnPlayer {
    fn start_game(&self) -> String {
        self.name.to_string()
    }

    /// Always try to move the furthest pawn.
    /// If none of the pawns can be moved with any of the mini-moves,
    /// return an empty vector of moves.
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let pawn_locs: PawnLocs = board.get_pawns_by_color(&self.color);
        let mut sorted_pawn_locs: Vec<(usize, Loc)> =
            Board::sort_player_locs(&self.color, pawn_locs);

        // Depending on whether the player tries to move the first or last pawn,
        // we need to reverse the order of the pawns.
        if self.should_reverse_path {
            sorted_pawn_locs.reverse();
        }

        'outer: for &(pawn_id, loc) in sorted_pawn_locs.iter() {
            for &mini_move in dice.rolls.iter() {
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
        moves
    }
}

fn move_first_pawn_player(name: String, color: Color) -> MoveEndPawnPlayer {
    // To move the furthest ahead pawn, we need to iterate over pawns
    // in reverse order, so we set should_reverse_list to true.
    MoveEndPawnPlayer::new(name, color, true)
}

fn move_last_pawn_player(name: String, color: Color) -> MoveEndPawnPlayer {
    MoveEndPawnPlayer::new(name, color, false)
}

mod test {
    use super::*;



    #[test]
    fn do_move_basic() {
        let test_player = move_first_pawn_player("Test".to_string(),
                                                 Color::Green);
        let test_board = Board::from(map!{
            Color::Green => [Loc::Spot {
                index: 58,
            },
            Loc::Nest,
            Loc::Nest,
            Loc::Home,
            ]
        });
        let test_dice = Dice { rolls: vec![5, 5] };
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
    // [1, 2]
    // Green: 67, 19, Home, 36
    // Blue: 18, Home, Nest, 13
    // Expect: MoveMain { start: 36, distance: 1 }
    #[test]
    fn do_move_choose_farthest_pawn() {
        let test_player = move_first_pawn_player("Test".to_string(),
                                                 Color::Green);
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
        let test_dice = Dice { rolls: vec![1, 2] };
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

    // Chooses second pawn if first pawn is blockaded
    // [3, 2]
    // Green: Nest, 34, 47, 19
    // Red: 49, 49
    // Blue: 50
    // Expect: MoveMain { start: 34, distance: 3 }
    #[test]
    fn do_move_choose_second_pawn_if_first_blockaded() {
        let test_player: MoveEndPawnPlayer =
            move_first_pawn_player("Test".to_string(), Color::Green);
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
        let test_dice = Dice { rolls: vec![3, 2] };
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

    // Chooses third pawn if first pawn would overshoot home and second is blockaded
    // [3, 2]
    // Green: Nest, 19, 406, 47
    // Red: 49, 49
    // Blue: 50
    // Expect: MoveMain { start: 19, distance: 3 }
    #[test]
    fn do_move_choose_second_pawn_if_first_overshoot_second_blockaded() {
        let test_player: MoveEndPawnPlayer =
            move_first_pawn_player("Test".to_string(), Color::Green);
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
        let test_dice = Dice { rolls: vec![3, 2] };
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

    // Chooses first pawn, leading to a bop
    // [3, 2]
    // Green: Nest, 34, 47, 19
    // Red: 49
    // Blue: 50
    // Expect: MoveMain { start: 47, distance: 2 }
    #[test]
    fn do_move_choose_first_pawn_and_bop() {
        let test_player: MoveEndPawnPlayer =
            move_first_pawn_player("Test".to_string(), Color::Green);
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
        let test_dice = Dice { rolls: vec![3, 2] };
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
        assert_eq!(test_player.do_move(test_board, test_dice),
                   vec![expected_move]);
    }

    // Enter, if no other pawns can be moved
    // [3, 2]
    // Green: Nest, 19, 406, 47
    // Red: 49, 49
    // Blue: 50, 21,
    // Yellow: 22, 22
    // Expect: EnterPiece
    // I changed the test so that 21 was also unable to be used, else 19 could move there
    #[test]
    fn do_move_enter_if_no_others_can_move() {
        let test_player: MoveEndPawnPlayer =
            move_first_pawn_player("Test".to_string(), Color::Green);
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
        let test_dice = Dice { rolls: vec![3, 2] };
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

    // Return empty array if no moves are valid
    // [3, 3]
    // Green: Nest, 19, 406, 47
    // Blue: 50 //TODOis 50 a safety square, could be trouble if yes
    // Yellow: 22, 22
    // Expect: []
    #[test]
    fn do_move_no_possible_moves() {
        let test_player: MoveEndPawnPlayer =
            move_first_pawn_player("Test".to_string(), Color::Green);
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
        let test_dice = Dice { rolls: vec![3, 3] };
        assert!(test_player
                    .do_move(test_board, test_dice)
                    .pop() == None);
    }

    #[test]
    fn do_move_into_blockade() {
        let test_player: XMLTestPlayer = XMLTestPlayer {
            color: Color::Red,
            name: String::from("Moses"),
            stream: TcpStream::connect("172.217.6.110:80").expect("Could not connect"),
        };
        let test_dice: Dice = Dice { rolls: vec![5] };
        let test_board: Board = Board::from(map!{
            test_player.color => [Loc::Spot { index: Board::get_entrance(&test_player.color) },
                           Loc::Spot { index: Board::get_entrance(&test_player.color) },
                           Loc::Nest,
                                  Loc::Nest,],
            Color::Yellow => [Loc::Spot { index: Board::get_entrance(&Color::Red)+5 },
                              Loc::Spot { index: Board::get_entrance(&Color::Red)+5 },
                               Loc::Nest,
                              Loc::Nest,]
        });


        let move_vector = test_player.do_move(test_board, test_dice);
        println!("The vector of moves is {:#?}", move_vector);
        assert!(move_vector == vec![]);
    }

    #[test]
    fn do_move_random1() {
        let test_player: XMLTestPlayer = XMLTestPlayer {
            color: Color::Red,
            name: String::from("Moses"),
            stream: TcpStream::connect("172.217.6.110:80").expect("Could not connect"),
        };
        let test_dice: Dice = Dice { rolls: vec![3, 5] };
        let test_board: Board = Board::from(map!{
            test_player.color => [Loc::Spot { index: Board::get_entrance(&test_player.color)+1 },
                                  Loc::Spot { index: Board::get_entrance(&test_player.color)+1 },
                                  Loc::Spot { index: Board::get_entrance(&test_player.color) },
                                  Loc::Nest,]
        });


        let move_vector = test_player.do_move(test_board, test_dice);
        println!("The vector of moves is {:#?}", move_vector);
        assert!(move_vector.len() > 1);
    }
}
