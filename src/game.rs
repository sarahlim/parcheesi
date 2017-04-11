extern crate rand;

use self::rand::Rng;

use std::collections::BTreeMap;
use super::board::{Color, ALL_COLORS, Board, Pawn};

/// Represents a game instance with connected Players.
struct Game<'a> {
    players: BTreeMap<Color, &'a Player>, // Players won't outlive game
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game { players: BTreeMap::new() }
    }

    /// Register a new player with the game.
    /// If there are no remaining colors available, return an error.
    fn register_player<T: Player + 'a>(&mut self, p: &'a T) -> () {
        let num_players_before = self.players.len();

        if self.players.len() <= ALL_COLORS.len() {
            // Assign a color to the new player.
            for color in ALL_COLORS.iter() {
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
    fn start_game() -> () {
        println!("not yet implemented");
    }
}

/// Simulates the result of rolling two dice.
fn roll_dice() -> Dice {
    let d1 = rand::thread_rng().gen_range(1, 7);
    let d2 = rand::thread_rng().gen_range(1, 7);

    Dice(d1, d2)
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


#[derive(Debug, Clone)]
/// Holds the result of two die rolls.
pub struct Dice(usize, usize);

#[derive(Debug, Clone, PartialEq)]
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
        id: i32,
    }

    impl Player for TestPlayer {
        fn start_game(&self, color: Color) -> () {
            println!("Player {} is color: {:?}", self.id, color);
        }

        fn do_move(&self, board: Board, dice: Dice) -> Move {
            let p = Pawn::new(0, Color::Red);
            Move {
                m_type: MoveType::EnterPiece,
                pawn: p,
            }
        }

        fn doubles_penalty(&self) -> () {
            println!("Player {} suffered a doubles penalty", self.id);
        }
    }

    #[test]
    /// Make sure dice are fair, i.e. the sum of two die rolls
    /// is most frequently 7.
    fn test_dice() {
        let mut freq = [0; 13]; // 2...12 are the outcomes
        let iters = 10000;

        for i in 0..iters {
            let Dice(d1, d2) = roll_dice();
            let sum = d1 + d2;
            freq[sum] += 1;
        }

        let max_index = freq.iter()
            .enumerate()
            .max_by_key(|&(_, x)| x)
            .unwrap()
            .0; // Index corresponds to sum

        assert_eq!(max_index, 7);
    }
    #[test]
    /// Test functionality to add new players. Players should
    /// all be assigned different colors, and no more than 4
    /// should be allowed to register.
    fn test_register_player() {
        let p1 = TestPlayer { id: 0 };
        let p2 = TestPlayer { id: 1 };
        let p3 = TestPlayer { id: 2 };
        let p4 = TestPlayer { id: 3 };
        let p5 = TestPlayer { id: 4 };
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
