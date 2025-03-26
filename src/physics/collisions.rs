use std::cmp;
use std::i32::MAX;

use sdl2::rect::{Point, Rect};

use crate::config::{
    self, COLLECTIBLES, DANGER_TILES, DAVE_CHILL_H, DAVE_CHILL_W, DAVE_SPEED, DAVE_SPEED_X,
    DOOR_TILE, GAME_TILE_SIZE, SCALE, SOLID_TILES,
};
use crate::game::camera::Camera;
use crate::game::collectibles::CollectibleManager;
use crate::game::dave::{self, Dave};
use crate::game::enemy::{self, Enemy};
use crate::game::state::GameState;
use crate::render::tile_atlas::TileAtlas;
use crate::resources::direction::{self, Direction};

pub struct CollisionDetector;

impl CollisionDetector {
    /// ✅ Checks if any corner of `dave_rect` collides with a solid tile
    pub fn check_solid_tile_collision(state: &GameState, direction: Direction) -> u32 {
        // Dave rect
        let dave_rect = state.dave.get_rect(direction);

        // ✅ Extract all four corners
        let corners = Self::get_corners(&dave_rect, direction);

        for &corner in &corners {
            // ✅ Convert pixel coordinates to tile index (floor division)
            let tile_x = (corner.x as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile_y = (corner.y as f32 / GAME_TILE_SIZE as f32).floor() as u32;

            // ✅ Retrieve the tile rectangle from TileAtlas
            let tile = state.level.get_tile(state.camera.x, tile_x, tile_y);

            // ✅ Check for intersection with Dave’s rectangle
            if Self::is_solid(tile) {
                let tile_rect = Rect::new(
                    (tile_x * GAME_TILE_SIZE) as i32,
                    (tile_y * GAME_TILE_SIZE) as i32,
                    GAME_TILE_SIZE,
                    GAME_TILE_SIZE,
                );
                if dave_rect.has_intersection(tile_rect) {
                    return 0; // 🚨 Collision detected!
                }
            }
        }
        // ✅ No collision detected
        if direction.is_horizontal() {
            DAVE_SPEED_X
        } else if direction.is_vertical() {
            DAVE_SPEED
        } else {
            0
        }
    }

    pub fn get_corners(rect: &Rect, direction: Direction) -> Vec<(Point)> {
        match direction {
            Direction::Up => vec![rect.top_left(), rect.top_right()],
            Direction::Down => vec![rect.bottom_left(), rect.bottom_right()],
            Direction::Left => vec![rect.top_left(), rect.bottom_left()],
            Direction::Right => vec![rect.top_right(), rect.bottom_right()],
            Direction::Chill => vec![
                rect.top_left(),
                rect.top_right(),
                rect.bottom_left(),
                rect.bottom_right(),
            ], // return all four corners
        }
    }

    /// ✅ Check collision with Dave
    pub fn check_collision_with_enemy(camera: Camera, enemy: &Enemy, dave_rect: Rect) -> bool {
        // check for enemy tile collision
        if enemy.is_enemy_on_screen(camera) && enemy.is_alive {
            let enemy_rect = enemy.get_rect(camera);
            if enemy_rect.has_intersection(dave_rect) {
                return true;
            }
        }
        false
    }

    pub fn check_collision_with_bullet(enemy: &Enemy, rect: Rect) -> bool {
        // check for enemy bullet collision
        if enemy.bullet.is_active {
            let bullet_rect = enemy.bullet.get_rect();
            if bullet_rect.has_intersection(rect) {
                return true;
            }
        }
        false
    }

    // pub fn get_displacement(tile_x: u32, tile_y: u32, dave: &Dave, direction: Direction) -> u32 {
    //     let tile_size = GAME_TILE_SIZE as i32;
    //     let dave_h = DAVE_CHILL_H as i32;
    //     let dave_w = DAVE_CHILL_W as i32;

    //     // be extra careful with the u32 to i32 conversions
    //     match direction {
    //         Direction::Left => dave.px - (tile_x + 1) * tile_size,
    //         Direction::Right => (tile_x * tile_size) - (dave.px + dave_w),
    //         Direction::Up => dave.py - (tile_y + 1) * tile_size,
    //         Direction::Down => tile_y * tile_size - (dave.py + dave_h),
    //         Direction::Chill => 0,
    //     }
    // }

    pub fn is_solid(tile: u8) -> bool {
        SOLID_TILES.contains(&tile)
    }

    pub fn is_collectible(tile: &u8) -> bool {
        COLLECTIBLES.contains_key(tile)
    }
}
