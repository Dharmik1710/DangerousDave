#[derive(Debug, Clone)]
pub struct MonsterState {
    pub px: i32,          // Monster pixel X position
    pub py: i32,          // Monster pixel Y position
    pub grid_x: u32,      // Monster grid X position (world grid)
    pub grid_y: u32,      // Monster grid Y position (world grid)
    pub next_px: i32,     // Next movement waypoint (pixel X)
    pub next_py: i32,     // Next movement waypoint (pixel Y)
    pub dead_timer: u32,  // Timer for monster death animation
    pub path_index: u32,  // Current index on its predefined path
    pub monster_type: u8, // Also used as a tileset index for rendering
}

impl Default for MonsterState {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            grid_x: 0,
            grid_y: 0,
            next_px: 0,
            next_py: 0,
            dead_timer: 0,
            path_index: 0,
            monster_type: 0,
        }
    }
}
