#![allow(dead_code, unused_variables)]

extern crate rand;

use super::game::{Move, MoveType};

use self::rand::Rng;

#[derive(Debug, Clone, PartialEq)]
/// Represents a set of mini-moves for a turn.
///
/// Initializing an instance is equivalent to rolling
/// a pair of six-sided dice.
///
/// After initialization, bonuses can be added, and mini-moves
/// can be used. The instance keeps track of which moves are used.
pub struct Dice {
    pub rolls: Vec<usize>,
}

#[derive(PartialEq, Eq)]
/// Helper enum to facilitate the state changes associated with entering
/// a new pawn.
///
/// Given any set of dice, there are three possibilities:
///
/// 1. Player can enter with a single roll of 5
/// 2. Player can enter with two rolls adding up to 5
/// 3. Player cannot enter based on available rolls
///
/// With cases (1) and (2), we keep track of the values of the
/// relevant rolls, in order to generate the next state.
pub enum EntryMove {
    WithFive, // (1)
    WithSum(usize, usize), // (2)
    NoEntry, // (3)
}


impl Dice {
    /// Initialize a new blank instance.
    pub fn new() -> Dice {
        Dice { rolls: Vec::new() }
    }

    /// Returns xml instance of Dice
    pub fn xmlify(&self) -> String {
        let xml_response: String = "<dice> ".to_string();
        let mut mini_move_string: String = "".to_string();
        for mini_move in &self.rolls {
            mini_move_string = mini_move_string + "<die> " +
                               &mini_move
                                    .clone()
                                    .to_string() +
                               " </die> ";
        }
        xml_response + &mini_move_string + "</dice>"
    }

    /// Initialize a new instance.
    ///
    /// Takes a predicate which denotes whether or not to apply
    /// the doubles bonus, if doubles are rolled.
    pub fn roll(apply_doubles_bonus: bool) -> (Dice, bool) {
        let d1: usize = rand::thread_rng().gen_range(1, 7);
        let d2: usize = rand::thread_rng().gen_range(1, 7);

        let is_doubles: bool = d1 == d2;
        let rolls: Vec<usize> = if is_doubles && apply_doubles_bonus {
            // Award bonus of tops and bottoms of dice.
            vec![d1, d1, 7 - d1, 7 - d1]
        } else {
            vec![d1, d2]
        };

        let dice: Dice = Dice { rolls: rolls };

        (dice, is_doubles)
    }

    /// Check whether the player can enter a new pawn.
    pub fn can_enter(&self) -> EntryMove {
        for (i, &d1) in self.rolls
                .iter()
                .enumerate() {
            // True if any rolls are 5...
            if d1 == 5 {
                return EntryMove::WithFive;
            }

            // ...or any two rolls sum to 5.
            for (j, &d2) in self.rolls
                    .iter()
                    .enumerate() {
                if i != j && d1 + d2 == 5 {
                    return EntryMove::WithSum(d1, d2);
                }
            }
        }

        EntryMove::NoEntry
    }

    /// Consume a game move from the available rolls.
    pub fn consume_move(&self, mv: &Move) -> Dice {
        match mv.m_type {
            MoveType::EnterPiece => self.consume_entry_move(),
            MoveType::MoveHome { distance, .. } |
            MoveType::MoveMain { distance, .. } => {
                self.consume_normal_move(distance)
            }
        }
    }

    /// Consume a mini-move from the list of available rolls, marking it as used.
    /// Returns a new struct with updated lists.
    fn consume_normal_move(&self, distance: usize) -> Dice {
        if let Some(index) = self.rolls
               .iter()
               .position(|&d| d == distance) {
            // Remove the element at the given index.
            let mut next_rolls: Vec<usize> = self.rolls.clone();
            next_rolls.remove(index);

            Dice { rolls: next_rolls }
        } else {
            // Element not found.
            panic!("Distance {} is not a valid move", distance);
        }
    }

    /// Consume mini-move(s) necessary to enter a pawn.
    /// Returns a new struct with updated lists.
    fn consume_entry_move(&self) -> Dice {
        match self.can_enter() {
            EntryMove::WithFive => self.consume_normal_move(5),
            EntryMove::WithSum(x, y) => {
                self.consume_normal_move(x)
                    .consume_normal_move(y)
            }
            EntryMove::NoEntry => panic!("No entry"),
        }
    }

    /// Returns true if there are no available moves
    pub fn all_used(&self) -> bool {
        self.rolls.is_empty()
    }


    /// Checks whether a given mini-move distance is in the
    /// list of valid moves.
    pub fn contains(&self, d: &usize) -> bool {
        self.rolls.contains(d)
    }

    /// Apply a bonus to the list of valid moves.
    pub fn give_bonus(&self, bonus: usize) -> Dice {
        let mut next_rolls: Vec<usize> = self.rolls.clone();
        next_rolls.push(bonus);

        Dice { rolls: next_rolls }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Make sure dice are fair, i.e. the sum of two die rolls
    /// is most frequently 7.
    fn dice_fair() {
        let mut freq = [0; 13]; // 2...12 are the outcomes

        for i in 0..10000 {
            let (dice, _) = Dice::roll(false);
            let sum = dice.rolls[0] + dice.rolls[1];
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
    /// Enter with 1, 4.
    fn enter_1_4() {
        let dice = Dice { rolls: vec![1, 4] };
        assert!(dice.can_enter() != EntryMove::NoEntry);
        assert_eq!(dice.consume_entry_move(), Dice { rolls: Vec::new() });
    }

    #[test]
    /// Enter with 2, 3.
    fn enter_2_3() {
        let dice = Dice { rolls: vec![2, 3] };
        assert!(dice.can_enter() != EntryMove::NoEntry);
        assert_eq!(dice.consume_entry_move(), Dice { rolls: Vec::new() });
    }

    #[test]
    /// Enter two pieces with double 5's.
    fn enter_two_pieces() {
        // Enter two pieces with double 5s
        let mut dice = Dice { rolls: vec![5, 5, 6] };
        assert!(dice.can_enter() != EntryMove::NoEntry);
        dice = dice.consume_entry_move();
        assert_eq!(dice, Dice { rolls: vec![5, 6] });
        assert!(dice.can_enter() != EntryMove::NoEntry);
        dice = dice.consume_entry_move();
        assert_eq!(dice, Dice { rolls: vec![6] });
    }

    #[test]
    /// Cannot enter with non-5 roll.
    fn illegal_enter() {
        let dice = Dice { rolls: vec![3, 3, 6] };
        assert!(dice.can_enter() == EntryMove::NoEntry);
    }
}
