use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub static SCALE: u32 = 3;
pub static GAME_TILE_SIZE: u32 = 16 * SCALE;

pub static SCREEN_WIDTH: u32 = 960;
pub static SCREEN_HEIGHT: u32 = 600;
pub static TOTAL_VIEWPORT_TILES_X: LazyLock<u32> =
    LazyLock::new(|| (SCREEN_WIDTH as f32 / GAME_TILE_SIZE as f32).floor() as u32);

// Camera related
pub static SCROLL_THRESHOLD: u32 = 2;

// DAVE config
// pub static DAVE_WIDTH_BASE: i32 = 20 * SCALE as i32;
// pub static DAVE_HEIGHT_BASE: i32 = 16 * SCALE as i32;
pub static DAVE_CHILL_W: u32 = 9 * SCALE;
pub static DAVE_CHILL_H: u32 = 16 * SCALE;
// pub static DAVE_CHILL_X_START: i32 = 4 * SCALE as i32;
// pub static DAVE_WALKING_W: i32 = 17 * SCALE;
// pub static DAVE_WALKING_H: i32 = 16 * SCALE;
// pub static DAVE_JUMPING_W: i32 = 12 * SCALE;
// pub static DAVE_JUMPING_H: i32 = 15 * SCALE;
// pub static DAVE_JETPACK_W: i32 = 9 * SCALE;
// pub static DAVE_JETPACK_H: i32 = 16 * SCALE;

pub const DAVE_SPEED: u32 = 3; // 4 pixels per frame
pub const DAVE_SPEED_X: u32 = 4;
pub static DAVE_JUMP: u32 = 16 * 2 * SCALE;
pub static DAVE_JUMP_COOLDOWN: u32 = 14; // total
pub static DAVE_JUMP_UP_COOLDOWN: u32 = 6;
pub static DAVE_DEFAULT_TILE: u8 = 54;

pub static DEAD_TIMER: i8 = 30;
pub static DEAD_TILE: u8 = 129;
pub static DOOR_TILE: u8 = 2;

pub static ENEMY_COOLDOWN: u8 = 0;

pub static BULLET_SPEED: u8 = 4;

/// 🚀 Static set of solid tiles (never changes)
pub static SOLID_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        1, 3, 5, // blue wall
        15, 16, // Tunnel
        17, // Walls
        18, // type of wall
        19, // Blue tile
        21, 22, 23, 24, 29, 30, 31, // Pink platform
    ])
});

/// 🚀 Static set of solid tiles (never changes)
pub static SHOOTING_ENEMIES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        89, 90, 91, 91, // brown monsters
    ])
});

/// 🚀 Static collectibles mapping (tile_id -> score)
pub static COLLECTIBLES: LazyLock<HashMap<u8, u32>> = LazyLock::new(|| {
    HashMap::from([
        // Diamond gives 1000 points
        (47, 1000),
        (49, 2000),
        (48, 500),
        (10, 0),
        (11, 0),
        (12, 0),
        (13, 0),
        (20, 0),
        (4, 0),    // Jetpack with fuel
        (51, 200), // ring
    ])
});

pub static DANGER_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        6, 7, 8, 9, // fire
        25, 26, 27, 28, // pink weed
        36, 37, 38, 39, 40, // water
    ])
});
