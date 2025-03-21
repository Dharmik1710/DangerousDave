use std::cmp;

use crate::{
    config::{self, DAVE_CHILL_W, GAME_TILE_SIZE},
    render::renderer::Renderer,
};

use super::state::GameState;

#[derive(Debug, Clone)]
pub struct Camera {
    pub x: u32,
    left_boundary: u32,
    right_boundary: u32,
    pub scroll_threshold: u32,
    pub tiles_viewport_x: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0,
            left_boundary: 0,
            right_boundary: 0,
            scroll_threshold: 2,
            tiles_viewport_x: 0,
        }
    }
}

impl Camera {
    pub fn setup(&mut self, renderer: &Renderer) {
        let screen_width = renderer.canvas.logical_size().0 as i32;
        let total_tiles_x = (screen_width as f32 / GAME_TILE_SIZE as f32).floor() as u32;

        self.left_boundary = 0;
        self.right_boundary = (total_tiles_x - self.scroll_threshold) * GAME_TILE_SIZE;
        self.tiles_viewport_x = total_tiles_x;
    }

    pub fn update(state: &mut GameState) {
        let is_left_boundary_crossed = state.dave.px <= state.camera.left_boundary;
        let is_right_boundary_crossed = state.dave.px + DAVE_CHILL_W >= state.camera.right_boundary;
        let mut x_shift: i32 = 0;
        if is_left_boundary_crossed {
            // x_shift
            // set w.r.t tile_size
            // can be -ve ot +ve
            x_shift = cmp::max(
                -(state.camera.x as i32),
                -((state.camera.tiles_viewport_x - 6) as i32),
            );

            // // ✅ Round to nearest lower multiple of GAME_TILE_SIZE
            // x_shift -= x_shift % GAME_TILE_SIZE;

            // update camera
            state.camera.move_left(x_shift);

            // update dave
            state.dave.update_position(x_shift);

            // update batches
            state.level.update_visible_tiles(&state.camera);
        }
        if is_right_boundary_crossed {
            // x shift
            // set w.r.t tile_size
            // can be -ve ot +ve
            x_shift = cmp::min(
                (100 - state.camera.x - state.camera.tiles_viewport_x) as i32,
                (state.camera.tiles_viewport_x - 7) as i32,
            );

            // ✅ Round to nearest lower multiple of GAME_TILE_SIZE
            // x_shift -= x_shift % GAME_TILE_SIZE;

            // update camera
            state.camera.move_right(x_shift);

            // update dave
            state.dave.update_position(x_shift);

            // update batches
            state.level.update_visible_tiles(&state.camera);
        }
    }

    pub fn move_left(&mut self, x_shift: i32) {
        // Converted directly as x is bounded such that x will never be -ve
        self.x = (self.x as i32 + x_shift) as u32;
        if self.x == 0 {
            self.left_boundary = 0;
        } else {
            self.left_boundary = 2 * GAME_TILE_SIZE;
        }
    }

    pub fn move_right(&mut self, x_shift: i32) {
        // Converted directly as x is bounded such that x will never be -ve
        self.x += x_shift as u32;
        if self.x >= 100 - self.tiles_viewport_x {
            self.right_boundary = self.tiles_viewport_x * GAME_TILE_SIZE;
        } else {
            self.right_boundary = (self.tiles_viewport_x - 2) * GAME_TILE_SIZE;
        }
    }

    // pub fn left_boundary(&self) -> i32 {
    //     self.left_boundary
    // }

    // pub fn right_boundary(&self) -> i32 {
    //     self.right_boundary
    // }
}
