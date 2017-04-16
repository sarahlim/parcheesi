#![allow(dead_code, unused_variables)]

extern crate rand;


use self::rand::Rng;

use std::collections::BTreeMap;
use super::board::{Color, Board, Pawn, Loc};
use super::constants::*;

type MiniMoves = Vec<usize>;

/// Represents a game instance with connected Players.
struct Game<'a> {
    players: BTreeMap<Color, &'a Player>, // Players won't outlive game
    current_turn: Option<Color>,
    board: Board,
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game {
            players: BTreeMap::new(),
            current_turn: None,
            board: Board::new(),
        }
    }

    fn is_blockaded(&self, dest: usize) -> bool {
        // Iterate over the board positions looking for blockades
        // loc is an array of locations by color of players
        for (c,locs) in self.board.positions.iter() {
            let mut occupancy = 0;
            for loc in locs.iter() {
                let x = Loc::Spot { index: dest };
                if x == *loc {
                    occupancy += 1;
                }
            }
            if occupancy == 2 {
                return true;
            }
            if occupancy >= 3 {
                panic!{"Three pawn blockade!"}
            }
        }
        false
    }
    
    fn is_valid_move (&self, mini_moves: MiniMoves, m: Move) -> bool {
        
        let Move{ pawn: Pawn { color, id },
                  m_type} = m;
        match m.m_type {
            MoveType::EnterPiece => {
                let all_pawns_entered = self.board.all_pawns_entered(color);
                let home_row_entrance = self.board.get_home_row_entrance(color);
                let is_blockade =  self.is_blockaded(home_row_entrance);
                all_pawns_entered && is_blockade
            },
            MoveType::MoveMain { start, distance } => {
                // Ensure pawn is currently at start location in the
                // Main Ring.
                if let Some(pawn_locs) = self.board.positions.get(&color) {
                    let spot = Loc::Spot{ index: start };
                    if pawn_locs[id] != spot {
                        return false;
                    }
                } // Jesus saves //
                // We want to ensure the distance traveled is legal
                if !mini_moves.contains(&distance) {
                    return false;
                }
                // Don't let the pawn go through any blockades on their
                // way to the destination.
                for i in 0..distance {
                    let end = self.board.get_main_ring_exit(color);
                    // pawns should wrap into their home row
                    // We have to this because we are using absolute addressing
                    // and some pawn's home rows may not be the next number after
                    // the end of the board
                    // If red is at 60, and it rolls a 5
                    // it would proceed 61,62,63,68,69
                    //                           ^ is the home row entrance
                    let is_past_end = start + i > (self.board.get_entrance(color) - EXIT_TO_ENTRANCE) % BOARD_SIZE;
                    let offset = if is_past_end { end } else { start };
                    if self.is_blockaded(offset + i) {
                        return false;
                    }
                }
                true
            }
        
            MoveType::MoveHome {start, distance } => {
                // Ensure pawn is currently at start location in the
                // Main Ring.
                if let Some(pawn_locs) = self.board.positions.get(&color) {
                    if start < self.board.get_home_row_entrance(color) {
                        return false;
                    }
                    let spot = Loc::Spot{ index: start };
                    if pawn_locs[id] != spot {
                        return false;
                    }
                } // Jesus saves //
                // We want to ensure the distance traveled is legal
                if !mini_moves.contains(&distance) {
                    return false;
                }

                
                // Don't let the pawn go through any blockades on their
                // way to the destination.
                for i in 0..distance {
                    let end = self.board.get_main_ring_exit(color);
                    // pawns should wrap into their home row
                    // We have to this because we are using absolute addressing
                    // and some pawn's home rows may not be the next number after
                    // the end of the board
                    // If red is at 60, and it rolls a 5
                    // it would proceed 61,62,63,68,69
                    //                           ^ is the home row entrance
                    let is_past_end = start + i > (self.board.get_entrance(color) - EXIT_TO_ENTRANCE) % BOARD_SIZE;
                    let offset = if is_past_end { end } else { start };
                    if self.is_blockaded(offset + i) {
                        return false;
                    }                 
                }
                // Allows us to see if the move is overshooting the home
                let overshoot = self.board.get_home_row_entrance(color) + HOME_ROW_LENGTH;
                if start + distance > overshoot {
                    return false;
                }
                true
            }
        }
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
    fn start_game(&mut self) -> () {
        if self.players.iter().len() == 0 {
            panic!("Can't start a game with zero players");
        } else {
            for (clr, p) in self.players.iter() {
                p.start_game(*clr);
            }

            println!("Starting game.");

            // Set the first registered player to the current turn.
            if let Some(clr) = self.players.keys().next() {
                self.current_turn = Some(*clr);
            }
            self.turn();
        }
    }

    fn is_game_over(&self) -> bool {
        let positions = &self.board.positions;
        // Game is over if one player has all of their pieces in
        // the Home Spot.
        for (clr, p) in self.players.iter() {
            if let Some(pawn_locs) = positions.get(clr) {
                // Iterate over pawn locations to check if all
                // are home.
                for loc in pawn_locs {
                    if let &Loc::Home = loc {
                        continue;
                    } else {
                        break;
                    }
                }

                // If we iterated through all of the pawns,
                // the player has won.
                return true;
            }
        }

        false
    }

    fn turn(&mut self) -> () {
        if self.is_game_over() {
            println!("Game over.");
        }

        let mut consecutive_turns = 0;
        let dice = roll_dice();
        let mut rolls: Vec<usize> = vec![dice.0, dice.1];

        // Check for doubles.
        if is_doubles(&dice) {
            // Check if the player has taken three consecutive doubles turns.
            if consecutive_turns >= 3 {
                // Player forfeits turn, and their furthest pawn moves
                // back to the nest.


            }

            // Check if all the player's pawns are on the board.
            // If so, begin a Movement of distance M, and the other
            // two pawns begin a Movement of distance (7 - M).
            // When dividing the pawns into pairs, a pair may not be a blockade
            // (starting on the same spot).
            // The list of mini-moves is empty afterwards.

            // If the player does not have all their pawns on the board,
            // the turn proceeds as normal.

            // Award the player another turn, and keep track of the number of turns.
            consecutive_turns += 1;
        } else {

        }
        // TODO: Implement this.
    }
}


/// Simulates the result of rolling two dice.
fn roll_dice() -> Dice {
    let d1 = rand::thread_rng().gen_range(1, 7);
    let d2 = rand::thread_rng().gen_range(1, 7);

    Dice(d1, d2)
}

fn is_doubles(dice: &Dice) -> bool {
    dice.0 == dice.1
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
