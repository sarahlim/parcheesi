#![allow(dead_code, unused_variables)]

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

static ALL_COLORS: [Color; 4] =
    [Color::Red, Color::Blue, Color::Yellow, Color::Green];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Represents the location of a pawn.
enum Location {
    Spot { index: usize },
    Nest,
    Home,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a color of a Pawn or Player.
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents a pawn on the board.
struct Pawn {
    id: usize, // 0..3
    color: Color,
}

impl Pawn {
    fn new(id: usize, color: Color) -> Pawn {
        assert!(id <= 3);

        Pawn {
            id: id,
            color: color,
        }
    }
}

type PawnLocations = [Location; 4];

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a board state, containing the positions
/// of all pawns.
/// Positions is a map from a color, C, to a four element array, where
/// each index, i, is the location of the ith pawn with color C.
struct Board {
    positions: BTreeMap<Color, PawnLocations>,
}

impl Board {
    fn is_safety(&self, location: Location) -> bool {
        match location {
            Location::Spot { index } => {
                match index {
                    0 | 7 | 12 | 17 | 24 | 29 | 34 | 41 | 46 | 51 | 58 | 63 => {
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn is_home_row(&self, color: Color, location: Location) -> bool {
        let color_offset = self.get_home_row_entrance(color);
        let home_row_entrance_index = match color_offset {
            Location::Spot { index } => index,
            _ => panic!("at the disco"),
        };

        let current_location = match location {
            Location::Spot { index } => index,
            _ => panic!(" "),
        };

        current_location < home_row_entrance_index + 7
    }

    fn get_entrance(&self, color: Color) -> Location {
        let offset = match color {
            Color::Red => RED_ENTRANCE,
            Color::Blue => BLUE_ENTRANCE,
            Color::Yellow => YELLOW_ENTRANCE,
            Color::Green => GREEN_ENTRANCE,
        };

        Location::Spot { index: offset }
    }

    fn get_home_row_entrance(&self, color: Color) -> Location {
        let offset = match color {
            Color::Red => RED_HOME_ROW,
            Color::Blue => BLUE_HOME_ROW,
            Color::Yellow => YELLOW_HOME_ROW,
            Color::Green => GREEN_HOME_ROW,
        };

        Location::Spot { index: offset }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a move selected by a player.
enum Move {
    /// Represents a move that starts on the main ring
    /// (but does not have to end up there).
    MoveMain {
        pawn: Pawn,
        start: usize,
        distance: usize,
    },

    /// Represents a move that starts on one of the
    /// home rows.
    MoveHome {
        pawn: Pawn,
        start: usize,
        distance: usize,
    },

    /// Represents a move where a player enters
    /// a piece.
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
    /// Inform the Player that a game has started, and
    /// what color the player is.
    fn start_game(&self, color: Color) -> ();

    /// Ask the player what move they want to make.
    fn do_move(&self, board: Board, dice: Dice) -> Move;

    /// Inform the player that they have suffered a doubles
    /// penalty.
    fn doubles_penalty(&self) -> ();
}

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

/// Test player.
struct TestPlayer {
    id: i32,
}

impl Player for TestPlayer {
    fn start_game(&self, color: Color) -> () {
        println!("Player {} is color: {:?}", self.id, color);
    }

    fn do_move(&self, board: Board, dice: Dice) -> Move {
        Move::EnterPiece { pawn: Pawn::new(0, Color::Red) }
    }

    fn doubles_penalty(&self) -> () {
        println!("Player {} suffered a doubles penalty", self.id);
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
