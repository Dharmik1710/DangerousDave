use std::cmp;
use std::i32::MAX;

use sdl2::rect::Rect;

use crate::config::{self, DAVE_SPEED, GAME_TILE_SIZE};
use crate::game::dave::Dave;
use crate::game::state::GameState;
use crate::resources::direction::{self, Direction};

pub struct CollisionDetector;

impl CollisionDetector {
    /// ✅ Checks if Dave will collide in the given direction
    pub fn check_collision(state: &GameState, direction: Direction) -> i32 {
        let tile_size_f = config::GAME_TILE_SIZE as f32;
        let tile_size = GAME_TILE_SIZE as i32;
        let corners = Self::get_relevant_corners(&state.dave, direction);
        let (px1, py1) = corners[0];
        let (px2, py2) = corners[1];

        // Convert to tile coordinates & check collision
        let tile_x1 = (px1 as f32 / tile_size_f).floor() as i32;
        let tile_y1 = (py1 as f32 / tile_size_f).floor() as i32;
        let tile_x2 = (px2 as f32 / tile_size_f).floor() as i32;
        let tile_y2 = (py2 as f32 / tile_size_f).floor() as i32;

        let is_horizontal = matches!(direction, Direction::Left | Direction::Right);
        let is_vertical = matches!(direction, Direction::Up | Direction::Down);

        let tile1 = state.level.get_tile(state.camera.x, tile_x1, tile_y1);
        let tile2 = state.level.get_tile(state.camera.x, tile_x2, tile_y2);

        let is_solid_1 = Self::is_solid(tile1);
        let is_solid_2 = Self::is_solid(tile2);

        if (is_solid_1
            && is_solid_2
            && (is_horizontal && (px1 % tile_size != 0 || px2 % tile_size != 0)
                || (is_vertical && (py1 % tile_size != 0 || py2 % tile_size != 0))))
            || ((is_solid_1 && px1 % tile_size != 0 && py1 % tile_size != 0)
                || (is_solid_2 && px2 % tile_size != 0 && py2 % tile_size != 0))
        {
            return 0;
        }
        config::DAVE_SPEED // ✅ No collision
    }

    /// ✅ Returns only relevant hitbox corners based on movement direction
    fn get_relevant_corners(dave: &Dave, direction: Direction) -> Vec<(i32, i32)> {
        let hitbox_w: i32 = config::DAVE_CHILL_W as i32;
        let hitbox_h: i32 = config::DAVE_CHILL_H as i32;
        let dave_speed = config::DAVE_SPEED;

        match direction {
            Direction::Up => vec![
                (dave.px, dave.py - dave_speed),
                (dave.px + hitbox_w, dave.py - dave_speed),
            ],
            Direction::Down => vec![
                (dave.px, dave.py + hitbox_h + dave_speed),
                (dave.px + hitbox_w, dave.py + hitbox_h + dave_speed),
            ],
            Direction::Left => vec![
                (dave.px - dave_speed, dave.py),
                (dave.px - dave_speed, dave.py + hitbox_h),
            ],
            Direction::Right => vec![
                (dave.px + hitbox_w + dave_speed, dave.py),
                (dave.px + hitbox_w + dave_speed, dave.py + hitbox_h),
            ],
            Direction::Chill => vec![],
        }
    }

    pub fn get_displacement(tile_x: i32, tile_y: i32, dave: &Dave, direction: Direction) -> i32 {
        let tile_size = config::GAME_TILE_SIZE as i32;
        let dave_h = config::DAVE_CHILL_H as i32;
        let dave_w = config::DAVE_CHILL_W as i32;
        match direction {
            Direction::Left => dave.px - (tile_x + 1) * tile_size,
            Direction::Right => (tile_x * tile_size) - (dave.px + dave_w),
            Direction::Up => dave.py - (tile_y + 1) * tile_size,
            Direction::Down => tile_y * tile_size - (dave.py + dave_h),
            Direction::Chill => 0,
        }
    }

    pub fn is_solid(tile: u8) -> bool {
        config::SOLID_TILES.contains(&tile)
    }
}
