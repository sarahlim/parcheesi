#![allow(dead_code)]

use super::board::Color;

pub static COLORS: [Color; 4] =
    [Color::Red, Color::Blue, Color::Yellow, Color::Green];

/// THESE ARE BOARD OFFSETS FOR EACH PLAYER.
pub static RED_ENTRANCE: usize = 4;
pub static BLUE_ENTRANCE: usize = 21;
pub static YELLOW_ENTRANCE: usize = 38;
pub static GREEN_ENTRANCE: usize = 55;

pub static RED_HOME_ROW: usize = 100;
pub static BLUE_HOME_ROW: usize = 200;
pub static YELLOW_HOME_ROW: usize = 300;
pub static GREEN_HOME_ROW: usize = 400;

pub static RED_EXIT: usize = 67;
pub static BLUE_EXIT: usize = 16;
pub static YELLOW_EXIT: usize = 33;
pub static GREEN_EXIT: usize = 50;

pub static HOME_ROW_LENGTH: usize = 7;
pub static BOARD_SIZE: usize = 68;

pub static SAFETY_SPOTS: &'static [usize] = &[4, 11, 16, 21, 28, 33, 38, 45,
                                              50, 55, 62, 67];

pub static BOP_BONUS: usize = 20;
pub static HOME_BONUS: usize = 10;
