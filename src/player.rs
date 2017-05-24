use super::board::{Color, Board};
use super::game::Move;
use super::dice::Dice;

/// Generic Player trait provides an interface for the
/// server to interact with players.
pub trait Player {
    /// Inform the Player that a game has started, and
    /// what color the player is.
    fn start_game(&self) -> String;

    /// Ask the player what move they want to make.
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move>;

    /// Inform the player that they have suffered a doubles
    /// penalty.
    fn doubles_penalty(&self) -> () {
        println!("Penalty on me")
    }
}
