use crate::animation::sprite::{create_wall_sprite, Sprite};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use sdl2::video::Window;

/// Draws a row or column of tiles at a given position
///
/// - `canvas`: SDL2 canvas for rendering
/// - `texture`: Tile texture to draw
/// - `num_tiles`: Number of tiles to draw in this row/column
/// - `tile_size`: The size of each tile (width and height)
/// - `start_x`: X position where drawing starts
/// - `start_y`: Y position where drawing starts
/// - `is_horizontal`: `true` for a row, `false` for a column
pub fn draw_object(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    src: Rect,
    num: u32,
    tile_size_w: u32,
    tile_size_h: u32,
    start_x: i32,
    start_y: i32,
    is_horizontal: bool,
    scale: u32
) {
    for i in 0..num {
        let (x, y) = if is_horizontal {
            (start_x + (i as i32 * tile_size_w as i32) * scale as i32, start_y)
        } else {
            (start_x, start_y + (i as i32 * tile_size_h as i32) * scale as i32)
        };

        let tile_rect = Rect::new(x, y, tile_size_w * scale, tile_size_h * scale);
        if let Err(e) = canvas.copy(texture, src, tile_rect) {
            eprintln!("Failed to draw sprite: {}", e);
        }
    }
}

/// Example function: Draw a given Sprite to the canvas at (dest_x, dest_y).
pub fn draw_wall_boundary(canvas: &mut WindowCanvas, texture: &Texture) {
    // get wall sprite
    let wall_sprite = create_wall_sprite();
    let src = wall_sprite.src_rect;

    // let num_tiles_x = 100;
    let scale: u32 = 3;
    let (num_tiles_x, num_tiles_y, x_right, y_bottom) = calculate_coord(canvas, src, scale);

    // Draw Top and Bottom Boundaries
    draw_object(
        canvas,
        texture,
        src,
        num_tiles_x,
        src.width(),
        src.height(),
        0,
        0,
        true,
        scale
    ); // Top row
    draw_object(
        canvas,
        texture,
        src,
        num_tiles_x,
        src.width(),
        src.height(),
        0,
        y_bottom,
        true,
        scale
    ); // Bottom row

    // Draw Left and Right Boundaries
    draw_object(
        canvas,
        texture,
        src,
        num_tiles_y,
        src.width(),
        src.height(),
        0,
        0,
        false,
        scale
    ); // Left column
    draw_object(
        canvas,
        texture,
        src,
        num_tiles_y,
        src.width(),
        src.height(),
        x_right,
        0,
        false,
        scale
    ); // Right column

}

pub fn calculate_coord(
    canvas: &sdl2::render::Canvas<sdl2::video::Window>,
    src: Rect,
    scale: u32
) -> (u32, u32, i32, i32) {
    // Get the window dimensions
    let (win_width, win_height) = canvas.window().size();

    // Calculate the number of whole tiles that fit vertically.
    let num_tiles_x = win_width / (src.width() * scale) + 1;
    let num_tiles_y = win_height / (src.height() * scale) + 1;

    let x_right: i32 = (win_width - (scale * src.width())) as i32;
    let y_bottom: i32 = (win_height - (scale * src.height())) as i32;

    return (num_tiles_x, num_tiles_y, x_right, y_bottom);
}
