use std::vec;

use crate::render::renderer::Renderer;

use super::bullet::Bullet;
use super::camera::{self, Camera};
use super::dave::Dave;
use super::enemy::Enemy;
use super::level::{self, Level};

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_level: u8,
    pub score: u32,
    pub lives: u32,
    pub camera: Camera,
    pub dave: Dave,
    pub enemies: Vec<Enemy>,
    pub level: Level,
    pub quit: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_level: 3,
            score: 0,
            lives: 3,
            camera: Camera::default(),
            dave: Dave::default(),
            enemies: vec![],
            level: Level::default(),
            quit: false,
        }
    }
}

impl GameState {
    pub fn init_level(&mut self, renderer: &Renderer) {
        // load the level
        self.level.load(self.current_level);

        // update camera
        self.camera.setup(renderer);

        // create batches
        self.level.update_visible_tiles(&self.camera);

        // update player init position
        self.dave.init_dave_position(self.current_level);

        // load enemies
        self.enemies = Enemy::spawn_enemies(self.current_level);
    }
}
