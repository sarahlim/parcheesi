use super::board::{Board, PawnLocs, Color, Pawn, Loc, MoveResult};
use super::game::{Move, MoveType};
use super::dice::{Dice, EntryMove};

/// Given some board and dice, iterate over the possible
/// next states, and yield the legal ones.

/// TODO: Currently Board has a method called has_valid_moves,
/// which needs to be ported here.
#[derive(Debug)]
pub struct GameTree {
    color: Color,
    board: Board,
    dice: Dice,
    pawns: PawnLocs,
    current_pawn: usize,
    current_roll: usize,
}

impl GameTree {
    pub fn new(board: Board, dice: Dice, color: Color) -> GameTree {
        GameTree {
            pawns: board.get_pawns_by_color(&color),
            color: color,
            board: board,
            dice: dice,
            current_pawn: 0,
            current_roll: 0,
        }
    }
}

impl Iterator for GameTree {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        // We don't know how Rust implements generators, so we're using
        // an iterator and manually saving the state.

        // Try entering.
        let entry: EntryMove = self.dice.can_enter();
        let entry_roll: Vec<usize> = match entry {
            EntryMove::WithFive => vec![5],
            EntryMove::WithSum(a, b) => vec![a, b], // (2)
            EntryMove::NoEntry => vec![],
        };

        if !entry_roll.is_empty() {
            // Get pawns.
            for (i, &loc) in self.pawns
                    .iter()
                .enumerate() {
                    if Loc::Nest == loc && !(self.board.get_blockades().iter().any(|&x| x == Loc::Spot {
                        index: Board::get_entrance(&self.color),
                    })) //Wonky closure to make sure we aren't going into our entrance that contains a blockade 
                    {
                        // Schedule pawn to enter.
                        let entry = Move {
                            pawn: Pawn {
                                id: i,
                                color: self.color,
                            },
                            m_type: MoveType::EnterPiece,
                        };

                        return Some(entry);
                        // TODO: Cache state for subsequent iterations.
                    }
            }
            let pawn_loc: Loc = self.pawns[self.current_pawn];
        }

        // We iterate over pawns from [0, 4) and rolls from [0, rolls.size()).
        while self.current_pawn < 4 {
            while self.current_roll < self.dice.rolls.len() {

                // Given some (pawn, roll) pair, we want to build the
                // resulting move, and test its validity.

                // Build the appropriate MoveType for the current
                // pawn's current location.
                let pawn_loc: Loc = self.pawns[self.current_pawn];
                let move_distance: usize = self.dice.rolls[self.current_roll];

                let m_type: MoveType = match pawn_loc {
                    Loc::Nest => MoveType::EnterPiece,
                    Loc::Home => continue,
                    Loc::Spot { index } => {
                        if Board::is_home_row(self.color, pawn_loc) {
                            MoveType::MoveHome {
                                start: index,
                                distance: move_distance,
                            }
                        } else {
                            MoveType::MoveMain {
                                start: index,
                                distance: move_distance,
                            }
                        }
                    }
                };

                // Need to build the whole move, including the MoveType
                // we just created.
                let mv: Move = Move {
                    pawn: Pawn {
                        color: self.color,
                        id: self.current_pawn,
                    },
                    m_type: m_type,
                };

                // Update current_roll to save state, in case we
                // return in the next branch.
                self.current_roll += 1;

                // Check if the constructed move is valid,
                // on an individual mini-move level,
                // as well as on the turn level.
                let temp_board: Board = self.board.clone();
                let is_valid_mini_move: bool =
                    Board::is_valid_move(&self.board, &self.dice, &mv);


                let move_result: Result<MoveResult,
                                        &'static str> = temp_board
                    .handle_move(mv);

                let is_valid_for_turn: bool = match move_result {
                    Ok(MoveResult(next_board, _)) => {
                        // Test mini move at the turn level.
                        self.board
                            .is_valid_turn(&next_board, &self.dice, self.color)
                    }
                    Err(_) => false,
                };

                if is_valid_mini_move && is_valid_for_turn {
                    // We have a valid move! Return it to the caller.
                    return Some(mv);
                }
            }

            // If we've exhausted all the possible mini-moves
            // for the current pawn, increment the current_pawn
            // and reset the current_roll counter.
            self.current_pawn += 1;
            self.current_roll = 0;
        }

        // If we've gone through all pairs of pawns and mini-moves,
        // there are no remaining valid moves.
        None
    }
}
