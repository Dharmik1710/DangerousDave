use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub static SCALE: u32 = 3;

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
pub static DAVE_JUMP: u32 = 16 * 2 * SCALE;
pub static DAVE_JUMP_COOLDOWN: u32 = 14; // total
pub static DAVE_JUMP_UP_COOLDOWN: u32 = 8;

/// 🚀 Static set of solid tiles (never changes)
pub static SOLID_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        17, // Walls
        15, 16, // Tunnel
        29, 30, 31, // Pink platform
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
    ])
});

pub static GAME_TILE_SIZE: u32 = 16 * SCALE as u32;
