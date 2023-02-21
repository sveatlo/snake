use crate::Direction;

#[derive(Debug)]
pub enum Event {
    None,
    Direction(Direction),
    Quit,
}
