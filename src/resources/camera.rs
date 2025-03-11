/// Represents the camera for frustum culling.
#[derive(Debug, Clone)]

/// All positions are in tile size units
pub struct Camera {
    pub x_offset: u8, // X position in tiles
    pub left_boundary: i32,
    pub right_boundary: i32,
    pub scroll_threshold: u8, // Number of tiles before screen moves
    pub tiles_viewport_x: u8,
    pub tile_size: u8,
    pub scale: u8
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x_offset: 0,
            left_boundary: 0, // tile_size * scale * threshold
            right_boundary: 0,
            scroll_threshold: 2,
            tiles_viewport_x: 0,
            tile_size: 16, // scaling 16x16 tile by factor of 3
            scale: 4
        }
    }
}