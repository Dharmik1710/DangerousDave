use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub static SCALE: u32 = 3;
pub static GAME_TILE_SIZE: u32 = 16 * SCALE;

pub static SCREEN_WIDTH: u32 = 960;
pub static SCREEN_HEIGHT: u32 = 600;
pub static TOTAL_VIEWPORT_TILES_X: LazyLock<u32> =
    LazyLock::new(|| (SCREEN_WIDTH as f32 / GAME_TILE_SIZE as f32).floor() as u32);

// Camera related
pub static SCROLL_THRESHOLD: u32 = 3;

// DAVE config
// pub static DAVE_WIDTH_BASE: i32 = 20 * SCALE as i32;
// pub static DAVE_HEIGHT_BASE: i32 = 16 * SCALE as i32;
pub static DAVE_CHILL_W: u32 = 9 * SCALE + 2;
pub static DAVE_CHILL_H: u32 = 16 * SCALE;
// pub static DAVE_CHILL_X_START: i32 = 4 * SCALE as i32;
// pub static DAVE_WALKING_W: i32 = 17 * SCALE;
// pub static DAVE_WALKING_H: i32 = 16 * SCALE;
// pub static DAVE_JUMPING_W: i32 = 12 * SCALE;
// pub static DAVE_JUMPING_H: i32 = 15 * SCALE;
// pub static DAVE_JETPACK_W: i32 = 9 * SCALE;
// pub static DAVE_JETPACK_H: i32 = 16 * SCALE;

pub const DAVE_SPEED: u32 = 2 * SCALE; // 4 pixels per frame
pub const DAVE_SPEED_X: u32 = 9;
pub static DAVE_JUMP: u32 = 16 * 2 * SCALE;
pub static DAVE_JUMP_COOLDOWN: u32 = 3; // total
pub static DAVE_JUMP_UP_COOLDOWN: u32 = 0;
pub static DAVE_DEFAULT_TILE: u8 = 54;

pub static DEAD_TIMER: i8 = 60;

pub static DEAD_TILE: u8 = 129;
pub static DOOR_TILE: u8 = 2;
pub static DAVE_BULLET_TILE: u8 = 127;
pub static ENEMY_BULLET_TILE: u8 = 124;

pub static BULLET_COOLDOWN: u8 = 1;
pub static ENEMY_COOLDOWN: u8 = 1;

pub static BULLET_SPEED: i32 = 5 * SCALE as i32;

pub static GUN_TILE: u8 = 20;
pub static JETPACK_TILE: u8 = 4;
pub static JETPACK_FUEL: u32 = 10000;

/// 🚀 Static set of solid tiles (never changes)
pub static SOLID_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        1, // brick wall
        3, // white platform
        5, // blue wall
        15, 16, // Tunnels
        17, // wall
        18, // wall
        19, // Blue tile
        21, 22, 23, 24, // wall/wedge
        29, 30, 31, // Pink platform
    ])
});

/// 🚀 Static set of solid tiles (never changes)
pub static SHOOTING_ENEMIES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        89, 90, 91, 91, // brown monsters
        93, 94, 95, 96, // pink
        105, 106, 107, 108, // grey
        109, 110, 111, 112, // brown
    ])
});

/// 🚀 Static collectibles mapping (tile_id -> score)
pub static COLLECTIBLES: LazyLock<HashMap<u8, u32>> = LazyLock::new(|| {
    HashMap::from([
        (4, 0),     // jetpack
        (10, 1000), // cup
        (20, 0),    // gun
        (47, 100),  // blue diamond
        (48, 50),   // pink ball
        (49, 150),  // red diamond
        (50, 300),  // crown
        (51, 200),  // ring
        (52, 50),   // green wand
    ])
});

pub static DANGER_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        6, 7, 8, 9, // fire
        25, 26, 27, 28, // pink weed
        36, 37, 38, 39, 40, // water
    ])
});

pub static CUP_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| HashSet::from([10, 11, 12, 13]));
