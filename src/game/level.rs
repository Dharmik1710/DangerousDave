use sdl2::rect::Rect;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

use super::camera::Camera;

#[derive(Debug, Clone)]
pub struct Level {
    pub tiles: Vec<u8>,
    pub batches: HashMap<u8, Vec<Rect>>,
    pub start_position: (u16, u16),
}

impl Default for Level {
    fn default() -> Self {
        Self {
            tiles: Vec::new(),
            batches: HashMap::new(),
            start_position: (0, 0), // Default position if level data is missing
        }
    }
}

impl Level {
    /// Loads level data from file and sets Dave's start position
    pub fn load(&mut self, level_num: u8) -> Result<()> {
        let path = format!("assets/levels/level{}.dat", level_num);
        let mut file = File::open(path)?;
        let mut buffer = vec![0; 1280];
        file.read_exact(&mut buffer)?;

        // Example: Retrieve Dave’s start position from a predefined table or metadata
        let start_position = match level_num {
            1 => (50, 100), // Example coordinates for level 1
            2 => (60, 200), // Example for level 2
            3 => (70, 300),
            _ => (0, 0), // Default fallback
        };

        self.tiles = buffer[256..1256].to_vec();
        self.start_position = start_position;

        Ok(())
    }

    pub fn update_visible_tiles(&mut self, camera: &Camera) {
        let mut visible_tiles: HashMap<u8, Vec<Rect>> = HashMap::new();

        let display_tile_size = (camera.tile_size as f32 * camera.scale).round() as i32;
        let total_columns = 100; // Fixed level width
        let total_rows = 10; // Fixed number of rows

        let start_col = camera.x_offset; // Leftmost visible tile
        let end_col = (camera.x_offset + camera.tiles_viewport_x as i32).min(total_columns);

        for row in 0..total_rows {
            for col in start_col..end_col {
                let tile_index = self.tiles[row as usize * total_columns as usize + col as usize];

                if tile_index == 0 {
                    continue; // Skip empty tiles
                }

                let dest_x = (col - start_col) as i32 * display_tile_size;
                let dest_y = row as i32 * display_tile_size;

                let rect = Rect::new(
                    dest_x,
                    dest_y,
                    display_tile_size as u32,
                    display_tile_size as u32,
                );
                visible_tiles
                    .entry(tile_index)
                    .or_insert_with(Vec::new)
                    .push(rect);
            }
        }
        self.batches = visible_tiles;
    }

    /// Getter for tile data
    pub fn tiles(&self) -> &[u8] {
        &self.tiles
    }
}
