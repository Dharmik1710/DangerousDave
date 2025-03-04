use sdl2::rect::Rect;

/// Represents a sub-rectangle (tile) in a sprite sheet.
/// `src_rect` is the portion of the entire sprite sheet that we want to draw.
pub struct Sprite {
    pub src_rect: Rect,
}

impl Sprite {
    /// Create a sprite from a given pixel offset and tile size.
    /// - `sheet_x`, `sheet_y` = top-left pixel coordinate within the image
    /// - `width`, `height` = size of the sub-rectangle (tile) in pixels
    pub fn new(sheet_x: i32, sheet_y: i32, width: u32, height: u32) -> Self {
        // The Rect::new() takes (x: i32, y: i32, width: u32, height: u32)
        let src_rect = Rect::new(sheet_x, sheet_y, width, height);
        Self { src_rect }
    }
}

/// If your sprite sheet has uniform tiles, you can create a helper function:
pub fn create_sprite_from_tile(
    tile_x: i32, 
    tile_y: i32, 
    tile_width: u32, 
    tile_height: u32
) -> Sprite {
    // Convert tile coordinates (in tile units) to pixel offsets
    let pixel_x = tile_x * tile_width as i32;
    let pixel_y = tile_y * tile_height as i32;
    Sprite::new(pixel_x, pixel_y, tile_width, tile_height)
}

/// If you have a known, specific "wall" tile in your sprite sheet, you can define a
/// specialized constructor or function for it. For example, if the "wall" tile is
/// at tile (3, 2) in a 16x16 tile grid:
pub fn create_wall_sprite() -> Sprite {
    // Example only — actual tile_x/tile_y might differ in your sheet!
    const TILE_X: i32 = 120;
    const TILE_Y: i32 = 105;
    const TILE_SIZE: u32 = 16; // or 32, or whatever your sprite sheet uses
    // create_sprite_from_tile(TILE_X, TILE_Y, TILE_SIZE, TILE_SIZE)
    return Sprite::new(TILE_X, TILE_Y, TILE_SIZE, TILE_SIZE);
}
