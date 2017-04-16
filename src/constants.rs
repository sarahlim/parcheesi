#![allow(dead_code, unused_variables)]

/// THESE ARE BOARD OFFSETS FOR EACH PLAYER.
use super::board::Color;

pub static RED_ENTRANCE: usize = 0;
pub static BLUE_ENTRANCE: usize = 17;
pub static YELLOW_ENTRANCE: usize = 34;
pub static GREEN_ENTRANCE: usize = 51;

pub static SAFETY_OFFSET: &'static [usize] = &[7, 12];
pub static HOME_ROW_LENGTH: usize = 7;

pub static RED_HOME_ROW: usize = 68;
pub static BLUE_HOME_ROW: usize = 75;
pub static YELLOW_HOME_ROW: usize = 82;
pub static GREEN_HOME_ROW: usize = 89;

pub static EXIT_TO_ENTRANCE: usize = 5;
pub static BOARD_SIZE: usize = 68;

pub static ALL_COLORS: [Color; 4] =
    [Color::Red, Color::Blue, Color::Yellow, Color::Green];


pub static BOP_BONUS: usize = 20;
pub static HOME_BONUS: usize = 10;
