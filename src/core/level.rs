use std::collections::HashMap;

use sdl2::rect::Rect;

#[derive(Debug, Clone)]
pub struct Level {
    pub raw: Vec<u8>,
    pub batches: HashMap<u8, Vec<Rect>>
}

impl Default for Level {
    fn default() -> Self {
        Self {
            raw: Vec::new(),
            batches: HashMap::new()
        }
    }
}
