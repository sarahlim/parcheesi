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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    fn do_move(&self, board: Board, dice: Dice) -> Move;

    fn doubles_penalty(&self) -> ();
}

/// Represents a game instance with connected Players.
pub struct Game<'a> {
    pub players: BTreeMap<Color, &'a Player>, // Players won't outlive game
    all_colors: [Color; 4],
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game {
            players: BTreeMap::new(),
            all_colors: [Color::Red, Color::Blue, Color::Yellow, Color::Green],
        }
    }

    /// Register a new player with the game.
    /// If there are no remaining colors available, return an error.
    fn register_player<T: Player + 'a>(&mut self, p: &'a T) -> () {
        let num_players_before = self.players.len();

        if self.players.len() <= self.all_colors.len() {
            // Assign a color to the new player.
            for color in self.all_colors.iter() {
                if !self.players
                        .contains_key(color) {
                    self.players
                        .insert(color.clone(), p);
                }
            }
            assert!(self.players.len() > num_players_before);
            println!("Added player to the game. Now there are {} players.",
                     self.players.len());
        } else {
            println!("Game is full :( Unable to add player. Sad!");
        }
    }

    fn start_game() -> () {
        println!("not yet implemented");
    }
}

/// Test player.
// struct TestPlayer {
//     id: i32,
// }

// impl Player for TestPlayer {
//     fn start_game(&self, color: Color) -> () {
//         println!("Player {} is color: {:?}", self.id, color);
//     }

//     fn do_move(&self, board: Board, dice: Dice) -> Move {
//         Move::EnterPiece { pawn: Pawn::new(0, Color::Red) }
//     }

//     fn doubles_penalty(&self) -> () {
//         println!("Player {} suffered a doubles penalty", self.id);
//     }
// }
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

    // #[test]
    // /// Test functionality to add new players. Players should
    // /// all be assigned different colors, and no more than 4
    // /// should be allowed to register.
    // fn test_register_player() {
    //     let mut game = Game::new();
    //     let p1 = TestPlayer { id: 0 };
    //     let p2 = TestPlayer { id: 1 };
    //     let p3 = TestPlayer { id: 2 };
    //     let p4 = TestPlayer { id: 3 };
    //     let p5 = TestPlayer { id: 3 };

    //     let players = [p1, p2, p3, p4];
    //     let colors = [Color::Red, Color::Blue, Color::Yellow, Color::Green];

    //     game.register_player(p1);
    //     assert!(game.players
    //                 .contains_key(&colors[0]));
    //     game.register_player(p2);
    //     assert!(game.players
    //                 .contains_key(&colors[1]));
    //     game.register_player(p3);
    //     assert!(game.players
    //                 .contains_key(&colors[2]));
    //     game.register_player(p4);
    //     assert!(game.players
    //                 .contains_key(&colors[3]));

    // for i in 0..4 {
    //     let p = &players[i];
    //     game.register_player(&players[i]);
    //     assert!(game.players
    //                 .contains_key(&colors[i]));
    // }

    // // Inserting the fifth player should result
    // // in no change to the game state.
    // let num_players = game.players.len();
    // game.register_player(p5);
    // assert_eq!(game.players.len(), num_players);

    // // All colors were used.
    // for clr in colors.iter() {
    //     assert!(game.players
    //                 .contains_key(clr));
    // }
    // }
}
