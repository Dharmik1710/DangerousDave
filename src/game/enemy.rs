use sdl2::rect::Rect;

use crate::{
    config::{
        DEAD_TIMER, ENEMY_BULLET_TILE, ENEMY_COOLDOWN, GAME_TILE_SIZE, SCALE, SHOOTING_ENEMIES,
        TOTAL_VIEWPORT_TILES_X,
    },
    physics::collisions::CollisionDetector,
    render::tile_atlas::TileAtlas,
    resources::direction::{self, Direction},
};

use super::{
    bullet::{self, Bullet},
    camera::Camera,
    dave::Dave,
    level::Level,
    state::{self, GameState},
};

#[derive(Debug, Clone, Default)]
pub struct Enemy {
    pub px: u32,
    pub py: u32,
    pub path_index: u32,
    pub dead_timer: i8,
    pub cooldown: u8,
    pub is_alive: bool,
    pub bullet: Bullet,
    pub can_shoot: bool,
    pub tile: u8,
}

impl Enemy {
    pub fn new(x: u32, y: u32, tile: u8) -> Self {
        let can_shoot = SHOOTING_ENEMIES.contains(&tile);
        Self {
            px: x * GAME_TILE_SIZE,
            py: y * GAME_TILE_SIZE,
            path_index: 0,
            dead_timer: DEAD_TIMER,
            cooldown: ENEMY_COOLDOWN,
            is_alive: true,
            bullet: Bullet::new(ENEMY_BULLET_TILE),
            can_shoot,
            tile,
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
    pub fn update_enemy(&mut self, level: &Level, dave: &Dave, camera: Camera) {
        if self.is_alive {
            // check the cooldown, decrement it if not 0
            if self.cooldown > 0 {
                self.cooldown -= 1;
                return;
            }

            // ✅ Extract delta x and y
            let (dx, dy) = level.path[self.path_index as usize];

            self.px = (self.px as i32 + (dx as i32) * (SCALE as i32)).max(0) as u32;
            self.py = (self.py as i32 + (dy as i32) * (SCALE as i32)).max(0) as u32;

            self.path_index = (self.path_index + 1) % level.path.len() as u32;

            // set the cooldown
            self.cooldown = ENEMY_COOLDOWN;
        } else if self.dead_timer > 0 {
            self.dead_timer -= 1;
        }

        // this is to handle edge case in case the enemy dies
        // but its bullet is still on screen
        self.shoot(dave, camera, level);
    }

    /// shoot
    pub fn shoot(&mut self, dave: &Dave, camera: Camera, level: &Level) {
        // shoot only when all of these are true
        //  - bullet is not active
        //  - the enemy is on screen
        //  - enemy can shoot
        if self.bullet.is_active {
            // check collision of bullet with solid tiles before updating
            let bullet_rect = self.bullet.get_rect(self.bullet.direction);
            if !CollisionDetector::check_solid_tile_collision(
                level,
                camera,
                bullet_rect,
                self.bullet.direction,
            ) {
                self.bullet.update();
            } else {
                self.bullet.is_active = false;
            }
        } else if self.is_enemy_on_screen(camera) && self.can_shoot {
            let px = self.px as i32 - (camera.x * GAME_TILE_SIZE) as i32;
            let py = self.py as i32;
            let direction = self.get_shoot_direction(dave, camera);
            self.bullet.fire(px, py, direction, self.tile);
        }
    }

    pub fn get_shoot_direction(&self, dave: &Dave, camera: Camera) -> Direction {
        let dave_pos_abs = camera.x * GAME_TILE_SIZE + dave.px;
        if dave_pos_abs < self.px {
            Direction::Left
        } else {
            Direction::Right
        }
    }

    /// Check if enemy is visible on screen
    pub fn is_enemy_on_screen(&self, camera: Camera) -> bool {
        let diff = self.px as i32 - (camera.x * GAME_TILE_SIZE) as i32;
        let is_visible = diff >= 0 && diff <= (*TOTAL_VIEWPORT_TILES_X * GAME_TILE_SIZE) as i32;
        self.is_alive && is_visible
    }

    pub fn dead(&mut self) {
        self.is_alive = false;
    }

    pub fn get_rect(&self, camera: Camera) -> Rect {
        let px = self.px as i32 - (camera.x * GAME_TILE_SIZE) as i32;
        let py = self.py as i32;
        let (w, h) = TileAtlas::get_dimension(self.tile);
        Rect::new(px, py, w, h)
    }
}
