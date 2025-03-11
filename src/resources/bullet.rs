use super::direction::Direction;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub direction: Direction, // For example, Left or Right
    pub px: i32,              // Pixel X position
    pub py: i32,              // Pixel Y position
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
            px: 0,
            py: 0,
        }
    }
}
