use super::player::Player;

pub trait NetworkPlayer: Player {
    fn connect(&mut self) -> ();
    fn send(&mut self, msg: String) -> ();
    fn receive(&mut self) -> ();
}
