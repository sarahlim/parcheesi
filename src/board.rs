#![allow(dead_code, unused_variables)]

use std::collections::BTreeMap;
use super::game::{Move, MoveType};
use super::constants::*;

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
pub enum Loc {
    Spot { index: usize },
    Nest,
    Home,
}

pub type PawnLocs = [Loc; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a color of a Pawn or Player.
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents a pawn on the board.
pub struct Pawn {
    pub color: Color,
    pub id: usize, // 0..3
}

impl Pawn {
    pub fn new(id: usize, color: Color) -> Pawn {
        assert!(id <= 3);

        Pawn {
            id: id,
            color: color,
        }
    }
}

pub struct MoveResult(pub Board, pub Option<Bonus>);

type Bonus = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a board, containing the positions of all pawns.
pub struct Board {
    positions: BTreeMap<Color, PawnLocs>,
}

impl Board {
    // Initialize a new game board in the starting configuration,
    // i.e. all pawns are in their respective nests.
    pub fn new() -> Board {
        let mut positions: BTreeMap<Color, PawnLocs> = BTreeMap::new();
        let init_pawn_locs: PawnLocs = [Loc::Nest; 4];

        for clr in ALL_COLORS.iter() {
            for i in 0..4 {
                positions.insert(clr.clone(), init_pawn_locs.clone());
            }
        }

        Board { positions: positions }
    }

    /// Checks if a player has won the game, and returns an Option containing
    /// the winner's color if one exists.
    pub fn has_winner(&self) -> Option<Color> {
        let all_home = |pawn_locs: PawnLocs| {
            pawn_locs
                .iter()
                .all(|&loc| loc == Loc::Home)
        };
        if let Some((clr, _)) =
            self.positions
                .iter()
                .find(|&(_, &pawn_locs)| all_home(pawn_locs)) {
            Some(clr.clone())
        } else {
            None
        }
    }

    /// Returns a list of all blockades on the board.
    pub fn get_blockades(&self) -> Vec<Loc> {
        let blockades_for_color = |locs: PawnLocs| {
            let mut blockades: Vec<Loc> = Vec::new();
            let mut seen: Vec<Loc> = Vec::new();
            for loc in locs.iter() {
                if seen.contains(loc) {
                    blockades.push(*loc);
                } else {
                    seen.push(*loc);
                }
            }
            blockades
        };

        self.positions
            .iter()
            .map(|(color, &locs)| blockades_for_color(locs))
            .fold(Vec::new(), |mut memo, mut b| {
                memo.append(&mut b);
                memo
            })
    }

    /// Checks the location of a pawn in the main ring.
    pub fn get_pawn_loc(&self, pawn: &Pawn) -> Loc {
        if let Some(locs) = self.positions
               .get(&pawn.color) {
            locs[pawn.id]
        } else {
            panic!("Couldn't get pawn location: couldn't find player with that color")
        }
    }

    pub fn all_pawns_entered(&self, color: &Color) -> bool {
        if let Some(pawn_locs) = self.positions.get(color) {
            for i in 0..4 {
                if let Loc::Nest = pawn_locs[i] {
                    return false;
                };
            }
        } else {
            panic!("THERE SHOULD BE PAWN LOCATIONS");
        }
        // It okay for a pawn to be at home.
        true
    }

    pub fn is_safety(&self, location: Loc) -> bool {
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

    pub fn is_home_row(&self, color: Color, location: Loc) -> bool {
        let home_row_entrance_index = self.get_home_row_entrance(color);

        let current_location = match location {
            Loc::Spot { index } => index,
            _ => panic!(" "),
        };

        current_location < home_row_entrance_index + HOME_ROW_LENGTH
    }

    pub fn get_main_ring_exit(&self, color: Color) -> usize {
        let entrance = self.get_entrance(color);
        (entrance - EXIT_TO_ENTRANCE) % BOARD_SIZE
    }

    pub fn get_entrance(&self, color: Color) -> usize {
        match color {
            Color::Red => RED_ENTRANCE,
            Color::Blue => BLUE_ENTRANCE,
            Color::Yellow => YELLOW_ENTRANCE,
            Color::Green => GREEN_ENTRANCE,
        }
    }

    pub fn get_home_row_entrance(&self, color: Color) -> usize {
        match color {
            Color::Red => RED_HOME_ROW,
            Color::Blue => BLUE_HOME_ROW,
            Color::Yellow => YELLOW_HOME_ROW,
            Color::Green => GREEN_HOME_ROW,
        }
    }

    pub fn can_bop(&self,
                   bopper_color: Color,
                   dest_index: usize)
                   -> Option<Pawn> {
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

    // The main ring is absolutely indexed, but the
    // Home Row entrances for each player are not continuous,
    // e.g. Green enters at 89, Red at 68, etc.
    // To compute whether the move should wrap into
    // the home row, we need to check whether the move would
    // go past the end of the main ring for the player.
    fn compute_main_ring_dest(&self,
                              color: Color,
                              start: usize,
                              distance: usize)
                              -> usize {
        let home_row_entrance: usize = self.get_home_row_entrance(color);
        let exit: usize = self.get_main_ring_exit(color);

        if start + distance > exit {
            let offset = (start + distance) - exit - 1;
            println!("color: {:#?}", color);
            println!("start: {}", start);
            println!("distance: {}", distance);
            println!("exit: {}", exit);
            println!("offset: {}", offset);
            println!("homerowentrance: {}", home_row_entrance);
            offset + home_row_entrance
        } else {
            start + distance
        }
    }

    /// Takes a move and returns a new board.
    pub fn handle_move(&self, m: Move) -> Result<MoveResult, &'static str> {
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

                        bonus = Some(BOP_BONUS);
                    }

                    println!("color: {:#?}", color);
                    println!("start: {}", start);
                    println!("distance: {}", distance);
                    let dest: usize =
                        self.compute_main_ring_dest(color, start, distance);
                    Loc::Spot { index: dest }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// To make testing less verbose, we can express a board in terms of
    /// the difference in pawn positions from an initial board.
    type BoardPosnDiff = BTreeMap<Color, PawnLocs>;

    /// Helper utility for testing that moves produce expected boards.
    fn move_tester(start: BoardPosnDiff,
                   m: Move,
                   expected: BoardPosnDiff,
                   expected_bonus: Option<Bonus>)
                   -> Result<MoveResult, &'static str> {
        let mut start_board = Board::new();
        let mut expected_board = Board::new();

        // Mutate the initial boards according to the start and expected
        // BoardPosnDiffs,
        for (clr, locs) in start.into_iter() {
            start_board
                .positions
                .insert(clr, locs);
        }
        for (clr, locs) in expected.into_iter() {
            expected_board
                .positions
                .insert(clr, locs);
        }

        match start_board.handle_move(m) {
            Ok(MoveResult(actual_board, bonus)) => {
                // Check that the actual bonus corresponds to the expected
                // bonus.
                println!("{:#?}", actual_board);
                assert_eq!(bonus, expected_bonus);
                assert_eq!(actual_board, expected_board);
                Ok(MoveResult(actual_board, bonus))
            }
            // If handling the move threw an error, our test should fail.
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
                Err(e)
            }
        }
    }

    /// Testing Conventions:
    ///
    /// - b0: starting BoardPosnDiff
    /// - m1: first dispatched Move
    /// - b<#>: the BoardPosnDiff that results from calling
    ///         b<#-1>.handle_move(m<#>)
    ///
    /// Always clone boards the first time they are used if needed,
    /// Always declare in the order: b0, m1, b1, (test), m2, b2, (test), etc.

    /// Pawn color comparison should work as intended.
    fn test_pawn_colors() {
        let y1 = Pawn::new(1, Color::Yellow);
        let r1 = Pawn::new(1, Color::Red);
        let r2 = Pawn::new(2, Color::Red);

        assert_ne!(y1.color, r2.color);
        assert_eq!(r1.color, r2.color);
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
    }

    //////////////////////////
    // Basic Movement Tests
    //////////////////////////

    #[test]
    // Testing basic movements (single pawn, no other players, no bonuses).
    fn test_move_piece() {
        let b0: BoardPosnDiff = map!{
            Color::Green => [Loc::Spot { index: GREEN_ENTRANCE },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest]
        };
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
        let b1 = map!{
            Color::Green => [Loc::Spot { index: 54 },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest]
        };
        move_tester(b0, m1, b1.clone(), None);

        let m2 = Move {
            m_type: MoveType::MoveMain {
                start: 54,
                distance: 4,
            },
            pawn: p.clone(),
        };
        let b2 = map!{
            Color::Green => [Loc::Spot { index: 58 },
            Loc::Nest,
            Loc::Nest,
            Loc::Nest]
        };
        move_tester(b1, m2, b2.clone(), None);

        let m3 = Move {
            m_type: MoveType::MoveMain {
                start: 58,
                distance: 6,
            },
            pawn: p.clone(),
        };
        let b3 = map!{
            Color::Green => [Loc::Spot { index: 64 },
                               Loc::Nest,
                               Loc::Nest,
                               Loc::Nest]
        };
        move_tester(b2, m3, b3, None);
    }

    #[test]
    /// Moving from main ring into home row.
    fn test_move_into_home_row() {
        let b0: BoardPosnDiff = map!{
           Color::Green => [Loc::Spot { index: 43 },
                              Loc::Nest,
                              Loc::Nest,
                              Loc::Nest]
       };
        let p = Pawn {
            color: Color::Green,
            id: 0,
        };
        let move_four = Move {
            m_type: MoveType::MoveMain {
                start: 43,
                distance: 4,
            },
            pawn: p,
        };
        let result = map!{
           Color::Green => [Loc::Spot { index: 89 },
                              Loc::Nest,
                              Loc::Nest,
                              Loc::Nest]
       };
        move_tester(b0, move_four, result.clone(), None);
    }

    #[test]
    #[ignore]
    /// Moving from home row to home row.
    fn test_move_on_home_row() {}

    #[test]
    #[ignore]
    /// Moving home should award a bonus.
    fn test_move_home_bonus() {}

    #[test]
    #[ignore]
    /// Cannot move if no piece is present on the square.
    fn test_move_no_piece_present() {}
}
