#![allow(dead_code, unused_variables)]

use std::collections::BTreeMap;

use super::dice::Dice;
use super::board::{Color, Board, Pawn, Loc, MoveResult};
use super::constants::*;

/// Represents a game instance with connected Players.
struct Game<'a> {
    players: BTreeMap<Color, &'a (Player + 'a)>, // Players won't outlive game
    dice: Dice,
    board: Board,
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game {
            players: BTreeMap::new(),
            board: Board::new(),
            dice: Dice::new(),
        }
    }

    /// Register a new player with the game.
    /// If there are no remaining colors available, return an error.
    fn register_player<T: Player + 'a>(&mut self, p: &'a T) -> () {
        let num_players_before = self.players.len();

        if self.players.len() <= COLORS.len() {
            // Assign a color to the new player.
            for color in COLORS.iter() {
                if !self.players
                        .contains_key(color) {
                    self.players
                        .insert(color.clone(), p);
                }
            }
            assert!(self.players.len() >= num_players_before);
            println!("Added player to the game. Now there are {} players.",
                     self.players.len());
        } else {
            println!("Game is full :( Unable to add player. Sad!");
        }
    }

    /// Start a game with the currently registered players.
    fn start_game(&mut self) -> () {
        // Notify all registered players.
        if self.players.is_empty() {
            panic!("Can't start a game with zero players");
        }

        for (clr, p) in self.players.iter() {
            p.start_game(*clr);
        }

        println!("Starting game.");

        while !self.is_game_over() {
            for (clr, p) in self.players.iter() {
                // TODO: How to make this work with mutable references?
                // self.give_turn(clr, *p);
                self.give_turn(clr, *p, Dice::roll);
            }
        }

        // When Game is over, print an announcement.
        println!("Game over.");
    }

    /// Communication layer to get a player's choice of move.
    fn get_player_move(&self, clr: &Color) -> Result<Move, &'static str> {
        if let Some(&player) = self.players.get(clr) {
            let m: Move = player.do_move(self.board.clone(), self.dice.clone());
            Ok(m)
        } else {
            Err("No player with color")
        }
    }

    /// Inform player of a doubles penalty, and administer any changes
    /// to the board.
    fn give_doubles_penalty(&self, color: &Color) {
        if let Some(&player) = self.players.get(color) {
            player.doubles_penalty();
        } else {
            panic!("No player with color {:?}", color);
        }
    }

    /// Predicate to check whether the game is over.
    /// Returns true if any of the following are true:
    ///
    /// - No players remaining (e.g. all of them cheated)
    /// - There is a winner (i.e. one player has all pawns home)
    fn is_game_over(&self) -> bool {
        self.board
            .has_winner()
            .is_some() || self.players.is_empty()
    }

    /// Give a turn to a player, keeping track of dice rolls
    /// and changes to the board.
    ///
    /// Chosen moves are validated at two points:
    ///
    /// (1) Every individual move is checked for validity with respect
    ///     to the board and dice it started from.
    ///
    /// (2) At the end of a turn, the starting and ending board/dice states
    ///     are compared to check for cross-turn validity.
    ///     For instance, we can only enforce that blockades don't move
    ///     together if we validate across the entire turn.
    fn give_turn<F>(&self,
                    color: &Color,
                    player: &Player,
                    roll: F)
                    -> (Board, Dice)
        where F: Fn(bool) -> (Dice, bool)
    {
        let mut doubles_rolled: i32 = 0;

        loop {
            // Check if all the player's pawns are on the board.
            // This determines how the dice roll is handled.
            let give_doubles_bonus: bool = self.board
                .all_pawns_entered(color);
            let (rolled_dice, is_doubles): (Dice, bool) =
                roll(give_doubles_bonus);

            if is_doubles {
                doubles_rolled += 1;
                if doubles_rolled > 2 {
                    // Assign doubles penalty.
                    self.give_doubles_penalty(&color);
                    break;
                }
            }

            // To prevent invalid moves from messing up the game
            // state, we capture and play individual moves on a
            // copy of the board and game state.
            let mut temp_board: Board = self.board.clone();
            let mut temp_dice: Dice = rolled_dice;
            let mut turn_done =
                Board::has_valid_moves(&temp_board, &temp_dice, color);

            while !turn_done {
                // Let the player choose a move, given the current board
                // and the available rolls.
                // Validation occurs between moves, and between turns.
                let chosen_move: Move =
                    player.do_move(temp_board.clone(), temp_dice.clone());

                if Board::is_valid_move(&temp_board, &temp_dice, &chosen_move) {
                    if let Ok(MoveResult(next_board, bonus)) =
                        temp_board.handle_move(chosen_move) {

                        // Update temp_board, and add bonus if it exists.
                        temp_board = next_board;
                        if let Some(amt) = bonus {
                            temp_dice = temp_dice.give_bonus(amt);
                        }
                    } else {
                        panic!("Move failed");
                    }
                } else {
                    // Move is invalid, player cheated.
                    panic!("Invalid move");
                }

                turn_done =
                    Board::has_valid_moves(&temp_board, &temp_dice, color);
            }

            // Now we want to validate the entire turn.
            if !self.board
                    .is_valid_turn(&temp_board, &temp_dice, *color) {
                panic!("Invalid turn");
            }

            // if self.is_valid_turn(temp_board,temp_dice)
            // ret temp_board, temp_dice
            // else panic!("Don't cheat");
            if !is_doubles {
                return (temp_board, temp_dice);
            }
        }

        unreachable!();
    }

    fn is_blockaded(&self, index: usize) -> bool {
        self.board
            .get_blockades()
            .contains(&Loc::Spot { index: index })
    }
}


/// Generic Player trait provides an interface for the
/// server to interact with players.
pub trait Player {
    /// Inform the Player that a game has started, and
    /// what color the player is.
    fn start_game(&self, color: Color) -> ();

    /// Ask the player what move they want to make.
    fn do_move(&self, board: Board, dice: Dice) -> Move;

    /// Inform the player that they have suffered a doubles
    /// penalty.
    fn doubles_penalty(&self) -> ();
}


#[derive(Debug,Copy, Clone, PartialEq)]
/// Represents a move selected by a player.
pub enum MoveType {
    /// Represents a move that starts on the main ring
    /// (but does not have to end up there).
    MoveMain { start: usize, distance: usize },

    /// Represents a move that starts on one of the
    /// home rows.
    MoveHome { start: usize, distance: usize },

    /// Represents a move where a player enters
    /// a piece.
    EnterPiece,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub m_type: MoveType,
    pub pawn: Pawn,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test player.
    struct TestPlayer {
        color: Color,
        chosen_move: Move,
    }

    impl TestPlayer {
        fn new(m_type: MoveType, color: Color) -> TestPlayer {
            let p = Pawn::new(0, color);
            let chosen_move: Move = Move {
                m_type: m_type,
                pawn: p,
            };

            TestPlayer {
                color: color,
                chosen_move: chosen_move,
            }
        }
    }

    impl Player for TestPlayer {
        fn start_game(&self, color: Color) -> () {
            println!("TestPlayer is color: {:?}", color);
        }

        fn do_move(&self, board: Board, dice: Dice) -> Move {
            self.chosen_move.clone()
        }

        fn doubles_penalty(&self) -> () {
            println!("TestPlayer {:?} suffered a doubles penalty", self.color);
        }
    }

    #[test]
    /// Test cannot ignore dice roll
    fn enter_1_4() {
        let m: MoveType = MoveType::EnterPiece;
        let p_1 = TestPlayer::new(m.clone(), Color::Green);
        let mut game: Game = Game {
            players: map!{ Color::Green => &p_1 as &Player },
            dice: Dice::new(),
            board: Board::new(),
        };
        let roll_fn = |_| {
            (Dice {
                 rolls: vec![1, 4],
                 used: vec![],
             },
             false)
        };
        let (next_board, next_dice) =
            game.give_turn(&Color::Green, &p_1, roll_fn);
        let green_entry = Board::get_entrance(&Color::Green);
        assert!(next_board.get_pawn_loc(&Color::Green, 0) ==
                Loc::Spot { index: green_entry });
    }

    #[test]
    /// Test functionality to add new players. Players should
    /// all be assigned different colors, and no more than 4
    /// should be allowed to register.
    fn test_register_player() {
        let m: MoveType = MoveType::EnterPiece;
        let p1 = TestPlayer::new(m.clone(), Color::Green);
        let p2 = TestPlayer::new(m.clone(), Color::Green);
        let p3 = TestPlayer::new(m.clone(), Color::Green);
        let p4 = TestPlayer::new(m.clone(), Color::Green);
        let p5 = TestPlayer::new(m.clone(), Color::Green);
        let mut game = Game::new();

        let players = [&p1, &p2, &p3, &p4];
        let colors = [Color::Red, Color::Blue, Color::Yellow, Color::Green];

        for i in 0..4 {
            let p = players[i];
            game.register_player(players[i]);
            assert!(game.players
                        .contains_key(&colors[i]));
        }

        // Inserting the fifth player should result
        // in no change to the game state.
        let num_players = game.players.len();
        game.register_player(&p5);
        assert_eq!(game.players.len(), num_players);

        // All colors were used.
        for clr in colors.iter() {
            assert!(game.players
                        .contains_key(clr));
        }
    }
}
