#![allow(dead_code, unused_variables)]

use std::collections::BTreeMap;
use super::game::{Move, MoveType};
use super::constants::*;
use super::dice::Dice;



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
    pub positions: BTreeMap<Color, PawnLocs>,
}

impl Board {
    // Initialize a new game board in the starting configuration,
    // i.e. all pawns are in their respective nests.
    pub fn new() -> Board {
        let mut positions: BTreeMap<Color, PawnLocs> = BTreeMap::new();
        let init_pawn_locs: PawnLocs = [Loc::Nest; 4];

        for clr in COLORS.iter() {
            for i in 0..4 {
                positions.insert(clr.clone(), init_pawn_locs.clone());
            }
        }

        Board { positions: positions }
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
                    Loc::Spot { index: Board::get_entrance(&color) }
                }
                MoveType::MoveHome { start, distance } => {
                    // Pawns may only move home by an exact amount.
                    let home_row_entrance = Board::get_home_row(&color);
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

    /// Determines whether the turn resulting in the given board is valid
    /// with respect to the current board.
    pub fn is_valid_turn(&self,
                         end: &Board,
                         dice: &Dice,
                         color: Color)
                         -> bool {
        // Check no valid moves remaining.
        if Board::has_valid_moves(end, dice, &color) {
            return false;
        }
        // Can't move blockades together.
        if let Some(pawn_locs) = self.positions.get(&color) {
            // pawn_locs = []
            let blockades: Vec<Loc> = self.get_blockades();

            for &blockade_loc in blockades.iter() {
                // Get the ids of the pawns that formed the blockade
                // at this location.
                let blockade_pawn_ids = pawn_locs
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|&(_, loc)| loc == blockade_loc)
                    .collect::<Vec<(usize, Loc)>>();

                // If the new locations of the pawns are the same,
                // the turn is invalid.
                let new_loc_1 =
                    end.get_pawn_loc(&color, blockade_pawn_ids[0].0);
                let new_loc_2 =
                    end.get_pawn_loc(&color, blockade_pawn_ids[1].0);

                if new_loc_1 == new_loc_2 {
                    return false;
                }
            }
            return true;
        } else {
            panic!("Couldn't get pawns for color");
        }
    }

    /// Determines whether the given board, dice, and color has any valid moves left.
    pub fn has_valid_moves(board: &Board, dice: &Dice, color: &Color) -> bool {
        if dice.all_used() {
            return false;
        }
        if let Some(pawn_locs) = board.positions.get(color) {
            let valid_for_roll = |&r| {
                pawn_locs
                    .iter()
                    .enumerate()
                    .any(|(i, loc)| {
                        let m_type: MoveType = match *loc {
                            Loc::Spot { index } => {
                                if Board::is_home_row(*color,
                                                      Loc::Spot {
                                                          index: index,
                                                      }) {
                                    MoveType::MoveHome {
                                        start: index,
                                        distance: r,
                                    }
                                } else {
                                    MoveType::MoveMain {
                                        start: index,
                                        distance: r,
                                    }
                                }
                            }
                            _ => return false,
                        };
                        let m = Move {
                            m_type: m_type,
                            pawn: Pawn {
                                color: *color,
                                id: i,
                            },
                        };
                        Board::is_valid_move(board, dice, &m)
                    })
            };
            dice.rolls
                .iter()
                .any(valid_for_roll)
        } else {
            panic!("Couldn't get pawn locations for player");
        }
    }

    pub fn is_valid_move(board: &Board, dice: &Dice, m: &Move) -> bool {
        let Move { pawn, m_type } = *m;
        let Pawn { color, id } = pawn;

        match m.m_type {
            MoveType::EnterPiece => {
                let all_pawns_entered = board.all_pawns_entered(&color);
                let home_row_entrance = Board::get_home_row(&color);
                let is_blockade =
                    board
                        .get_blockades()
                        .contains(&Loc::Spot { index: home_row_entrance });
                all_pawns_entered && is_blockade
            }
            MoveType::MoveMain { start, distance } => {
                // Pawn is currently at start location in the Main Ring.
                let current_pawn_loc: Loc =
                    board.get_pawn_loc(&pawn.color, pawn.id);
                let start_loc: Loc = Loc::Spot { index: start };
                if current_pawn_loc != start_loc {
                    return false;
                }

                // Chosen move distance is a valid mini-move.
                if !dice.contains(&distance) {
                    return false;
                }

                // Don't let the pawn go through any blockades on their
                // way to the destination.
                let blockades: Vec<Loc> = board.get_blockades();
                for i in 0..distance {
                    let end = Board::get_exit(&color);
                    // pawns should wrap into their home row
                    // We have to this because we are using absolute addressing
                    // and some pawn's home rows may not be the next number
                    // after the end of the board
                    // If red is at 60, and it rolls a 5
                    // it would proceed 61,62,63,68,69
                    //                           ^ is the home row entrance
                    let is_past_end = start + i >
                                      (Board::get_entrance(&color) -
                                       EXIT_TO_ENTRANCE) %
                                      BOARD_SIZE;
                    let offset = if is_past_end { end } else { start };

                    let current_spot: Loc = Loc::Spot { index: offset + i };
                    let blockades: Vec<Loc> = board.get_blockades();
                    if blockades.contains(&current_spot) {
                        return false;
                    }
                }
                true
            }

            MoveType::MoveHome { start, distance } => {
                // Main Ring.
                // Pawn is currently at start location in the Main Ring.
                let current_pawn_loc: Loc =
                    board.get_pawn_loc(&pawn.color, pawn.id);
                let start_loc: Loc = Loc::Spot { index: start };
                if current_pawn_loc != start_loc {
                    return false;
                }

                // Chosen move distance is a valid mini-move.
                if !dice.contains(&distance) {
                    return false;
                }

                // Don't let the pawn go through any blockades on their
                // way to the destination.
                for i in 0..distance {
                    let end = Board::get_exit(&color);
                    // pawns should wrap into their home row
                    // We have to this because we are using absolute addressing
                    // and some pawn's home rows may not be the next number
                    // after the end of the board
                    // If red is at 60, and it rolls a 5
                    // it would proceed 61,62,63,68,69
                    //                           ^ is the home row entrance
                    let is_past_end = start + i >
                                      (Board::get_entrance(&color) -
                                       EXIT_TO_ENTRANCE) %
                                      BOARD_SIZE;
                    let offset = if is_past_end { end } else { start };
                    let current_spot: Loc = Loc::Spot { index: offset + i };
                    let blockades: Vec<Loc> = board.get_blockades();
                    if blockades.contains(&current_spot) {
                        return false;
                    }
                }

                // Allows us to see if the move is overshooting the home
                let overshoot = Board::get_home_row(&color) + HOME_ROW_LENGTH;
                if start + distance > overshoot {
                    return false;
                }

                true
            }
        }
    }

    // Takes a move and generates the sequence of board locations
    // corresponding to the pawn's travel.
    fn move_path(m: Move) -> Vec<Loc> {
        vec![Loc::Nest]
    }

    // Associated helper function to compute the next (start + 1) location
    // for a pawn's movement, based on its color.
    //
    // Note that this function does NOT have access to the board state.
    // It's merely used to abstract over the unfortunate arithmetic from
    // absolute indexing.
    fn next_loc(color: &Color, start: &Loc) -> Result<Loc, &'static str> {
        let entrance: usize = Board::get_entrance(&color);
        let home_row: usize = Board::get_home_row(&color);
        let exit: usize = Board::get_home_row(&color);

        let result: Loc = match *start {
            // If we start from Home, there is no next location -- that would
            // be overshooting.
            Loc::Home => return Err("Can't overshoot home"),

            // If we start from the Nest, the next location is the player's
            // entrance.
            Loc::Nest => Loc::Spot { index: entrance },

            // If we start from a spot on the board, need more information
            // about the spot, so we match again on the index.
            Loc::Spot { index } => {
                match index {
                    // If we are on the main ring exit, enter the home row.
                    ex if ex == exit => Loc::Spot { index: home_row },

                    // If we are anywhere in home row, one of the following
                    // occurs:
                    // (1) Increment by 1, a regular move
                    // (2) Land home
                    // (3) Overshoot home (this case may be redundant?)
                    hr if hr >= home_row => {
                        let home: usize = home_row + HOME_ROW_LENGTH;
                        let dest: usize = index + 1;

                        match dest {
                            x if x > home => return Err("Can't overshoot home"),
                            h if h == home => Loc::Home,
                            _ => Loc::Spot { index: dest },
                        }
                    }

                    // Otherwise, it's just an ordinary move on the main ring.
                    // Return the next location, modulo the board size.
                    _ => Loc::Spot { index: index + 1 % BOARD_SIZE },
                }
            }

        };

        Ok(result)
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
    pub fn get_pawn_loc(&self, color: &Color, id: usize) -> Loc {
        if let Some(locs) = self.positions.get(color) {
            locs[id]
        } else {
            panic!("Couldn't get pawn location: couldn't find player with that color")
        }
    }

    /// Returns whether all the pawns for a given player have left
    /// the nest.
    pub fn all_pawns_entered(&self, color: &Color) -> bool {
        if let Some(locs) = self.positions.get(&color) {
            locs.iter()
                .all(|&loc| loc != Loc::Nest)
        } else {
            panic!("Couldn't find pawns for given player");
        }
    }

    /// Associated function to check whether a Loc is home row.
    pub fn is_home_row(color: Color, loc: Loc) -> bool {
        if let Loc::Spot { index } = loc {
            index >= Board::get_home_row(&color)
        } else {
            false
        }
    }

    /// Associated function to check whether a Loc is a safety spot.
    pub fn is_safety(loc: Loc) -> bool {
        if let Loc::Spot { index } = loc {
            SAFETY_SPOTS.contains(&index)
        } else {
            false
        }
    }

    /// Associated function to return the exit index for a player.
    pub fn get_exit(color: &Color) -> usize {
        let entrance = Board::get_entrance(&color);
        (entrance - EXIT_TO_ENTRANCE) % BOARD_SIZE
    }

    /// Associated function to return the entry index for a player.
    pub fn get_entrance(color: &Color) -> usize {
        match *color {
            Color::Red => RED_ENTRANCE,
            Color::Blue => BLUE_ENTRANCE,
            Color::Yellow => YELLOW_ENTRANCE,
            Color::Green => GREEN_ENTRANCE,
        }
    }

    /// Associated function to return the home row entrance index for a player.
    pub fn get_home_row(color: &Color) -> usize {
        match *color {
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
        let bopper_entrance = Board::get_entrance(&bopper_color);

        if Board::is_safety(dest_loc) && dest_index != bopper_entrance {
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
        let home_row_entrance: usize = Board::get_home_row(&color);
        let exit: usize = Board::get_exit(&color);

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
    fn move_from_entrance() {
        // ['red', 4, 5, 9, 'enter and move 5'],
        // ['green', 55, 5, 60, 'enter and move 5'],
        // ['yellow', 38, 5, 43, 'enter and move 5\n\n'],
        // ['blue', 21, 5, 26, 'enter and move 5\n\n'],

        let start_indices = [4, 21, 38, 55];
        let end_indices = [9, 26, 43, 60];
        let posns = map!{
            Color::Red => [Loc::Spot { index: 4 }, Loc::Nest, Loc::Nest, Loc::Nest],
            Color::Blue => [Loc::Spot { index: 21 }, Loc::Nest, Loc::Nest, Loc::Nest],
            Color::Yellow => [Loc::Spot { index: 38 }, Loc::Nest, Loc::Nest, Loc::Nest],
            Color::Green => [Loc::Spot { index: 55 }, Loc::Nest, Loc::Nest, Loc::Nest]
        };
        let board = Board::from(posns);
        let moves = COLORS
            .iter()
            .enumerate()
            .map(|(i, color)| {
                Move {
                    m_type: MoveType::MoveMain {
                        start: start_indices[i],
                        distance: 5,
                    },
                    pawn: Pawn {
                        id: 0,
                        color: *color,
                    },
                }
            });
        let dice = Dice {
            rolls: vec![5],
            used: vec![],
        };

        assert!(moves.all(|mv| Board::is_valid_move(&board, &dice, &mv)));
    }

    #[test]
    #[ignore]
    fn main_ring_wrap_index() {
        // ['blue', 66, 5, 3, 'move 5 on main ring, wrap around'],
        // ['green', 66, 5, 3, 'move 5 on main ring, wrap around'],
        // ['yellow', 64, 5, 1, 'move 5 on main ring, wrap around\n\n'],
    }

    #[test]
    #[ignore]
    fn main_ring_to_home_row() {
        // ['red', 66, 5, 103, 'move from main ring to home row'],
        // ['yellow', 33, 1, 300, 'move from main ring to home row'],
        // ['green', 45, 6, 400, 'move from main ring to home row'],
        // ['blue', 13, 6, 202, 'move from main ring to home row\n\n'],
    }

    #[test]
    #[ignore]
    fn main_ring_bonus() {
        // ['yellow', 38, 10, 48, 'take a 10 point bonus on the main ring\n\n'],
    }


    #[test]
    #[ignore]
    fn move_within_home_row() {
        // ['red', 100, 5, 105, 'move legally within home row'],
        // ['green', 400, 3, 403, 'move legally within home row\n\n'],
        // ['yellow', 301, 6, 'Home', 'move legally within home row\n\n'],
        // ['blue', 203, 4, 'Home', 'move legally within home row\n\n'],
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn cannot_overshoot_home_from_home_row() {
        // ['red', 100, 20, 'Bad', 'move illegally within home row'],
        // ['green', 404, 6, 'Bad', 'move illegally within home row\n\n'],
        // ['yellow', 304, 6, 'Bad', 'move illegally within home row\n\n'],
        // ['blue', 203, 5, 'Bad', 'move illegally within home row\n\n'],
    }


    #[test]
    #[ignore]
    #[should_panic]
    fn cannot_overshoot_home_with_bonus() {
        // ['red', 64, 20, 'Bad', 'wrap illegally into home row'],
        // ['green', 49, 10, 'Bad', 'wrap illegally into home row\n\n'],
        // ['yellow', 28, 20, 'Bad', 'wrap illegally into home row\n\n'],
        // ['blue', 16, 10, 'Bad', 'wrap illegally into home row\n\n'],
    }


    #[test]
    #[ignore]
    fn main_to_home_row_with_bonus() {
        // ['yellow', 30, 10, 306, 'take a 10 point bonus and wrap to home row\n\n'],
        // ['red', 64, 10, 106, 'take a 10 point bonus and wrap to home row\n\n'],
        // ['blue', 1, 20, 204, 'take a 20 point bonus and wrap to home row\n\n'],
        // ['green', 36, 20, 405, 'take a 20 point bonus and wrap to home row\n\n'],
    }


    #[test]
    #[ignore]
    fn main_to_home_with_bonus() {
        // ['blue', 4, 20, 'Home', 'take a 20 bonus and land at home'],
    }
}
