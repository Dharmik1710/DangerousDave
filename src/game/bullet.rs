use sdl2::rect::Rect;

use crate::config::{GAME_TILE_SIZE, SCALE, SCREEN_WIDTH};
use crate::game::state::GameState;
use crate::render::tile_atlas::TileAtlas;
use crate::resources::direction::{self, Direction};

use super::camera::{self, Camera};

#[derive(Debug, Clone)]
pub struct Bullet {
    pub px: i32,
    pub py: i32,
    pub direction: Direction,
    pub is_active: bool,
    pub tile: u8,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            direction: Direction::default(),
            is_active: false,
            tile: 121,
        }
    }
}

impl Bullet {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

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

    pub fn upadate_as_per_cam(&mut self, x_shift: i32) {
        self.px -= x_shift * GAME_TILE_SIZE as i32;
    }

    // fire bullet
    // (px, py) - position of the enemy/dave in pixel
    pub fn fire(&mut self, px: i32, py: i32, shoot_direction: Direction, tile_num: u8) {
        let (tile_w, _) = TileAtlas::get_dimension(tile_num);
        self.direction = shoot_direction;
        let offset_x = match shoot_direction {
            Direction::Right => tile_w,
            _ => 0,
        };
        self.px = px + offset_x as i32;
        self.py = py;
        self.is_active = true;
    }

    // Update bullet as per camera
    pub fn update_as_per_cam() {}

    pub fn get_rect(&self) -> Rect {
        let (w, h) = TileAtlas::get_dimension(self.tile);
        Rect::new(self.px, self.py, w, h)
    }
}
