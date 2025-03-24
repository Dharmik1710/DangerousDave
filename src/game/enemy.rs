use sdl2::rect::Rect;

use crate::{
    config::{ENEMY_COOLDOWN, GAME_TILE_SIZE, SCALE},
    render::tile_atlas::TileAtlas,
};

use super::{camera::Camera, state::GameState};

#[derive(Debug, Clone, Default)]
pub struct Enemy {
    pub x: u32,
    pub y: u32,
    pub path_index: u32,
    pub dead_timer: u32,
    pub enemy_tile: u8,
    pub cooldown: u8,
    pub is_alive: bool,
}

impl Enemy {
    pub fn new(x: u32, y: u32, enemy_tile: u8) -> Self {
        Self {
            x: x * GAME_TILE_SIZE,
            y: y * GAME_TILE_SIZE,
            path_index: 0,
            dead_timer: 0,
            enemy_tile,
            cooldown: ENEMY_COOLDOWN,
            is_alive: true,
        }
    }

    /// ✅ Spawns enemies for the level
    pub fn spawn_enemies(level_number: u8) -> Vec<Enemy> {
        match level_number {
            1 => vec![],
            2 => vec![],
            3 => vec![Self::new(44, 4, 89), Self::new(59, 4, 89)],
            4 => vec![Self::new(32, 2, 93)],
            5 => vec![
                Self::new(15, 3, 97),
                Self::new(33, 3, 97),
                Self::new(49, 3, 97),
            ],
            6 => vec![
                Self::new(10, 8, 101),
                Self::new(28, 8, 101),
                Self::new(45, 2, 101),
                Self::new(40, 8, 101),
            ],
            7 => vec![
                Self::new(5, 2, 105),
                Self::new(16, 1, 105),
                Self::new(46, 2, 105),
                Self::new(56, 3, 105),
            ],
            8 => vec![
                Self::new(53, 5, 109),
                Self::new(72, 2, 109),
                Self::new(84, 1, 109),
            ],
            9 => vec![
                Self::new(35, 8, 113),
                Self::new(41, 8, 113),
                Self::new(49, 8, 113),
                Self::new(65, 8, 113),
            ],
            10 => vec![
                Self::new(45, 8, 117),
                Self::new(51, 2, 117),
                Self::new(65, 3, 117),
                Self::new(82, 5, 117),
            ],
            _ => vec![], // Default: No enemies
        }
    }

    /// ✅ Moves the enemy (Patrolling, AI, etc.)
    pub fn update_enemy(&mut self, path: &[(i8, i8)]) {
        if self.is_alive {
            // check the cooldown, decrement it if not 0
            if self.cooldown > 0 {
                self.cooldown -= 1;
                return;
            }
            // ✅ Extract delta x and y
            let (dx, dy) = path[self.path_index as usize];

            self.x = (self.x as i32 + dx as i32).max(0) as u32;
            self.y = (self.y as i32 + dy as i32).max(0) as u32;

            self.path_index = (self.path_index + 1) % path.len() as u32;

            // set the cooldown
            self.cooldown = ENEMY_COOLDOWN;
        }
    }

    /// Check if enemy is visible on screen
    pub fn is_enemy_on_screen(&self, camera: &Camera) -> bool {
        let diff = self.x as i32 - (camera.x * GAME_TILE_SIZE) as i32;
        let is_visible = diff >= 0 && diff <= (camera.tiles_viewport_x * GAME_TILE_SIZE) as i32;
        self.is_alive && is_visible
    }

    /// ✅ Check collision with Dave
    pub fn check_collision(&self, dave: &Rect, camera: &Camera) -> bool {
        if self.is_enemy_on_screen(camera) && self.is_alive {
            let x = self.x - (camera.x * GAME_TILE_SIZE);
            let y = self.y;
            let (h, w) = TileAtlas::get_dimension(self.enemy_tile);
            let enemy_rect = Rect::new(x as i32, y as i32, h * SCALE, w * SCALE);
            return enemy_rect.has_intersection(*dave);
        }
        false
    }
}
