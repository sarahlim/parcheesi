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
static HOME_ROW_LENGTH: usize = 7;

static RED_HOME_ROW: usize = 68;
static BLUE_HOME_ROW: usize = 75;
static YELLOW_HOME_ROW: usize = 82;
static GREEN_HOME_ROW: usize = 89;

static ALL_COLORS: [Color; 4] =
    [Color::Red, Color::Blue, Color::Yellow, Color::Green];


static BOP_BONUS: usize = 20;
static HOME_BONUS: usize = 10;

macro_rules! map {
    ( $( $k:expr => $v:expr ),+ ) => {
        {
            let mut temp_map = ::std::collections::BTreeMap::new();
            $(
                temp_map.insert($k, $v);
            )+
            temp_map
        }
    };
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Represents the location of a pawn.
enum Loc {
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

type PawnLocs = [Loc; 4];



#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a board state, containing the positions
/// of all pawns.
/// Positions is a map from a color, C, to a four element array, where
/// each index, i, is the location of the ith pawn with color C.


struct Board {
    pub positions: BTreeMap<Color, PawnLocs>,
}

struct MoveResult(Board, Option<Bonus>);

impl Board {
    fn new() -> Board {
        let mut positions = BTreeMap::new();
        let init_pawn_locations = [Loc::Nest; 4];

        for c in ALL_COLORS.iter() {
            positions.insert(c.clone(), init_pawn_locations.clone());
        }

        Board { positions: positions }
    }

    fn is_safety(&self, location: Loc) -> bool {
        match location {
            Loc::Spot { index } => {
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

    fn is_home_row(&self, color: Color, location: Loc) -> bool {
        let home_row_entrance_index = self.get_home_row_entrance(color);

        let current_location = match location {
            Loc::Spot { index } => index,
            _ => panic!(" "),
        };

        current_location < home_row_entrance_index + HOME_ROW_LENGTH
    }

    fn get_entrance(&self, color: Color) -> usize {
        match color {
            Color::Red => RED_ENTRANCE,
            Color::Blue => BLUE_ENTRANCE,
            Color::Yellow => YELLOW_ENTRANCE,
            Color::Green => GREEN_ENTRANCE,
        }
    }

    fn get_home_row_entrance(&self, color: Color) -> usize {
        match color {
            Color::Red => RED_HOME_ROW,
            Color::Blue => BLUE_HOME_ROW,
            Color::Yellow => YELLOW_HOME_ROW,
            Color::Green => GREEN_HOME_ROW,
        }
    }

    fn can_bop(&self, bopper_color: Color, dest_index: usize) -> Option<Pawn> {
        // A pawn can bop if all of the following are true:
        // - dest index is not a safety spot,
        // - dest contains one pawn of a different color.

        // 1. If dest_index is safety,
        //    a. bopper's entrance => MIGHT BE ABLE TO BOP, KEEP CHECKING
        //    b. any other safety => CANNOT BOP
        let dest_loc = Loc::Spot { index: dest_index };
        let bopper_entrance = self.get_entrance(bopper_color);

        if self.is_safety(dest_loc) && dest_index != bopper_entrance {
            return None;
        }

        // 2. If spot is not a safety, check if it's occupied
        //    a. Occupied by opponent
        //       i. Blockade => CANNOT BOP
        //       ii. Not blockade => CAN BOP
        //    b. Unoccupied => CANNOT BOP
        let is_dest_index = |&l: &&Loc| match *l {
            Loc::Spot { index } => index == dest_index,
            _ => false,
        };

        for (c, locs) in self.positions.iter() {
            if *c == bopper_color {
                continue;
            }

            // We're looking at the pawn locations of an opponent.
            // Iterate over those and check for opponent pawns.
            let mut occupants = Vec::new();

            for (i, loc) in locs.iter().enumerate() {
                if is_dest_index(&loc) {
                    occupants.push((i, loc));
                }
            }

            // Now `occupants` is a vector of the current opponent's
            // pawns occupying the destination spot.
            if occupants.len() == 1 {
                if let Some((id, _)) = occupants.pop() {
                    // Need to return the boppee.
                    let boppee = Pawn { id: id, color: *c };
                    return Some(boppee);
                };
            } else if occupants.len() == 2 {
                // There is a blockade, so we can't bop no matter what.
                // Immediately return without checking other players' positions.
                return None;
            }
        }

        // If we got through all other players' positions and found no
        // single-occupants, there is nothing to bop.
        None
    }

    /// Takes a move and returns a new board.
    fn handle_move(&self, m: Move) -> Result<MoveResult, &'static str> {
        let mut positions = self.positions.clone();
        let Move {
            pawn: Pawn { color, id },
            m_type,
        } = m;

        let mut bonus = None;


        // The color of the pawn being moved tells us
        // which player's pawn locations we need to modify.
        let player_pawns = self.positions.get(&color);

        if let Some(pawns) = player_pawns {
            let mut next_pawns = *pawns;

            let next_loc = match m_type {
                MoveType::EnterPiece => {
                    Loc::Spot { index: self.get_entrance(color) }
                }
                MoveType::MoveHome { start, distance } => {
                    // Pawns may only move home by an exact amount.
                    let home_row_entrance = self.get_home_row_entrance(color);
                    let home_index = home_row_entrance + HOME_ROW_LENGTH;
                    let dest_index = start + distance;

                    if dest_index == home_index {
                        bonus = Some(HOME_BONUS);
                        Loc::Home
                    } else if dest_index < home_index {
                        // If the destination is short of the Home,
                        // It is just a regular movement in the Home Row.
                        Loc::Spot { index: start + distance }
                    } else {
                        return Err("Can't overshoot home");
                    }
                }
                MoveType::MoveMain { start, distance } => {
                    // If the destination spot has a single pawn of
                    // another color, bop it back to start.
                    let destination = start + distance;

                    if let Some(boppee) = self.can_bop(color, destination) {
                        // If a bop occurs, we need to handle two side effects:

                        // 1. Move the boppee pawn back to its home.
                        let boppee_locs = self.positions
                            .get(&boppee.color);
                        if let Some(old_locs) = boppee_locs {
                            let mut next_locs = old_locs.clone();
                            next_locs[boppee.id] = Loc::Nest; // Monster
                            positions.insert(boppee.color, next_locs);
                        }

                        // 2. Make sure we return a BOP_BONUS move for the bopper.
                        bonus = Some(BOP_BONUS);
                    }

                    // Finally, return the location of the move.
                    Loc::Spot { index: start + distance }
                }
            };

            next_pawns[id] = next_loc;

            // Modify the copy of the positions map
            // with the new positions of the moving player's
            // pawns.
            positions.insert(color, next_pawns);
        }

        Ok(MoveResult(Board { positions: positions }, bonus))
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a move selected by a player.
enum MoveType {
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
struct Move {
    m_type: MoveType,
    pawn: Pawn,
}

#[derive(Debug, Clone)]
/// Holds the result of two die rolls.
struct Dice(usize, usize);

type Bonus = usize;

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

    #[test]
    /// Bopping is allowed if and only if the bopper
    /// and boppee are different colors, and the boppee
    /// is the only pawn on its spot.
    fn test_can_bop() {
        let mut start_board = Board::new();
        let red_pawn_locs = [Loc::Spot { index: RED_ENTRANCE },
                             Loc::Nest,
                             Loc::Nest,
                             Loc::Nest];
        let green_pawn_locs =
            [Loc::Spot { index: 3 }, Loc::Nest, Loc::Nest, Loc::Nest];
        let mut positions = start_board
            .positions
            .clone();
        positions.insert(Color::Red, red_pawn_locs);
        positions.insert(Color::Green, green_pawn_locs);

        start_board = Board { positions: positions };

        // Can bop if different colors and only one pawn on
        // destination square.
        let r_normal = start_board.can_bop(Color::Red, 3);
        assert!(r_normal.is_some());
        assert_eq!(r_normal.unwrap(),
                   Pawn {
                       id: 0,
                       color: Color::Green,
                   });

        // Can't bop own pawn.
        let r_own = start_board.can_bop(Color::Green, 3);
        assert!(r_own.is_none());

        // Can't bop if spot is uninhabited.
        let r_empty = start_board.can_bop(Color::Red, 4);
        assert!(r_empty.is_none());

        let mut blockade_board = start_board.clone();
        let mut green_pawn_blockade_locs = green_pawn_locs;
        green_pawn_blockade_locs[1] = Loc::Spot { index: 3 };

        let mut blockade_positions = blockade_board.positions;
        blockade_positions.insert(Color::Green, green_pawn_blockade_locs);
        blockade_board = Board { positions: blockade_positions };

        // Can't bop a blockade.
        let r_blockade = blockade_board.can_bop(Color::Red, 3);
        assert!(r_blockade.is_none());

        let mut entrance_board = Board::new();
        let green_pawn_entrance_locs = [Loc::Spot { index: RED_ENTRANCE },
                                        Loc::Nest,
                                        Loc::Nest,
                                        Loc::Nest];

        let mut entrance_positions = entrance_board.positions;
        entrance_positions.insert(Color::Green, green_pawn_entrance_locs);
        entrance_board = Board { positions: entrance_positions };

        // Can bop an opponent pawn off our entrance square.
        // Note that the only time we move onto the entrance will
        // be when entering a new pawn.
        // Any other pawn should already have turned off into the
        // Home Row.
        let r_entrance = entrance_board.can_bop(Color::Red, RED_ENTRANCE);
        assert!(r_entrance.is_some());
        assert_eq!(r_entrance.unwrap(),
                   Pawn {
                       id: 0,
                       color: Color::Green,
                   });
    }

    #[test]
    fn handle_enter() {
        let board = Board::new();
        let p = Pawn {
            color: Color::Green,
            id: 0,
        };
        let m = Move {
            m_type: MoveType::EnterPiece,
            pawn: p,
        };
        let mut expected = Board::new();
        let green_pawn_locs = [Loc::Spot { index: GREEN_ENTRANCE },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest];
        let mut positions = expected.positions.clone();
        positions.insert(Color::Green, green_pawn_locs);
        expected = Board { positions: positions };
        let result = board.handle_move(m);
        match result {
            Ok(MoveResult(b, _)) => assert_eq!(expected, b),
            Err(e) => assert!(false),
        };

        //TODO give board a method to update the positions
        //set_posn(&self,p: Pawn,dest: Loc) -> Board
    }

    #[test]
    fn normal_move() {
        let p = Pawn {
            color: Color::Green,
            id: 0,
        };
        let m1 = Move {
            m_type: MoveType::MoveMain {
                start: GREEN_ENTRANCE,
                distance: 3,
            },
            pawn: p,
        };

        let green_pawn_locs = [Loc::Spot { index: GREEN_ENTRANCE },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest];
        let b0 = Board {
            positions: map!{
            Color::Red => [Loc::Nest; 4],
            Color::Blue => [Loc::Nest; 4],
            Color::Yellow => [Loc::Nest; 4],
            Color::Green => green_pawn_locs
        },
        };

        let b1 = Board {
            positions: map!{
            Color::Red => [Loc::Nest; 4],
            Color::Blue => [Loc::Nest; 4],
            Color::Yellow => [Loc::Nest; 4],
            Color::Green =>  [Loc::Spot { index: 54 },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest]
        },
        };

        let b2 = Board {
            positions: map!{
            Color::Red => [Loc::Nest; 4],
            Color::Blue => [Loc::Nest; 4],
            Color::Yellow => [Loc::Nest; 4],
            Color::Green =>  [Loc::Spot { index: 64 },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest]
        },
        };

        let r1 = b0.handle_move(m1);
        match r1 {
            Ok(MoveResult(b, _)) => assert_eq!(b1, b),
            Err(e) => assert!(false),
        };

        //assert_eq!(b1, b0.handle_move(m1));

        let m2 = Move {
            m_type: MoveType::MoveMain {
                start: 54,
                distance: 4,
            },
            pawn: p.clone(),
        };



        let m3 = Move {
            m_type: MoveType::MoveMain {
                start: 58,
                distance: 6,
            },
            pawn: p.clone(),
        };

        let mut r2 = b1.handle_move(m2);
        match r2 {
            Ok(MoveResult(b, _)) => {
                r2 = b.handle_move(m3);
                match r2 {
                    Ok(MoveResult(final_board, _)) => {
                        assert_eq!(final_board, b2)
                    }
                    Err(e) => assert!(false),
                }
            }
            Err(e) => assert!(false),
        };

        //assert_eq!(b2,
        //          b1.handle_move(m2)
        //             .handle_move(m3));

    }

    #[test]
    #[ignore]
    fn test_handle_bop() {

        assert!(false);
    }
    #[test]
    #[ignore]
    fn double_bonus() {

        assert!(false);
    }

    #[test]
    #[ignore]
    fn double_repeats() {
        assert!(false);
    }

    #[test]
    #[ignore]
    fn home_move() {
        assert!(false);
    }

    #[test]
    #[ignore]
    fn home_bonus() {
        assert!(false);
    }
    //TODO test for equality between format strings
}
