mod events;
pub mod game;
mod level;
mod renderer;
mod snake;

// Point represents a position in the gameplan
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_compatible_with(&self, other: Direction) -> bool {
        !matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }
}
