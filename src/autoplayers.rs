use super::player::Player;
use super::constants::*;
use super::board::{Board, Pawn, Color, Path, Loc, PawnLocs};
use super::game::Move;
use super::dice::Dice;

struct MoveFirstPawnPlayer {
    color: Color,
}

fn sort_locs_by_color(color: &Color, locs: PawnLocs) -> PawnLocs {
    // Try generating the whole path.
    // let path = Path::new(*color).take(BOARD_SIZE);

    // // Get the ordinal index of a pawn location, with respect to the
    // // player's whole path on the board.
    // let pawn_loc_ordinal = |&pawn_loc| -> usize {
    //     let is_pawn_loc = |path_loc| path_loc == pawn_loc;
    //     let position: Option<usize> = path.position(is_pawn_loc);

    //     match position {
    //         Some(index) => index,
    //         None => panic!("Pawn is out of bounds"),
    //     }
    // };

    // // Clone the list of locations so we can sort it.
    // let locs_vector: Vec<Loc> = Vec::with_capacity(locs.len());

    // locs_clone
    //     .iter()
    //     .enumerate()
    //     .collect::<[(usize, &Loc); 4]>()
    //     .sort_by(|&(_, loc)| pawn_loc_ordinal(loc))
    locs
}

impl Player for MoveFirstPawnPlayer {
    /// Always try to move the furthest pawn.
    /// If none of the pawns can be moved with any of the mini-moves,
    /// return an empty vector of moves.
    fn do_move(&self, board: Board, dice: Dice) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // Sort the pawns according to how far they are along the board.
        let pawns: PawnLocs = match board.get_pawns_by_color(&self.color) {
            Ok(pawns) => sort_locs_by_color(&self.color, pawns),
            Err(msg) => panic!(msg),
        };

        moves
    }
}

mod test {
    use super::*;

    #[test]
    fn move_first_pawn_if_able() {
        Board::from(map! { Color::Green => [Loc::Spot { index: 2 },
            Loc::Spot { index: 15 },
            Loc::Spot { index: 19 },
            Loc::Nest ],
            Color::Blue => [Loc::Spot { index: 34 },
            Loc::Spot { index: 38 },
            Loc::Spot { index: 45 },
            Loc::Spot { index 49 }]
        });

        let mut game = Game::new();

        let roll_fn = |_| {
            (Dice {
                 rolls: vec![3, 1],
                 used: vec![],
             },
             false)
        };

        let expected_move_1: MoveType = MoveType::MoveMain {
            start: 19,
            distance: 3,
        };

        let expected_pawn: Pawn = Pawn {
            color: Color::Green,
            id: 2,
        };

        let expected_move_2: MoveType = MoveType::MoveMain {
            start: 22,
            distance: 1,
        };
    }
}
