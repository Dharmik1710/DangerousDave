use std::cmp;
use std::i32::MAX;

use sdl2::rect::Rect;

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
    pub fn check_collision(state: &mut GameState, direction: Direction) -> u32 {
        let dave_rect = Self::get_apparent_rect(&state.dave, direction);

        // ✅ Extract all four corners
        let corners = [
            dave_rect.top_left(),
            dave_rect.top_right(),
            dave_rect.bottom_left(),
            dave_rect.bottom_right(),
        ];

        let mut displacement = DAVE_SPEED_X;
        for &corner in &corners {
            // ✅ Convert pixel coordinates to tile index (floor division)
            let tile_x = (corner.x as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile_y = (corner.y as f32 / GAME_TILE_SIZE as f32).floor() as u32;

            // ✅ Retrieve the tile rectangle from TileAtlas
            let tile = state.level.get_tile(state.camera.x, tile_x, tile_y);

            // check for collectibles
            Self::check_collectibles(state, tile_x, tile_y, tile);

            // Check if Dave steps on a deadly tile
            Self::check_danger_tile(state, tile);

            // Check dave's collision with enemy and its bullet
            Self::check_collision_with_enemy(
                &mut state.dave,
                state.camera,
                &mut state.enemies,
                dave_rect,
            );

            // Check if Dave has completed the level
            Self::check_door_collision(state, tile);

            // ✅ Check for intersection with Dave’s rectangle
            if Self::is_solid(tile) {
                let tile_rect = Rect::new(
                    (tile_x * GAME_TILE_SIZE) as i32,
                    (tile_y * GAME_TILE_SIZE) as i32,
                    GAME_TILE_SIZE,
                    GAME_TILE_SIZE,
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
        let hitbox_w = DAVE_CHILL_W;
        let hitbox_h = DAVE_CHILL_H;

        // be extra careful with the u32 to i32 conversions
        match direction {
            Direction::Up => Rect::new(
                dave.px as i32,
                (dave.py - DAVE_SPEED) as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Down => Rect::new(
                dave.px as i32,
                (dave.py + DAVE_SPEED) as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Left => Rect::new(
                (dave.px - DAVE_SPEED) as i32,
                dave.py as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Right => Rect::new(
                (dave.px + DAVE_SPEED) as i32,
                dave.py as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Chill => todo!(),
        }
    }

    /// ✅ Check collision with Dave
    pub fn check_collision_with_enemy(
        dave: &mut Dave,
        camera: Camera,
        enemies: &mut [Enemy],
        dave_rect: Rect,
    ) -> bool {
        for enemy in enemies.iter_mut() {
            // check for enemy tile collision
            if enemy.is_enemy_on_screen(camera) && enemy.is_alive {
                let enemy_rect = enemy.get_rect(camera);
                if enemy_rect.has_intersection(dave_rect) {
                    enemy.dead();
                    dave.dead();
                }
            }
            // check for enemy bullet collision
            if enemy.bullet.is_active {
                let bullet_rect = enemy.bullet.get_rect();
                if bullet_rect.has_intersection(dave_rect) {
                    dave.dead();
                    enemy.bullet.is_active = false;
                }
            }
        }
        false
    }

    pub fn check_collectibles(state: &mut GameState, tile_x: u32, tile_y: u32, tile: u8) {
        // todo!("Remove this is_collectible check as it is not required, handled by next if check");
        if Self::is_collectible(&tile) {
            // ✅ Then check if we can get collectible points
            if let Some(points) = COLLECTIBLES.get(&tile) {
                state.dave.collect(*points);

                // ✅ Remove collectible from level (set tile to 0)
                state
                    .level
                    .update_tile(state.camera.x, tile_x, tile_y, tile);
            }
        }
    }

    pub fn check_danger_tile(state: &mut GameState, tile: u8) {
        if DANGER_TILES.contains(&tile) {
            state.dave.dead();
        }
    }

    pub fn check_door_collision(state: &GameState, tile: u8) {
        if tile == DOOR_TILE {}
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
