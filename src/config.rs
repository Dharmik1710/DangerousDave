use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub static SCALE: u32 = 3;

// DAVE config
pub static DAVE_WIDTH_BASE: u16 = 20 * SCALE as u16;
pub static DAVE_HEIGHT_BASE: u16 = 16 * SCALE as u16;
pub static DAVE_CHILL_W: u32 = 9 * SCALE;
pub static DAVE_CHILL_H: u32 = 16 * SCALE;
pub static DAVE_CHILL_X_START: i32 = 4 * SCALE as i32;
pub static DAVE_WALKING_W: u32 = 17 * SCALE;
pub static DAVE_WALKING_H: u32 = 16 * SCALE;
pub static DAVE_JUMPING_W: u32 = 12 * SCALE;
pub static DAVE_JUMPING_H: u32 = 15 * SCALE;
pub static DAVE_JETPACK_W: u32 = 9 * SCALE;
pub static DAVE_JETPACK_H: u32 = 16 * SCALE;

pub const DAVE_SPEED: i32 = 3; // 4 pixels per frame
pub static DAVE_JUMP: i32 = 16 * 3 * SCALE as i32;

/// 🚀 Static set of solid tiles (never changes)
pub static SOLID_TILES: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    HashSet::from([
        17, // Walls
        15, 16, // Tunnel
    ])
});

/// 🚀 Static collectibles mapping (tile_id -> score)
pub static COLLECTIBLES: LazyLock<HashMap<u8, i32>> = LazyLock::new(|| {
    HashMap::from([
        (47, 1000), // Diamond gives 1000 points
                    // TODO
    ])
});

pub static GAME_TILE_SIZE: u16 = 16 * SCALE as u16;
