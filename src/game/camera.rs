use crate::{config, render::renderer::Renderer};

#[derive(Debug, Clone)]
pub struct Camera {
    pub x: u32,
    pub y: u32,
    left_boundary: u32,
    right_boundary: u32,
    pub scroll_threshold: u8,
    pub tiles_viewport_x: u8,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            left_boundary: 0,
            right_boundary: 0,
            scroll_threshold: 2,
            tiles_viewport_x: 0,
        }
    }
}

impl Camera {
    pub fn setup(&mut self, renderer: &Renderer) {
        let screen_width = renderer.canvas.window().size().0 as i32;
        let total_tiles_x = (screen_width / config::GAME_TILE_SIZE as i32) as u8;

        self.left_boundary = 0;
        self.right_boundary =
            ((total_tiles_x - self.scroll_threshold) as u32) * config::GAME_TILE_SIZE as u32;
        self.tiles_viewport_x = total_tiles_x;
    }

    // pub fn move_left(&mut self) {
    //     if self.x_offset > self.left_boundary {
    //         self.x_offset -= 1;
    //     }
    // }

    // pub fn move_right(&mut self) {
    //     if self.x_offset < self.right_boundary {
    //         self.x_offset += 1;
    //     }
    // }

    // pub fn left_boundary(&self) -> i32 {
    //     self.left_boundary
    // }

    // pub fn right_boundary(&self) -> i32 {
    //     self.right_boundary
    // }
}
