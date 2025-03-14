use sdl2::render::Canvas;

use crate::{render::renderer::{self, Renderer}, resources::app_resources::AppResources};
use super::level::Level;


#[derive(Debug, Clone)]
pub struct Camera {
    pub x_offset: i32,
    left_boundary: i32,
    right_boundary: i32,
    pub scroll_threshold: i8,
    pub tiles_viewport_x: i8,
    pub tile_size: i8,
    pub scale: i8,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x_offset: 0,
            left_boundary: 0,
            right_boundary: 0,
            scroll_threshold: 2,
            tiles_viewport_x: 0,
            tile_size: 16,
            scale: 4,
        }
    }
}

impl Camera {
    pub fn setup(&mut self, renderer: &Renderer) {
        let screen_width = renderer.canvas.window().size().0 as i32;
        let total_tiles_x = (screen_width / (self.tile_size * self.scale) as i32) as i8;

        self.left_boundary = 0;
        self.right_boundary = ((total_tiles_x - self.scroll_threshold) as i32) * self.tile_size as i32 * self.scale as i32;
        self.tiles_viewport_x = total_tiles_x;
    }

    pub fn move_left(&mut self) {
        if self.x_offset > self.left_boundary {
            self.x_offset -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x_offset < self.right_boundary {
            self.x_offset += 1;
        }
    }

    pub fn left_boundary(&self) -> i32 {
        self.left_boundary
    }

    pub fn right_boundary(&self) -> i32 {
        self.right_boundary
    }
}
