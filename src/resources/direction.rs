#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Chill,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Chill
    }
}

impl Direction {
    /// Returns the opposite direction
    pub fn opposite(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Chill => Direction::Chill,
        }
    }

    /// Checks if the direction is horizontal (Left/Right)
    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    /// Checks if the direction is vertical (Up/Down)
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn is_left(self) -> bool {
        matches!(self, Direction::Left)
    }

    pub fn is_right(self) -> bool {
        matches!(self, Direction::Right)
    }
}
