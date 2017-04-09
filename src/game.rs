extern crate rand;

use self::rand::Rng;
use std::collections::BTreeMap;


/// THESE ARE BOARD OFFSETS FOR EACH PLAYER.
static RED_ENTRANCE: usize = 0;
static BLUE_ENTRANCE: usize = 17;
static YELLOW_ENTRANCE: usize = 34;
static GREEN_ENTRANCE: usize = 51;

static SAFETY_OFFSET: &'static [usize] = &[7, 12];

static RED_HOME_ROW: usize = 68;
static BLUE_HOME_ROW: usize = 75;
static YELLOW_HOME_ROW: usize = 82;
static GREEN_HOME_ROW: usize = 89;

#[derive(Debug, Copy, Clone, PartialEq)]
/// Represents the location of a pawn.
pub enum Location {
    Spot { index: usize },
    Nest,
    Home,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a color of a Pawn or Player.
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a pawn on the board.
pub struct Pawn {
    id: usize, // 0..3
    color: Color,
}

impl Pawn {
    pub fn new(id: usize, color: Color) -> Pawn {
        assert!(0 <= id && id <= 3);

        Pawn {
            id: id,
            color: color,
        }
    }
}

#[derive(Debug, Clone)]
/// Represents a board state, containing the positions
/// of all pawns.
/// Positions is a map from a color, C, to a four element array, where
/// each index, i, is the location of the ith pawn with color C.
pub struct Board {
    positions: BTreeMap<Color, [Location; 4]>,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a move selected by a player.
pub enum Move {
    MoveMain {
        pawn: Pawn,
        start: usize,
        distance: usize,
    },
    MoveHome {
        pawn: Pawn,
        start: usize,
        distance: usize,
    },
    EnterPiece { pawn: Pawn },
}

#[derive(Debug, Clone)]
/// Holds the result of two die rolls.
pub struct Dice(usize, usize);

/// Simulates the result of rolling two dice.
pub fn roll_dice() -> Dice {
    let d1 = rand::thread_rng().gen_range(1, 7);
    let d2 = rand::thread_rng().gen_range(1, 7);

    Dice(d1, d2)
}

/// Generic Player trait provides an interface for the
/// server to interact with players.
trait Player {
    fn start_game(&self, color: Color) -> ();

    fn do_more(&self, board: Board, dice: Dice) -> Move;

    fn doubles_penalty(&self) -> ();
}

/// Represents a game instance with connected Players.
pub struct Game<'a> {
    players: Vec<&'a Player>, // Players won't outlive game
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game { players: Vec::new() }
    }

    fn register_player(&mut self, p: &'a Player) -> () {
        self.players.push(p);
        println!("Added player to the game. Now there are {} players.",
                 self.players.len());
    }

    fn start_game() -> () {
        println!("not yet implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Pawn color comparison should work as intended.
    fn test_pawn_colors() {
        let y1 = Pawn::new(1, Color::Yellow);
        let r1 = Pawn::new(1, Color::Red);
        let r2 = Pawn::new(2, Color::Red);

        assert_ne!(y1.color, r2.color);
        assert_eq!(r1.color, r2.color);
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
}
