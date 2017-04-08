#[derive(Debug, Clone, PartialEq)]
/// Represents a color of a piece or player.
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, Clone)]
/// Represents a pawn on the board.
pub struct Pawn {
    id: i32, // 0..3
    color: Color,
}

impl Pawn {
    pub fn new(id: i32, color: Color) -> Pawn {
        Pawn {
            id: id,
            color: color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pawn_colors() {
        let y1 = Pawn::new(1, Color::Yellow);
        let r1 = Pawn::new(1, Color::Red);
        let r2 = Pawn::new(2, Color::Red);
        assert_ne!(y1.color, r2.color);
        assert_eq!(r1.color, r2.color);
    }
}
