use crate::game::state::GameState;
use crate::resources::direction::Direction;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: u32,
    pub y: u32,
    pub direction: Direction,
    pub active: bool,
}

impl Bullet {
    pub fn new(x: u32, y: u32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            active: true,
        }
    }

    pub fn update(&mut self) {
        // match self.direction {
        //     Direction::Left => self.x -= self.speed,
        //     Direction::Right => self.x += self.speed,
        //     Direction::Up => todo!(),
        //     Direction::Down => todo!(),
        //     Direction::Chill => todo!(),
        // }
    }

    // pub fn check_collision(&mut self, state: &GameState) {
    //     // If bullet moves out of bounds or hits an enemy, deactivate it
    //     if self.x < 0 || self.x > state.camera.right_boundary() {
    //         self.active = false;
    //     }
    // }
}
