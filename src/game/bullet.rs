use crate::game::state::GameState;
use crate::resources::direction::Direction;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub direction: Direction,
    pub active: bool,
}

impl Bullet {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            x,
            y,
            speed: 5,
            direction,
            active: true,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Up => todo!(),
            Direction::Down => todo!(),
            Direction::Chill => todo!(),
        }
    }

    // pub fn check_collision(&mut self, state: &GameState) {
    //     // If bullet moves out of bounds or hits an enemy, deactivate it
    //     if self.x < 0 || self.x > state.camera.right_boundary() {
    //         self.active = false;
    //     }
    // }
}
