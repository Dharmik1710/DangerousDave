use super::camera::Camera;
use crate::config::GAME_TILE_SIZE;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug, Clone, Default)]
pub struct Level {
    pub path: Vec<(i8, i8)>,
    pub tiles: Vec<u8>,
    pub batches: HashMap<u8, Vec<Rect>>,
}

impl Level {
    /// Loads level data from file and sets Dave's start position
    pub fn load(&mut self, level_num: u8) -> Result<()> {
        let path = format!("assets/levels/level{}.dat", level_num);
        let mut file = File::open(path)?;
        let mut buffer = vec![0; 1280];
        file.read_exact(&mut buffer)?;

        // update tiles
        self.tiles = buffer[256..1256].to_vec();

        // update path
        self.parse_enemy_path(buffer);

        Ok(())
    }

    pub fn get_tile(&self, cam_x: u32, tile_x: u32, tile_y: u32) -> u8 {
        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed level height

        let col_index = tile_x + cam_x;
        let row_index = tile_y;

        if col_index >= total_columns || row_index >= total_rows {
            return 0; // Out of bounds, return empty tile
        }

        let index = (row_index * total_columns + col_index) as usize;
        self.tiles.get(index).copied().unwrap_or(0)
    }

    /// ✅ Updates the tile at (tile_x, tile_y) in the level
    pub fn update_tile(&mut self, cam_x: u32, tile_x: u32, tile_y: u32, old_tile: u8) {
        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed level height

        let col_index = tile_x + cam_x;
        let row_index = tile_y;

        // ✅ Bounds check to prevent out-of-bounds access
        if col_index >= total_columns || row_index >= total_rows {
            return; // Out of bounds, do nothing
        }

        // ✅ Compute the 1D index from (x, y)
        let index = (row_index * total_columns + col_index) as usize;

        if let Some(tile) = self.tiles.get_mut(index) {
            *tile = 0; // ✅ Update the tile safely
        }

        // update the batches hashmap too
        self.update_batches_tile(tile_x, tile_y, old_tile);
    }

    pub fn update_batches_tile(&mut self, tile_x: u32, tile_y: u32, old_tile: u8) {
        let tile_px = (tile_x * GAME_TILE_SIZE) as i32;
        let tile_py = (tile_y * GAME_TILE_SIZE) as i32;

        // ✅ Get mutable reference to the Vec<Rect>
        if let Some(tile_rects) = self.batches.get_mut(&old_tile) {
            // ✅ Remove the Rect that matches the given tile position
            tile_rects.retain(|rect| !(rect.x == tile_px && rect.y == tile_py));
        }
    }

    pub fn update_visible_tiles(&mut self, camera: &Camera) {
        let mut visible_tiles: HashMap<u8, Vec<Rect>> = HashMap::new();

        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed number of rows

        let start_col = camera.x; // Leftmost visible tile
        let end_col = (camera.x + camera.tiles_viewport_x).min(total_columns);

        for row in 0..total_rows {
            for col in start_col..end_col {
                let tile_index = self.tiles[(row * total_columns + col) as usize];

                if tile_index == 0 {
                    continue; // Skip empty tiles
                }

                let dest_x = ((col - start_col) * GAME_TILE_SIZE) as i32;
                let dest_y = (row * GAME_TILE_SIZE) as i32;

                let rect = Rect::new(dest_x, dest_y, GAME_TILE_SIZE, GAME_TILE_SIZE);
                visible_tiles.entry(tile_index).or_default().push(rect);
            }
        }
        self.batches = visible_tiles;
    }

    /// Getter for tile data
    pub fn tiles(&self) -> &[u8] {
        &self.tiles
    }

    /// ✅ Parses the movement path for enemies from the buffer
    pub fn parse_enemy_path(&mut self, buffer: Vec<u8>) {
        let mut movement_path: Vec<(i8, i8)> = Vec::new();
        let path = buffer[0..256].to_vec();
        let mut iter = path.chunks_exact(2); // Iterate in steps of 2

        while let Some(&[dx, dy]) = iter.next() {
            if dx == 0xEA && dy == 0xEA {
                break; // Stop parsing at loop point
            }

            movement_path.push((dx as i8, dy as i8)); // Convert to signed integers
        }

        self.path = movement_path;
    }
}
