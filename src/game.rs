extern crate rand;

use self::rand::Rng;

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
pub struct Board {}

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
struct Dice(usize, usize);

/// Simulates the result of rolling two dice.
fn roll_dice() -> Dice {
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
