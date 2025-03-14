use sdl2::render::Texture;

use crate::render::render_utils::create_visible_tile_batches;
use crate::resources::app_resources::{self, AppResources};
use crate::resources::state::GameState;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Result};

// pub fn init_level(state: &mut GameState, app_resources: &AppResources) -> Result<()> {
//     let level_num = state.current_level;
//     let path = format!("assets/level/level{}.dat", level_num);

//     let mut file = File::open(&path)?;
//     let mut buffer = vec![0; 1280];
//     file.read_exact(&mut buffer)?;

//     // Validate file size
//     if buffer.len() < 1280 {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "Level file is too short",
//         ));
//     }

//     // --- Parse tile data (1000 bytes starting at byte 256) ---
//     let tile_data = &buffer[256..1256];
//     let mut tiles = Vec::with_capacity(1000);
//     tiles.extend_from_slice(tile_data);

//     // updating the data in gamestate obj
//     state.level.raw = tiles;

//     // set the camera
//     let tiles_viewport_x: u8 =
//         (app_resources.canvas.window().size().0 / state.camera.tile_size as u32) as u8;
//     state.camera.left_boundary = 0;
//     state.camera.right_boundary =
//         (tiles_viewport_x - state.camera.scroll_threshold) * tiles_viewport_x;
//     state.camera.tiles_viewport_x = tiles_viewport_x;

//     Ok(())
// }

pub fn init_level(state: &mut GameState, app_resources: &AppResources) -> Result<()> {
    let level_num = state.current_level;
    let path = format!("assets/levels/level{}.dat", level_num);

    // Parse the level (No need for &mut GameState)
    let tiles = parse_level_data(&path)?;

    // Update GameState (Now we need &mut GameState)
    update_game_state(state, app_resources, tiles);

    // Create hashmap for rendering
    create_visible_tile_batches(state, app_resources);

    Ok(())
}

/// Parses the level file and extracts tile data (No `&mut GameState` required)
fn parse_level_data(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0; 1280];
    file.read_exact(&mut buffer)?;

    // Validate file size
    if buffer.len() < 1280 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Level file is too short",
        ));
    }

    // Extract tile data (1000 bytes starting at byte 256)
    let tile_data = &buffer[256..1256];
    let mut tiles = Vec::with_capacity(1000);
    tiles.extend_from_slice(tile_data);

    Ok(tiles) // Return parsed tile data
}

/// Updates `GameState` using parsed tile data (Requires `&mut GameState`)
fn update_game_state(state: &mut GameState, app_resources: &AppResources, tiles: Vec<u8>) {
    // Update game state with parsed tile data
    state.level.raw = tiles;

    // Set camera properties
    let tiles_viewport_x: u8 = (app_resources.canvas.window().size().0
        / (state.camera.tile_size * state.camera.scale) as u32)
        as u8;
    state.camera.left_boundary = 0;
    state.camera.right_boundary =
        (tiles_viewport_x - state.camera.scroll_threshold) as i32 * tiles_viewport_x as i32;
    state.camera.tiles_viewport_x = tiles_viewport_x;
}
