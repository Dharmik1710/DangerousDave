use std::sync::LazyLock;

pub const SCALE: f32 = 4.5;

// DAVE config
pub static DAVE_WIDTH: LazyLock<u16> = LazyLock::new(|| (20.0 * SCALE).round() as u16);
pub static DAVE_HEIGHT: LazyLock<u16> = LazyLock::new(|| (16.0 * SCALE).round() as u16);
pub const DAVE_SPEED: i32 = 4;

pub static GAME_TILE_SIZE: LazyLock<u16> = LazyLock::new(|| (16.0 * SCALE).round() as u16);
