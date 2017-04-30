use super::player::Player;
use super::constants::*;
use super::board::{Board, Pawn, Color, Path, Loc, PawnLocs};
use super::game::{Move, MoveType};
use super::dice::Dice;

pub struct MoveFirstPawnPlayer {
    color: Color,
}


//fn get_moves_from_loc(dice: &Dice, index: usize) -> Vec<Move> {
//
//}

// Function to expose an iterator of the dice rolls, for now jank patch with dices.rolls.iter()

impl Player for MoveFirstPawnPlayer {
    /// Always try to move the furthest pawn.
    /// If none of the pawns can be moved with any of the mini-moves,
    /// return an empty vector of moves.
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let pawn_locs: PawnLocs = board.get_pawns_by_color(&self.color); // call sort player locs here, that vector of pairs removes need to call enumerated below
        // TODO:: Move this into library
        'outer: for (pawn_id, &loc) in pawn_locs
                .iter()
                .enumerate() {
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
                }
            }
        }
        println!("{:#?}",moves);
        moves
    }
}

mod test {
    use super::*;


    #[test]
    fn do_move_basic() {
        let test_player: MoveFirstPawnPlayer =
            MoveFirstPawnPlayer { color: Color::Green };
        let test_board = Board::from( map!{
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
        assert!(test_player.do_move(test_board, test_dice).pop() == Some(expected_move));
    }


}

