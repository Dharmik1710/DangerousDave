use crate::config::{GAME_TILE_SIZE, SCREEN_WIDTH};
use crate::game::state::GameState;
use crate::render::tile_atlas::TileAtlas;
use crate::resources::direction::{self, Direction};

use super::camera::{self, Camera};

#[derive(Debug, Clone, Default)]
pub struct Bullet {
    pub px: i32,
    pub py: i32,
    pub direction: Direction,
    pub is_active: bool,
}

impl Bullet {
    pub fn update(&mut self) {
        match self.direction {
            Direction::Left => self.px -= 4,
            Direction::Right => self.px += 4,
            _ => {}
        }

        if self.px < 0 || self.px > SCREEN_WIDTH as i32 {
            self.is_active = false
        }
    }

    // fire bullet
    // (px, py) - position of the enemy/dave in pixel
    pub fn fire(&mut self, px: i32, py: i32, shoot_direction: Direction, tile_num: u8) {
        let tile_dimension = TileAtlas::get_dimension(tile_num);
        self.direction = shoot_direction;
        let offset_x = match shoot_direction {
            Direction::Right => tile_dimension.0,
            _ => 0,
        };
        self.px = px + offset_x as i32;
        self.py = py;
        self.is_active = true;
    }

    // Update bullet as per camera
    pub fn update_as_per_cam() {}
}
