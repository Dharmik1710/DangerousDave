use super::camera::Camera;
use crate::config;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug, Clone, Default)]
pub struct Level {
    pub tiles: Vec<u8>,
    pub batches: HashMap<u8, Vec<Rect>>,
    pub dave_init_pos: (u16, u16),
}

impl Level {
    /// Loads level data from file and sets Dave's start position
    pub fn load(&mut self, level_num: u8) -> Result<()> {
        let path = format!("assets/levels/level{}.dat", level_num);
        let mut file = File::open(path)?;
        let mut buffer = vec![0; 1280];
        file.read_exact(&mut buffer)?;

        // Example: Retrieve Dave’s start position from a predefined table or metadata
        let dave_init_pos = match level_num {
            1 => (2, 8),
            2 => (1, 8),
            3 => (2, 5),
            4 => (1, 5),
            5 => (2, 8),
            6 => (2, 8),
            7 => (1, 2),
            8 => (2, 8),
            9 => (6, 1),
            10 => (2, 8),
            _ => (0, 0), // Default fallback
        };

        self.tiles = buffer[256..1256].to_vec();
        self.dave_init_pos = dave_init_pos;

        Ok(())
    }

    pub fn get_tile(&self, cam_x: u32, tile_x: i32, tile_y: i32) -> u8 {
        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed level height

        let col_index = tile_x + cam_x as i32;
        let row_index = tile_y;

        if col_index < 0
            || col_index >= total_columns as i32
            || row_index < 0
            || row_index >= total_rows as i32
        {
            return 0; // Out of bounds, return empty tile
        }

        let index = (row_index as usize) * total_columns + (col_index as usize);
        self.tiles.get(index).copied().unwrap_or(0)
    }

    /// ✅ Updates the tile at (tile_x, tile_y) in the level
    pub fn update_tile(&mut self, cam_x: u32, tile_x: i32, tile_y: i32, new_tile: u8) {
        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed level height

        let col_index = tile_x + cam_x as i32;
        let row_index = tile_y;

        // ✅ Bounds check to prevent out-of-bounds access
        if col_index < 0
            || col_index >= total_columns as i32
            || row_index < 0
            || row_index >= total_rows as i32
        {
            return; // Out of bounds, do nothing
        }

        // ✅ Compute the 1D index from (x, y)
        let index = (row_index as usize) * total_columns + (col_index as usize);

        if let Some(tile) = self.tiles.get_mut(index) {
            *tile = new_tile; // ✅ Update the tile safely
        }
    }

    pub fn update_visible_tiles(&mut self, camera: &Camera) {
        let mut visible_tiles: HashMap<u8, Vec<Rect>> = HashMap::new();

        let display_tile_size = config::GAME_TILE_SIZE as u32;
        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed number of rows

        let start_col = camera.x; // Leftmost visible tile
        let end_col = (camera.x + camera.tiles_viewport_x as u32).min(total_columns);

        for row in 0..total_rows {
            for col in start_col..end_col {
                let tile_index = self.tiles[row as usize * total_columns as usize + col as usize];

                if tile_index == 0 {
                    continue; // Skip empty tiles
                }

                let dest_x = ((col - start_col) * display_tile_size) as i32;
                let dest_y = (row * display_tile_size) as i32;

                let rect = Rect::new(
                    dest_x,
                    dest_y,
                    display_tile_size as u32,
                    display_tile_size as u32,
                );
                visible_tiles.entry(tile_index).or_default().push(rect);
            }
        }
        self.batches = visible_tiles;
    }

    /// Getter for tile data
    pub fn tiles(&self) -> &[u8] {
        &self.tiles
    }
}
