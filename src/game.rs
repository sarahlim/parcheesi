extern crate rand;

use self::rand::Rng;

#[derive(Debug, Clone, PartialEq)]
/// Represents a color of a piece or player.
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
        Pawn {
            id: id,
            color: color,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a move selected by the player.
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

// #[derive(Debug, Clone)]
/// Generic Player trait provides an interface for the
/// server to interact with players.
// trait Player {
//     fn start_game(&self, color: Color) -> ();
//     fn do_more(&self, board: Board, dice: Dice) -> Move;
//     fn doubles_penalty(&self) -> ();
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pawn_colors() {
        let y1 = Pawn::new(1, Color::Yellow);
        let r1 = Pawn::new(1, Color::Red);
        let r2 = Pawn::new(2, Color::Red);
        assert_ne!(y1.color, r2.color);
        assert_eq!(r1.color, r2.color);
    }

    #[test]
    /// Make sure dice are fair.
    fn check_dice() {
        let mut freq = [0; 13]; // 2...12 are the outcomes
        let iters = 10000;

        for i in 0..iters {
            let Dice(d1, d2) = roll_dice();
            let sum = d1 + d2;
            freq[sum] += 1;
        }

        let mut max_index = 0;

        for (j, &value) in freq.iter().enumerate() {
            if value > freq[max_index] {
                max_index = j;
            }
        }

        assert_eq!(max_index, 7);
    }
}
