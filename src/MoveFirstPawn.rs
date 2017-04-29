use super::player::{Player};
use super::board::{Board,Pawn,Color};
use super::dice::Dice;

struct MoveFirstPawnPlayer {
    color: Color,
    
}

//MoveFirstPawn tests
mod test {
    use super::*;


    #[test]
    fn move_first_pawn_if_able() {
        //
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
                rolls: vec![3,1],
                used: vec![],
            },
             false)
        };

        let expected_move_1: MoveType = MoveType::MoveMain {
            start: 19,
            distance: 3,
        };

        let expected_pawn:Pawn = Pawn {
            color: Color::Green,
            id: 2,
        };

        let expected_move_2: MoveType = MoveType::MoveMain {
            start: 22,
            distance: 1,
        };

        
    }

    fn
