use std::cmp;

use crate::{
    config::{DAVE_CHILL_W, GAME_TILE_SIZE, SCROLL_THRESHOLD, TOTAL_VIEWPORT_TILES_X},
    game::state::GameState,
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub x: u32,
    left_boundary: u32,
    right_boundary: u32,
}

impl Default for Camera {
    fn default() -> Self {
        let right_boundary = (*TOTAL_VIEWPORT_TILES_X - SCROLL_THRESHOLD) * GAME_TILE_SIZE;
        Self {
            x: 0,
            left_boundary: 0,
            right_boundary,
        }
    }
}

impl Camera {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn update(state: &mut GameState) {
        let is_left_boundary_crossed = state.dave.px <= state.camera.left_boundary;
        let is_right_boundary_crossed = state.dave.px + DAVE_CHILL_W >= state.camera.right_boundary;

        if is_left_boundary_crossed || is_right_boundary_crossed {
            // x_shift
            // set w.r.t tile_size
            // can be -ve ot +ve
            let mut x_shift: i32 = 0;
            if is_left_boundary_crossed {
                x_shift = cmp::max(
                    -(state.camera.x as i32),
                    -((*TOTAL_VIEWPORT_TILES_X - 3 * SCROLL_THRESHOLD) as i32),
                );

                // update camera
                state.camera.move_left(x_shift);
            }
            if is_right_boundary_crossed {
                // x shift
                // set w.r.t tile_size
                // can be -ve ot +ve
                x_shift = cmp::min(
                    (100 - state.camera.x - *TOTAL_VIEWPORT_TILES_X) as i32,
                    (*TOTAL_VIEWPORT_TILES_X - (3 * SCROLL_THRESHOLD + 1)) as i32,
                );

                // update camera
                state.camera.move_right(x_shift);
            }

            // update dave
            state.dave.update_position(x_shift);

            // update batches
            state.level.update_visible_tiles(&state.camera);

            // update bullets of enemies position
            state
                .enemies
                .iter_mut()
                .for_each(|enemy| enemy.bullet.upadate_as_per_cam(x_shift));

            // update dave's bullet
            state.dave.bullet.upadate_as_per_cam(x_shift);
        }
    }

    pub fn move_left(&mut self, x_shift: i32) {
        // Converted directly as x is bounded such that x will never be -ve
        self.x = (self.x as i32 + x_shift) as u32;
        if self.x == 0 {
            self.left_boundary = 0;
        } else {
            self.left_boundary = SCROLL_THRESHOLD * GAME_TILE_SIZE;
        }
        self.right_boundary = (*TOTAL_VIEWPORT_TILES_X - SCROLL_THRESHOLD) * GAME_TILE_SIZE;
    }

    pub fn move_right(&mut self, x_shift: i32) {
        // Converted directly as x is bounded such that x will never be -ve
        self.x = (self.x as i32 + x_shift) as u32;
        if self.x >= 100 - *TOTAL_VIEWPORT_TILES_X {
            self.right_boundary = *TOTAL_VIEWPORT_TILES_X * GAME_TILE_SIZE;
        } else {
            self.right_boundary = (*TOTAL_VIEWPORT_TILES_X - SCROLL_THRESHOLD) * GAME_TILE_SIZE;
        }
        self.left_boundary = SCROLL_THRESHOLD * GAME_TILE_SIZE;
    }
}
