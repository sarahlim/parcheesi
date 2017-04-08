mod game;

fn main() {
    println!("Hello, world!");
    let p = game::Pawn::new(1, game::Color::Yellow);
    let m1 = game::Move::EnterPiece { pawn: p };
    // println!("{}!", p.id);
}
