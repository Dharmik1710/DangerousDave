#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub grid_x: u32,
    pub grid_y: u32,
    pub path_index: u32,
    pub dead_timer: u32,
    pub enemy_type: u8,
}


impl Enemy {
    pub fn move_enemy(&mut self) {
        self.x += 1; // Dummy movement for now
    }

    pub fn is_dead(&self) -> bool {
        self.dead_timer > 0
    }
}
