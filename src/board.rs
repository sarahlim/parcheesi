#![allow(dead_code)]
use std;
use std::fmt;
use std::collections::BTreeMap;
use super::game::{Move, MoveType};
use super::constants::*;
use super::dice::{Dice, EntryMove};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Represents the location of a pawn.
pub enum Loc {
    Spot { index: usize },
    Nest,
    Home,
}

/// Represents a sequence of locations for each player.
#[derive(Debug, Copy, Clone)]
pub struct Path {
    color: Color,
    current_loc: Loc,
    has_started: bool,
}

impl Path {
    pub fn new(color: Color) -> Path {
        Path {
            color: color,
            current_loc: Loc::Nest,
            has_started: false,
        }
    }

    pub fn started(color: Color, start: Loc) -> Path {
        Path {
            color: color,
            current_loc: start,
            has_started: true,
        }
    }
}

impl Iterator for Path {
    type Item = Loc;

    /// Return an Option<Loc> denoting the next location.
    fn next(&mut self) -> Option<Loc> {
        // To get the iterator to include its starting location, we use
        // a hacky indicator boolean to denote the first time next() is called.
        if !self.has_started {
            self.has_started = true;
            return Some(Loc::Nest);
        }

        // Compute the next location in the path, based on the path color.
        let entrance: usize = Board::get_entrance(&self.color);
        let home_row: usize = Board::get_home_row(&self.color);
        let exit: usize = Board::get_exit(&self.color);

        let result: Option<Loc> = match self.current_loc {
            // If we start from Home, there is no next location -- that would
            // be overshooting.
            Loc::Home => None,

            // If we start from the Nest, the next location is the player's
            // entrance.
            Loc::Nest => Some(Loc::Spot { index: entrance }),

            // If we start from a spot on the board, need more information
            // about the spot, so we match again on the index.
            Loc::Spot { index } => {
                match index {
                    // If we are on the main ring exit, enter the home row.
                    ex if ex == exit => Some(Loc::Spot { index: home_row }),

                    // If we are anywhere in home row, one of the following
                    // occurs:
                    // (1) Increment by 1, a regular move
                    // (2) Land home
                    // (3) Overshoot home (this case may be redundant?)
                    hr if hr >= home_row => {
                        let home: usize = home_row + HOME_ROW_LENGTH;
                        let dest: usize = index + 1;

                        match dest {
                            x if x > home => None,  // Can't overshoot home
                            h if h == home => Some(Loc::Home),
                            _ => Some(Loc::Spot { index: dest }),
                        }
                    }

                    // Otherwise, it's just an ordinary move on the main ring.
                    // Return the next location, modulo the board size.
                    _ => Some(Loc::Spot { index: (index + 1) % BOARD_SIZE }),
                }
            }
        };

        if let Some(next_loc) = result {
            // Update the current instance.
            self.current_loc = next_loc;
        }

        result
    }
}

pub type PawnLocs = [Loc; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a color of a Pawn or Player.b
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
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

    pub fn xmlify(&self) -> String {
        let xml_response =
            "<pawn> <color> ".to_string() + &self.color.to_string().to_lowercase() +
            " </color> <id> " + &self.id.to_string() +
            " </id> </pawn>";
        xml_response
    }
}

pub struct MoveResult(pub Board, pub Option<Bonus>);

type Bonus = usize;

/// To make testing less verbose, we can express a board in terms of
/// the difference in pawn positions from an initial board.
type BoardPosnDiff = BTreeMap<Color, PawnLocs>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a board, containing the positions of all pawns.
pub struct Board {
    pub positions: BTreeMap<Color, PawnLocs>,
}

impl Board {
    /// Initialize a new game board in the starting configuration,
    /// i.e. all pawns are in their respective nests.
    pub fn new() -> Board {
        let mut positions: BTreeMap<Color, PawnLocs> = BTreeMap::new();
        let init_pawn_locs: PawnLocs = [Loc::Nest; 4];

        for clr in COLORS.iter() {
            for _ in 0..4 {
                positions.insert(clr.clone(), init_pawn_locs.clone());
            }
        }

        Board { positions: positions }
    }

    /// Alternate constructor for a board, which takes a partial map of positions
    /// and merges with the default board.
    pub fn from(posns: BoardPosnDiff) -> Board {
        let board: Board = Board::new();
        let mut positions: BTreeMap<Color, PawnLocs> = board.positions.clone();

        for (clr, &locs) in posns.iter() {
            positions.insert(clr.clone(), locs);
        }

        Board { positions: positions }
    }

    /// Takes a move and returns a new board.
    pub fn handle_move(&self, m: Move) -> Result<MoveResult, &'static str> {
        let Move {
            pawn: Pawn { color, id },
            m_type,
        } = m;

        let mut next_positions = self.positions.clone();
        let mut bonus = None;

        // The color of the pawn being moved tells us
        // which player's pawn locations we need to modify.
        let player_pawns: PawnLocs = self.get_pawns_by_color(&color);
        let mut next_pawns = player_pawns.clone();

        let next_loc: Loc = match m_type {
            MoveType::EnterPiece => {
                Loc::Spot { index: Board::get_entrance(&color) }
            }
            MoveType::MoveHome { start, distance } |
            MoveType::MoveMain { start, distance } => {
                let start_loc: Loc = Loc::Spot { index: start };
                let mut move_path: Vec<Loc> = Path::started(color, start_loc)
                    .take(distance)
                    .collect();

                let next_loc: Loc = match move_path.pop() {
                    Some(loc) => loc,
                    None => panic!("Couldn't get move end"),
                };

                if next_loc == Loc::Home {
                    bonus = Some(HOME_BONUS);
                }

                if let Some(bopped) = self.can_bop(color, next_loc) {
                    // If a bop occurs, we need to handle two side effects:
                    // 1. Move the bopped pawn back to the nest.
                    let bopped_pawn_locs: PawnLocs =
                        self.get_pawns_by_color(&bopped.color);
                    let mut next_bopped_pawns: PawnLocs = bopped_pawn_locs
                        .clone();
                    next_bopped_pawns[bopped.id] = Loc::Nest;
                    next_positions.insert(bopped.color, next_bopped_pawns);

                    // 2. Award the bop bonus.
                    bonus = Some(BOP_BONUS);
                }

                next_loc
            }
        };

        // Modify the copy of the position map with the next_position
        // of the moving player's pawns.
        next_pawns[id] = next_loc;
        next_positions.insert(color, next_pawns);
        let result: MoveResult =
            MoveResult(Board { positions: next_positions }, bonus);

        Ok(result)
    }

    /// Determines blockades were moved together.
    pub fn is_valid_turn (&self, 
                         end: &Board,
                         dice: &Dice,
                         color: Color)
                         -> bool {
        // Can't move blockades together.
        let pawns: PawnLocs = self.get_pawns_by_color(&color);
        let blockades: Vec<Loc> = self.get_blockades();

        for &blockade_loc in blockades.iter() {
            // Get the ids of the pawns that formed the blockade
            // at this location.
            let blockade_pawn_ids = pawns
                .iter()
                .cloned()
                .enumerate()
                .filter(|&(_, loc)| loc == blockade_loc)
                .collect::<Vec<(usize, Loc)>>();

            // If the new locations of the pawns are the same,
            // the turn is invalid.
            let new_loc_1 = end.get_pawn_loc(&color, blockade_pawn_ids[0].0);
            let new_loc_2 = end.get_pawn_loc(&color, blockade_pawn_ids[1].0);

            if new_loc_1 == new_loc_2 {
                return false;
            }
        }

        // If we got through all of the pawns and there aren't any
        // blockades moved together, the turn is valid.
        true
    }

    /// As with other xmlify methods, we wish to return the xml representation of the board
    /// where a board is represented as <board> start main home-rows home </board>. Each of these
    /// pieces of the representation is represented by a piece-loc, which is defined as <piece-loc> pawn <loc> number </loc> </piece-loc>
    pub fn xmlify(&self) -> String {
        let mut start_string: String = "<start>".to_string();
        let mut main_row_string: String = "<main>".to_string();
        let mut home_row_string: String = "<home-rows>".to_string();
        let mut home_string: String = "<home>".to_string();

        let posns: BTreeMap<Color, PawnLocs> = self.positions.clone();

        for (clr, &locs) in posns.iter() {
            for (id, loc) in locs.iter()
                    .cloned()
                    .enumerate() {
                let pawn: Pawn = Pawn {
                    id: id,
                    color: *clr,
                };
                match loc {
                    Loc::Nest => {
                        start_string = start_string + " " + &pawn.xmlify();
                    }
                    Loc::Home => {
                        home_string = home_string + " " + &pawn.xmlify();
                    }
                    Loc::Spot { index } => {
                        if index > Board::get_exit(clr) {
                            home_row_string = home_row_string +
                                              " <piece-loc> " +
                                              &pawn.xmlify() +
                                              " <loc> " +
                                              &index.to_string() +
                                              " </loc> </piece-loc>";
                        } else {
                            main_row_string = main_row_string +
                                              " <piece-loc> " +
                                              &pawn.xmlify() +
                                              " <loc> " +
                                              &index.to_string() +
                                              " </loc> </piece-loc>";
                        }
                    }
                }
            }
        }
        "<board> ".to_string() + &start_string + " </start> " +
        &main_row_string + " </main> " + &home_row_string +
        " </home-rows> " + &home_string + " </home> </board>"
    }




    /// Sort a player's pawns according to their position relative to the player's
    /// path around the board.
    pub fn sort_player_locs(color: &Color,
                            locs: PawnLocs)
                            -> Vec<(usize, Loc)> {
        // Clone the list of locations so we can sort it.
        let mut loc_vec = locs.iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, Loc)>>();

        // Generate the path for the player with the given color.
        // This allows us to sort locations with respect to the path, which
        // is not in numeric order.
        let path: Vec<Loc> = Path::new(*color).collect();

        // This closure uses the generated path to get the ordinal index of a
        // pawn's location along its path.
        let pawn_loc_ordinal = |&pawn_loc| -> usize {
            let is_pawn_loc = |path_loc| path_loc == pawn_loc;
            let position: Option<usize> = path.clone()
                .into_iter()
                .position(is_pawn_loc);

            match position {
                Some(index) => index,
                None => panic!("Pawn {:?} is out of bounds", pawn_loc),
            }
        };

        loc_vec.sort_by_key(|&(_, loc)| pawn_loc_ordinal(&loc));
        loc_vec
    }

    /// Determines whether the given board, dice, and color has any valid moves left.
    /// TODO: Does this preserve legality within turns?
    pub fn has_valid_moves(board: &Board, dice: &Dice, color: &Color) -> bool {
        // Check 

        
        if dice.all_used() {
            return false;
        }

        let pawns: PawnLocs = board.get_pawns_by_color(color);

        // Takes a roll, and checks whether any of the
        // pawns are eligible to move by that distance.
        let valid_for_roll = |&r| -> bool {

            // Helper function takes a pawn and its location,
            // and checks whether that pawn is eligible to move
            // the distance given by the outer closure.
            let build_move_and_check =
                |(pawn_id, &loc): (usize, &Loc)| -> bool {
                    let pawn = Pawn {
                        color: *color,
                        id: pawn_id,
                    };

                    let m_type = match loc {
                        Loc::Spot { index } => {
                            if Board::is_home_row(*color, loc) {
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
                        // Early return false if the pawn is Home or in the Nest,
                        // because it can't take a move.
                        _ => return false, 
                    };

                    let mv = Move {
                        m_type: m_type,
                        pawn: pawn,
                    };

                    Board::is_valid_move(board, dice, &mv)
                };

            // Finally iterate over all pawns and check whether they
            // would form a valid move.
            pawns
                .iter()
                .enumerate()
                .any(build_move_and_check)
        };

        dice.rolls
            .iter()
            .any(valid_for_roll)
    }

    /// Determines whether an individual mini-move is valid, given some board and dice.
    pub fn is_valid_move(board: &Board, dice: &Dice, m: &Move) -> bool {
        let Move { pawn, m_type } = *m;
        let Pawn { color, id } = pawn;

        match m_type {
            MoveType::EnterPiece => {
                // EnterPiece is valid iff
                // - Dice fulfill conditions for entering
                // - Entered pawn was formerly at nest
                // - No blockades on player's entrance
                let is_dice_valid: bool = match dice.can_enter() {
                    EntryMove::NoEntry => false,
                    _ => true,
                };

                println!("is_dice_valid: {}", is_dice_valid);

                let pawn_in_nest: bool = match board.get_pawn_loc(&color, id) {
                    Loc::Nest => true,
                    _ => false,
                };

                println!("pawn in nest: {}", pawn_in_nest);
                let entrance = Board::get_entrance(&color);
                let is_entrance_blockaded =
                    board
                        .get_blockades()
                        .contains(&Loc::Spot { index: entrance });

                println!("is entrance blockaded: {}", is_entrance_blockaded);

                is_dice_valid && pawn_in_nest && !is_entrance_blockaded
            }
            MoveType::MoveMain { start, distance } |
            MoveType::MoveHome { start, distance } => {
                let start_loc: Loc = Loc::Spot { index: start };
                let finish_loc: Loc = Loc::Spot { index: start + distance };

                // Pawn is currently at start location in the Main Ring.
                let current_pawn_loc: Loc = board.get_pawn_loc(&color, id);
                if current_pawn_loc != start_loc {
                    return false;
                }
                // This will return false when the pawn attempts to bop on a safety square
                if Board::is_safety(finish_loc) &&
                   board.full_safety_square(finish_loc, color) {
                    return false;
                }

                // Chosen move distance is a valid mini-move.
                if !dice.contains(&distance) {
                    return false;
                }

                // Check for blockades along the path.
                let blockades: Vec<Loc> = board.get_blockades();
                let mut move_path = Path::started(color.clone(), start_loc)
                    .take(distance);
                let has_blockade_on_path: bool =
                    move_path.any(|path_loc| blockades.contains(&path_loc));

                if has_blockade_on_path {
                    return false;
                }

                // Make sure we don't overshoot home.
                let home_row_entrance: usize = Board::get_home_row(&color);
                if start + distance > home_row_entrance + HOME_ROW_LENGTH {
                    return false;
                }

                true
            }
        }
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

    /// Checks to see if a location has a pawn there already
    pub fn full_safety_square(&self, dest_loc: Loc, color: Color) -> bool {
        //TODO some how make this not reuse code from can bop
        let is_dest = |l: Loc| l == dest_loc;
        for (&c, locs) in self.positions.iter() {
            if color == c {
                continue;
            }
            // We are looking for locations of opponents pawns on the given space
            let occupants: Vec<(usize, Loc)> = locs.iter()
                .cloned()
                .enumerate()
                .filter(|&(_, loc)| is_dest(loc))
                .collect();

            if !occupants.is_empty() {
                return true;
            }
        }
        false
    }

    /// Returns a list of all blockades on the board.
    pub fn get_blockades(&self) -> Vec<Loc> {
        let blockades_for_color = |locs: PawnLocs| {
            let mut blockades: Vec<Loc> = Vec::new();
            let mut seen: Vec<Loc> = Vec::new();
            for loc in locs.iter() {
                if *loc != Loc::Nest && *loc != Loc::Home {
                    if seen.contains(loc) {
                        blockades.push(*loc);
                    } else {
                        seen.push(*loc);
                    }
                }
            }
            blockades
        };

        self.positions
            .iter()
            .map(|(_, &locs)| blockades_for_color(locs))
            .fold(Vec::new(), |mut memo, mut b| {
                memo.append(&mut b);
                memo
            })
    }

    /// Get a reference to all the pawns for a given player.
    pub fn get_pawns_by_color(&self, color: &Color) -> PawnLocs {
        match self.positions.get(&color) {
            Some(pawns) => *pawns,
            None => panic!("Couldn't get pawns for color"),
        }
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
        match *color {
            Color::Red => RED_EXIT,
            Color::Blue => BLUE_EXIT,
            Color::Yellow => YELLOW_EXIT,
            Color::Green => GREEN_EXIT,
        }
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

    /// Determine whether a player can bop some pawn on a given destination spot.
    pub fn can_bop(&self, bopper_color: Color, dest_loc: Loc) -> Option<Pawn> {
        // A pawn can bop if all of the following are true:
        // - dest index is not a safety spot,
        // - dest contains one pawn of a different color.

        // 1. If dest_index is safety,
        //    a. bopper's entrance => MIGHT BE ABLE TO BOP, KEEP CHECKING
        //    b. any other safety => CANNOT BOP
        let bopper_entrance_index: usize = Board::get_entrance(&bopper_color);
        let bopper_entrance: Loc = Loc::Spot { index: bopper_entrance_index };

        if Board::is_safety(dest_loc) && dest_loc != bopper_entrance {
            return None;
        }

        // 2. If spot is not a safety, check if it's occupied
        //    a. Occupied by opponent
        //       i. Blockade => CANNOT BOP
        //       ii. Not blockade => CAN BOP
        //    b. Unoccupied => CANNOT BOP
        let is_dest = |l: Loc| l == dest_loc;
        let blockades: Vec<Loc> = self.get_blockades();
        let has_blockade = |l: Loc| blockades.contains(&l);

        for (c, locs) in self.positions.iter() {
            if *c == bopper_color {
                continue;
            }

            // We're looking at the pawn locations of an opponent.
            // Iterate over those and check for opponent pawns.
            let mut occupants: Vec<(usize, Loc)> = locs.iter()
                .cloned()
                .enumerate()
                .filter(|&(_, loc)| is_dest(loc) && !has_blockade(loc))
                .collect();

            // Now `occupants` is a vector of the current opponent's
            // pawns occupying the destination spot.
            if !occupants.is_empty() {
                // Should be exactly one occupant.
                assert_eq!(occupants.len(), 1);

                let (id, _) = occupants.pop().unwrap();
                let bopped = Pawn { id: id, color: *c };

                return Some(bopped);
            }
        }

        // If we got through all other players' positions and found no
        // single-occupants, there is nothing to bop.
        None
    }


    fn draw_cell(is_safety: bool, pawns: &Vec<Pawn>) -> String {
        // TODO: This needs to be tested.
        // Examples:
        //
        // Input:
        // Board::draw_cell(false, vec![Pawn {
        //     color: Color::Green,
        //     id: 3,
        // },
        // Pawn {
        //     color: Color::Green,
        //     id: 1,
        // }];
        // ).as_str();
        //
        // Expected:
        // +--------+
        // | G3 G1  |
        // +--------+
        //
        // Input:
        // Board::draw_cell(true, vec![Pawn {
        //     color: Color::Green,
        //     id: 2,
        // }]);

        // Expected:
        // +--------+
        // |///G2///|
        // +--------+

        let CELL_WIDTH = 8;

        let repeat_x = |c: &str, num_repeats: usize| {
            std::iter::repeat(c)
                .take(num_repeats)
                .collect::<String>()
        };

        let pawn_to_str = |&Pawn { color, id }| -> String {
            let clr = match color {
                Color::Red => "R",
                Color::Blue => "B",
                Color::Green => "G",
                Color::Yellow => "Y",
            };

            clr.to_string() + &id.to_string()
        };

        let pawn_strs = pawns
            .iter()
            .map(pawn_to_str)
            .collect::<Vec<String>>()
            .join(" ");

        let body_str = if is_safety {
            format!("{:/^width$}", pawn_strs, width = CELL_WIDTH)
        } else {
            format!("{:^width$}", pawn_strs, width = CELL_WIDTH)
        };

        format!("{corner}{hsep}{corner}\n\
                 {vsep}{body}{vsep}\n\
                 {corner}{hsep}{corner}",
                corner = "+",
                hsep = repeat_x("-", CELL_WIDTH),
                vsep = "|",
                body = body_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct TestMove {
        color: Color,
        start_index: usize,
        distance: usize,
    }

    impl TestMove {
        fn make_board(&self) -> Board {
            Board::from(map!{
                self.color => [Loc::Spot { index: self.start_index }, Loc::Nest, Loc::Nest, Loc::Nest]
            })
        }

        fn make_dice(&self) -> Dice {
            Dice { rolls: vec![self.distance] }
        }

        fn make_move(&self) -> Move {
            let pawn: Pawn = Pawn {
                id: 0,
                color: self.color,
            };

            let m_type: MoveType = match self.start_index {
                x if x > Board::get_home_row(&self.color) => {
                    MoveType::MoveHome {
                        start: self.start_index,
                        distance: self.distance,
                    }
                }
                _ => {
                    MoveType::MoveMain {
                        start: self.start_index,
                        distance: self.distance,
                    }
                }
            };

            Move {
                pawn: pawn,
                m_type: m_type,
            }
        }

        fn is_valid_with_board_dice(&self, board: Board, dice: Dice) -> bool {
            Board::is_valid_move(&board, &dice, &self.make_move())
        }

        pub fn is_valid_with_board(&self, board: Board) -> bool {
            self.is_valid_with_board_dice(board, self.make_dice())
        }

        pub fn is_valid_with_dice(&self, dice: Dice) -> bool {
            self.is_valid_with_board_dice(self.make_board(), dice)
        }

        pub fn is_valid(&self) -> bool {
            self.is_valid_with_board_dice(self.make_board(), self.make_dice())
        }

        pub fn next(&self) -> (Loc, Option<Bonus>) {
            let board: Board = self.make_board();

            let result: Result<MoveResult, &'static str> =
                board.handle_move(self.make_move());

            if let Ok(MoveResult(next_board, bonus)) = result {
                let next_location: Loc =
                    next_board.get_pawn_loc(&self.color, 0);
                (next_location, bonus)
            } else {
                panic!("Move resulted in an error");
            }
        }
    }

    /// Takes a map of TestMove => Expected pairs, and checks that
    /// everything matches.
    fn test_all_moves(fixtures: BTreeMap<TestMove, (Loc, Option<Bonus>)>) {
        for (tm, &(expected_loc, expected_bonus)) in fixtures.iter() {
            assert!(tm.is_valid());
            let (actual_loc, actual_bonus) = tm.next();
            assert_eq!(expected_loc, actual_loc);
            assert_eq!(expected_bonus, actual_bonus);
        }
    }

    #[test]
    // Pawn color comparison works.
    fn test_pawn_colors() {
        let y1 = Pawn::new(1, Color::Yellow);
        let r1 = Pawn::new(1, Color::Red);
        let r2 = Pawn::new(2, Color::Red);

        assert_ne!(y1.color, r2.color);
        assert_eq!(r1.color, r2.color);
    }

    #[test]
    // Location sorting correctly handles the Home.
    fn sort_player_locs_with_home() {
        let posns = [Loc::Spot { index: 57 },
                     Loc::Home,
                     Loc::Spot { index: 0 },
                     Loc::Spot { index: 402 }];

        let expected: Vec<(usize, Loc)> = vec![(0, Loc::Spot { index: 57 }),
                                               (2, Loc::Spot { index: 0 }),
                                               (3, Loc::Spot { index: 402 }),
                                               (1, Loc::Home)];

        let actual: Vec<(usize, Loc)> = Board::sort_player_locs(&Color::Green,
                                                                posns);

        println!("{:#?}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    // Location sorting correctly handles the Nest.
    fn sort_player_locs_with_nest() {
        let posns = [Loc::Spot { index: 57 },
                     Loc::Nest,
                     Loc::Spot { index: 302 },
                     Loc::Spot { index: 0 }];

        let expected: Vec<(usize, Loc)> = vec![(1, Loc::Nest),
                                               (0, Loc::Spot { index: 57 }),
                                               (3, Loc::Spot { index: 0 }),
                                               (2, Loc::Spot { index: 302 })];

        let actual: Vec<(usize, Loc)> = Board::sort_player_locs(&Color::Yellow,
                                                                posns);

        println!("{:#?}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    // Location sorting respects different players' trajectories.
    fn sort_player_locs_by_color() {
        let posns = [Loc::Spot { index: 11 },
                     Loc::Spot { index: 30 },
                     Loc::Spot { index: 49 },
                     Loc::Spot { index: 66 }];

        assert_eq!(Board::sort_player_locs(&Color::Red, posns),
                   vec![(0, Loc::Spot { index: 11 }),
                        (1, Loc::Spot { index: 30 }),
                        (2, Loc::Spot { index: 49 }),
                        (3, Loc::Spot { index: 66 })]);

        assert_eq!(Board::sort_player_locs(&Color::Blue, posns),
                   vec![(1, Loc::Spot { index: 30 }),
                        (2, Loc::Spot { index: 49 }),
                        (3, Loc::Spot { index: 66 }),
                        (0, Loc::Spot { index: 11 })]);

        assert_eq!(Board::sort_player_locs(&Color::Yellow, posns),
                   vec![(2, Loc::Spot { index: 49 }),
                        (3, Loc::Spot { index: 66 }),
                        (0, Loc::Spot { index: 11 }),
                        (1, Loc::Spot { index: 30 })]);

        assert_eq!(Board::sort_player_locs(&Color::Green, posns),
                   vec![(3, Loc::Spot { index: 66 }),
                        (0, Loc::Spot { index: 11 }),
                        (1, Loc::Spot { index: 30 }),
                        (2, Loc::Spot { index: 49 })]);
    }

    #[test]
    // A pawn of Color A can bop a single pawn of Color B.
    fn can_bop_other_pawn() {
        let board: Board = Board::from(map!{
            Color::Green => [Loc::Spot { index: 14 }, Loc::Nest, Loc::Nest, Loc::Nest],
            Color::Yellow => [Loc::Spot { index: 29 }, Loc::Nest, Loc::Nest, Loc::Nest]
        });

        assert_eq!(board
                       .can_bop(Color::Red, Loc::Spot { index: 14 })
                       .unwrap(),
                   Pawn {
                       color: Color::Green,
                       id: 0,
                   });

        assert_eq!(board
                       .can_bop(Color::Blue, Loc::Spot { index: 29 })
                       .unwrap(),
                   Pawn {
                       color: Color::Yellow,
                       id: 0,
                   });
    }

    #[test]
    // A pawn of Color A cannot bop another pawn of Color A.
    fn cannot_bop_own_pawn() {
        let board: Board = Board::from(map!{
            Color::Red => [Loc::Spot { index: 13 }, Loc::Spot { index: 14 }, Loc::Nest, Loc::Nest]
        });

        assert!(board
                    .can_bop(Color::Red, Loc::Spot { index: 13 })
                    .is_none());
    }

    #[test]
    // A pawn cannot bop anything on an empty spot.
    fn cannot_bop_empty() {
        let board: Board = Board::new();

        assert!(board
                    .can_bop(Color::Green, Loc::Spot { index: 13 })
                    .is_none());
    }

    #[test]
    // A pawn cannot bop on a blockaded spot.
    fn cannot_bop_blockade() {
        let board: Board = Board::from(map!{
            Color::Red => [Loc::Spot { index: 13 }, Loc::Spot { index: 13 }, Loc::Nest, Loc::Nest]
        });

        assert!(board
                    .can_bop(Color::Green, Loc::Spot { index: 13 })
                    .is_none());
    }

    #[test]
    // A pawn can bop another pawn off its entrance.
    fn can_bop_off_entrance() {
        let board: Board = Board::from(map!{
            Color::Green => [Loc::Spot { index: 4 }, Loc::Nest, Loc::Nest, Loc::Nest]
        });

        assert_eq!(board
                       .can_bop(Color::Red, Loc::Spot { index: 4 })
                       .unwrap(),
                   Pawn {
                       color: Color::Green,
                       id: 0,
                   });
    }

    #[test]
    // A pawn cannot be bopped from an entrance spot by another pawn in the main ring.
    // TODO: Our current implementation of can_bop() doesn't take into account the entering
    // move or pawn, so this test won't pass.
    fn cannot_bop_off_entrance_without_entering() {
        let board: Board = Board::from(map!{
            Color::Green => [Loc::Spot { index: 4 }, Loc::Nest, Loc::Nest, Loc::Nest]
        });

        // Yellow can never bop Green off Red's entrance.
        assert!(board
                    .can_bop(Color::Yellow, Loc::Spot { index: 4 })
                    .is_some());
        // .is_none());
    }

    #[test]
    // A pawn cannot bop another player's pawn off a safety spot.
    fn cannot_bop_off_safety() {
        let board: Board = Board::from(map!{
            Color::Green => [Loc::Spot { index: 11 }, Loc::Nest, Loc::Nest, Loc::Nest]
        });

        assert!(board
                    .can_bop(Color::Yellow, Loc::Spot { index: 11 })
                    .is_none());
    }

    #[test]
    // Board handles entering moves.
    fn enter() {
        let board = Board::new();
        let pawn = Pawn {
            color: Color::Green,
            id: 0,
        };
        let mv = Move {
            m_type: MoveType::EnterPiece,
            pawn: pawn,
        };

        let MoveResult(result_board, bonus) = board
            .handle_move(mv)
            .unwrap();

        assert_eq!(result_board, Board::from(map!{
            Color::Green => [Loc::Spot { index: 55 }, Loc::Nest, Loc::Nest, Loc::Nest]
        }));
        assert_eq!(bonus, None);
    }

    #[test]
    // Board handles movements from the entering square.
    fn move_from_entrance() {
        let tests: BTreeMap<TestMove, (Loc, Option<Bonus>)> = map!{
            TestMove { color: Color::Red, start_index: 4, distance: 5 } => (Loc::Spot { index: 9 }, None),
            TestMove { color: Color::Green, start_index: 55, distance: 5 } => (Loc::Spot { index: 60 }, None),
            TestMove { color: Color::Blue, start_index: 38, distance: 5 } => (Loc::Spot { index: 43 }, None),
            TestMove { color: Color::Yellow, start_index: 21, distance: 5 } => (Loc::Spot { index: 26 }, None)
        };

        test_all_moves(tests);
    }

    #[test]
    // Board handles modular wrapping of main ring movements.
    fn main_ring_wrap_index() {
        let tests: BTreeMap<TestMove, (Loc, Option<Bonus>)> = map!{
            TestMove { color: Color::Blue, start_index: 66, distance: 5 } => (Loc::Spot { index: 3 }, None),
            TestMove { color: Color::Green, start_index: 66, distance: 5 } => (Loc::Spot { index: 3 }, None),
            TestMove { color: Color::Yellow, start_index: 64, distance: 5 } => (Loc::Spot { index: 1 }, None)
        };

        test_all_moves(tests);
    }

    #[test]
    // Board handles moving from the main ring to the home row.
    fn main_ring_to_home_row() {
        let tests: BTreeMap<TestMove, (Loc, Option<Bonus>)> = map!{
            TestMove { color: Color::Red, start_index: 66, distance: 5 } => (Loc::Spot { index: 103 }, None),
            TestMove { color: Color::Yellow, start_index: 33, distance: 1 } => (Loc::Spot { index: 300 }, None),
            TestMove { color: Color::Green, start_index: 45, distance: 6 } => (Loc::Spot { index: 400 }, None),
            TestMove { color: Color::Blue, start_index: 13, distance: 6 } => (Loc::Spot { index: 202 }, None)
        };

        test_all_moves(tests);
    }

    #[test]
    // Board handles jumping from the main ring to the home row, using a bonus.
    fn main_ring_bonus() {
        let tm = TestMove {
            color: Color::Yellow,
            start_index: 38,
            distance: 10,
        };

        assert_eq!(tm.next(), (Loc::Spot { index: 48 }, None));
    }

    #[test]
    // Board handles movement within the home row.
    fn move_within_home_row() {
        let tests = map!{
            TestMove { color: Color::Red, start_index: 100, distance: 5 } => (Loc::Spot { index: 105, }, None),
            TestMove { color: Color::Green, start_index: 400, distance: 3 } => (Loc::Spot { index: 403 }, None),
            TestMove { color: Color::Yellow, start_index: 301, distance: 6 } => (Loc::Home, Some(10)),
            TestMove { color: Color::Blue, start_index: 203, distance: 4 } => (Loc::Home, Some(10))
        };

        test_all_moves(tests);
    }

    #[test]
    #[should_panic]
    // Overshooting home from within the home row is invalid,
    // and causes a panic.
    fn cannot_overshoot_home_from_home_row() {
        let tests = vec![TestMove {
                             color: Color::Red,
                             start_index: 100,
                             distance: 5,
                         },
                         TestMove {
                             color: Color::Green,
                             start_index: 404,
                             distance: 3,
                         },
                         TestMove {
                             color: Color::Yellow,
                             start_index: 304,
                             distance: 6,
                         },
                         TestMove {
                             color: Color::Blue,
                             start_index: 203,
                             distance: 4,
                         }];

        assert!(!tests
                     .iter()
                     .any(|tm| tm.is_valid()));

        for tm in tests.iter() {
            tm.next();
        }
    }

    #[test]
    #[should_panic]
    // Overshooting home from the main ring is invalid,
    // and causes a panic.
    fn cannot_overshoot_home_from_main_ring() {
        let tests = vec![TestMove {
                             color: Color::Red,
                             start_index: 64,
                             distance: 20,
                         },
                         TestMove {
                             color: Color::Green,
                             start_index: 49,
                             distance: 10,
                         },
                         TestMove {
                             color: Color::Yellow,
                             start_index: 28,
                             distance: 20,
                         },
                         TestMove {
                             color: Color::Blue,
                             start_index: 16,
                             distance: 10,
                         }];

        assert!(!tests
                     .iter()
                     .any(|tm| tm.is_valid()));

        for tm in tests.iter() {
            tm.next();
        }
    }

    #[test]
    // Board handles jumping from main ring to home row, using bonus.
    fn main_to_home_row_with_bonus() {
        let tests = map!{
            TestMove { color: Color::Red, start_index: 64, distance: 10 } => (Loc::Spot { index: 106, }, None),
            TestMove { color: Color::Green, start_index: 36, distance: 20 } => (Loc::Spot { index: 405 }, None),
            TestMove { color: Color::Yellow, start_index: 30, distance: 10 } => (Loc::Spot { index: 306 }, None),
            TestMove { color: Color::Blue, start_index: 1, distance: 20 } => (Loc::Spot { index: 204 }, None)
        };

        test_all_moves(tests);
    }

    #[test]
    // Board handles jumping from main ring to home, using bonus.
    fn main_to_home_with_bonus() {
        let tm = TestMove {
            color: Color::Blue,
            start_index: 4,
            distance: 20,
        };

        assert_eq!(tm.next(), (Loc::Home, Some(HOME_BONUS)));
    }
}
