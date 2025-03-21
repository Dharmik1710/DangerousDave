use sdl2::rect::Rect;

use crate::config::GAME_TILE_SIZE;

use super::{camera::Camera, state::GameState};

#[derive(Debug, Clone, Default)]
pub struct Enemy {
    pub x: u32,
    pub y: u32,
    pub path_index: u32,
    pub dead_timer: u32,
    pub enemy_tile: u32,
    pub is_alive: bool,
}

impl Enemy {
    pub fn new(x: u32, y: u32, enemy_tile: u32) -> Self {
        Self {
            x,
            y,
            path_index: 0,
            dead_timer: 0,
            enemy_tile,
            is_alive: true,
        }
    }

    /// ✅ Spawns enemies for the level
    pub fn spawn_enemies(level_number: u8) -> Vec<Enemy> {
        match level_number {
            1 => vec![],
            2 => vec![Self::new(44, 4, 89), Self::new(59, 4, 89)],
            3 => vec![Self::new(32, 2, 93)],
            4 => vec![
                Self::new(15, 3, 97),
                Self::new(33, 3, 97),
                Self::new(49, 3, 97),
            ],
            5 => vec![
                Self::new(10, 8, 101),
                Self::new(28, 8, 101),
                Self::new(45, 2, 101),
                Self::new(40, 8, 101),
            ],
            6 => vec![
                Self::new(5, 2, 105),
                Self::new(16, 1, 105),
                Self::new(46, 2, 105),
                Self::new(56, 3, 105),
            ],
            7 => vec![
                Self::new(53, 5, 109),
                Self::new(72, 2, 109),
                Self::new(84, 1, 109),
            ],
            8 => vec![
                Self::new(35, 8, 113),
                Self::new(41, 8, 113),
                Self::new(49, 8, 113),
                Self::new(65, 8, 113),
            ],
            9 => vec![
                Self::new(45, 8, 117),
                Self::new(51, 2, 117),
                Self::new(65, 3, 117),
                Self::new(82, 5, 117),
            ],
            _ => vec![], // Default: No enemies
        }
    }

    /// ✅ Moves the enemy (Patrolling, AI, etc.)
    pub fn update_enemies(state: &mut GameState, camera: Camera, path: Vec<(u32, u32)>) {
        if !state.enemies.is_empty() {
            for enemy in state.enemies.iter_mut() {
                let diff = enemy.x as i32 - camera.x as i32;
                let is_visible = diff >= 0 && diff <= camera.tiles_viewport_x as i32;
                if is_visible && enemy.is_alive {
                    // ✅ Extract new position safely
                    if let Some(&(x, y)) = path.get(enemy.path_index as usize) {
                        enemy.x = x;
                        enemy.y = y;
                    }
                }
            }
        }
    }

    /// ✅ Check collision with Dave
    pub fn check_collision(&self, dave: &Rect) -> bool {
        let x = self.x * GAME_TILE_SIZE;
        let y = self.y * GAME_TILE_SIZE;
        let enemy_rect = Rect::new(x as i32, y as i32, GAME_TILE_SIZE, GAME_TILE_SIZE);
        enemy_rect.has_intersection(*dave)
    }
}
