use std::cmp;
use std::i32::MAX;

use sdl2::rect::Rect;

use crate::config::{self, DAVE_SPEED, GAME_TILE_SIZE};
use crate::game::collectibles::CollectibleManager;
use crate::game::dave::Dave;
use crate::game::state::GameState;
use crate::render::tile_atlas::TileAtlas;
use crate::resources::direction::{self, Direction};

pub struct CollisionDetector;

impl CollisionDetector {
    /// ✅ Checks if any corner of `dave_rect` collides with a solid tile
    pub fn check_collision(state: &mut GameState, direction: Direction) -> i32 {
        let tile_size_f = config::GAME_TILE_SIZE as f32;
        let tile_size = config::GAME_TILE_SIZE as u32;

        let dave_rect = Self::get_apparent_rect(&state.dave, direction);

        // ✅ Extract all four corners
        let corners = [
            dave_rect.top_left(),
            dave_rect.top_right(),
            dave_rect.bottom_left(),
            dave_rect.bottom_right(),
        ];

        let mut displacement = DAVE_SPEED;
        for &corner in &corners {
            // ✅ Convert pixel coordinates to tile index (floor division)
            let tile_x = (corner.x as f32 / tile_size_f).floor() as i32;
            let tile_y = (corner.y as f32 / tile_size_f).floor() as i32;

            // ✅ Retrieve the tile rectangle from TileAtlas
            let tile = state.level.get_tile(state.camera.x, tile_x, tile_y);

            // check for collectibles
            Self::check_collectibles(state, tile_x, tile_y, tile);

            // ✅ Check for intersection with Dave’s rectangle
            if Self::is_solid(tile) {
                let tile_rect = Rect::new(
                    tile_x * tile_size as i32,
                    tile_y * tile_size as i32,
                    tile_size,
                    tile_size,
                );
                if dave_rect.has_intersection(tile_rect) {
                    displacement = 0; // 🚨 Collision detected!
                }
            }
        }

        displacement // ✅ No collision detected
    }

    /// ✅ Returns only relevant hitbox corners based on movement direction
    fn get_apparent_rect(dave: &Dave, direction: Direction) -> Rect {
        let hitbox_w = config::DAVE_CHILL_W;
        let hitbox_h = config::DAVE_CHILL_H;

        match direction {
            Direction::Up => Rect::new(dave.px, dave.py - DAVE_SPEED, hitbox_w, hitbox_h),
            Direction::Down => Rect::new(dave.px, dave.py + DAVE_SPEED, hitbox_w, hitbox_h),
            Direction::Left => Rect::new(dave.px - DAVE_SPEED, dave.py, hitbox_w, hitbox_h),
            Direction::Right => Rect::new(dave.px + DAVE_SPEED, dave.py, hitbox_w, hitbox_h),
            Direction::Chill => todo!(),
        }
    }

    pub fn check_collectibles(state: &mut GameState, tile_x: i32, tile_y: i32, tile: u8) {
        // todo!("Remove this is_collectible check as it is not required, handled by next if check");
        if Self::is_collectible(&tile) {
            // ✅ Then check if we can get collectible points
            if let Some(points) = config::COLLECTIBLES.get(&tile) {
                state.dave.collect(*points);

                // ✅ Remove collectible from level (set tile to 0)
                state
                    .level
                    .update_tile(state.camera.x, tile_x, tile_y, tile);
            }
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

    pub fn is_collectible(tile: &u8) -> bool {
        config::COLLECTIBLES.contains_key(tile)
    }
}
