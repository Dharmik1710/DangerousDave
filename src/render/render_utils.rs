use std::collections::HashMap;

use crate::resources::app_resources::AppResources;
use crate::resources::camera;
use crate::resources::state::GameState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use sdl2::sys::SDL_Rect;
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
    src: SDL_Rect,
    num: u32,
    tile_size_w: u32,
    tile_size_h: u32,
    start_x: i32,
    start_y: i32,
    is_horizontal: bool,
    scale: u32,
) {
    for i in 0..num {
        let (x, y) = if is_horizontal {
            (
                start_x + (i as i32 * tile_size_w as i32) * scale as i32,
                start_y,
            )
        } else {
            (
                start_x,
                start_y + (i as i32 * tile_size_h as i32) * scale as i32,
            )
        };

        let tile_rect = Rect::new(x, y, tile_size_w * scale, tile_size_h * scale);
        if let Err(e) = canvas.copy(texture, Some(src.into()), tile_rect) {
            eprintln!("Failed to draw sprite: {}", e);
        }
    }
}

// // render level
// pub fn render_level(
//     canvas: &mut Canvas<Window>,
//     texture: &Texture,
//     level: &LevelObjects,
// ) {
//     for (_, objects) in level.iter() {
//         for (src, num, start_x, start_y, is_horizontal, scale) in objects {
//             draw_object(
//                 canvas,
//                 texture,
//                 *src.src_rect,
//                 *num,
//                 src.src_rect.width(),
//                 src.src_rect.height(),
//                 *start_x,
//                 *start_y,
//                 *is_horizontal,
//                 *scale,
//             );
//         }
//     }
// }

/// Updates the camera using a sliding window approach.
/// For right movement, x_shift = min((level_x_len - total_vp_tiles) * tile_size, (total_vp_tiles - 7) * tile_size + 1)
/// For left movement, x_shift = max(-camera_offset_x, -(total_vp_tiles-6) * tile_size + 1))
/// new camera position relative to level:= x' = x + x_shift
/// new dave/movable objects position:=  xd' = xd - x_shift
///
/// level_x_len = 100
/// tile_size = 16
pub fn update_camera(app_resources: &AppResources, state: &mut GameState) {

    // let tile_size = 16 * 3; // Assuming 16x16 tiles with scale 3
    // let screen_tiles_x = app_resources.canvas.window().size().0 / tile_size;
    // let threshold = state.camera.scroll_threshold;

    // let left_boundary = state.camera.left_boundary;
    // let right_boundary = state.camera.right_boundary;
    // let dave_x = state.dave.x;

    // if dave_x < left_boundary {
    //     state.camera.x_offset = (dave_x - (threshold * tile_size) as i32).max(0);

    // } else if dave_x > right_boundary {
    //     state.camera.x_offset = (dave_x - ((screen_tiles_x - threshold) * tile_size) as i32).min(
    //         ((state.level_width * tile_size) as i32) - state.camera.viewport_width as i32,
    //     );
    // }
}

pub fn create_visible_tile_batches(state: &mut GameState, app_resources: &AppResources) {
    let mut batches: HashMap<u8, Vec<Rect>> = HashMap::new();
    let camera = &state.camera;

    let scaled_tile_size = state.camera.tile_size * state.camera.scale;
    let tiles = &state.level.raw;

    let total_columns: u8 = 100; // Fixed level width
    let total_rows: u8 = 10; // Fixed number of rows

    let start_col: u8 = camera.x_offset; // Camera's leftmost tile
    let end_col: u8 = (camera.x_offset + camera.tiles_viewport_x); // Rightmost visible tile

    for row in 0..total_rows {
        // Always loop over 10 fixed rows
        for col in start_col..end_col.min(total_columns) {
            let tile_index = tiles[row as usize * total_columns as usize + col as usize];

            if tile_index == 0 {
                continue; // Skip empty tiles
            }

            let dest_x: i32 = ((col - start_col) as i32 * scaled_tile_size as i32);
            let dest_y: i32 = (row as i32 * scaled_tile_size as i32);

            let dest_rect = Rect::new(
                dest_x,
                dest_y,
                scaled_tile_size as u32,
                scaled_tile_size as u32,
            );

            // Insert into the HashMap
            batches
                .entry(tile_index)
                .or_insert_with(Vec::new)
                .push(dest_rect);
        }
    }

    state.level.batches = batches; // Return the HashMap<u8, Vec<Rect>>
}
