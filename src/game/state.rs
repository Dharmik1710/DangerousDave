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
    pub jetpack: u32,
    pub has_cup: bool,
    pub has_gun: bool,
    pub on_door: bool,
    pub game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_level: 2,
            score: 0,
            lives: 5,
            camera: Camera::default(),
            dave: Dave::default(),
            enemies: vec![],
            level: Level::default(),
            quit: false,
            jetpack: 0,
            has_cup: false,
            has_gun: false,
            on_door: false,
            game_over: false,
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        // reset camera
        self.camera.reset();

        // reset dave
        self.dave.reset();

        // reset enemies bullet
        self.enemies
            .iter_mut()
            .for_each(|enemy| enemy.bullet.reset());

        // Start dave from original position
        self.dave.init_dave_position(self.current_level);

        // update tiles in hashmap
        self.level.update_visible_tiles(&self.camera);
    }

    pub fn init_level(&mut self) {
        // load the level
        self.level.load(self.current_level);

        // reset
        self.reset();

        // load enemies
        self.enemies = Enemy::spawn_enemies(self.current_level);
    }

    pub fn respawn_dave(&mut self) {
        if self.lives == 0 {
            self.game_over = true;
            return;
        }

        // decrement lives
        self.lives -= 1;

        // reset
        self.reset();
    }

    pub fn is_level_complete(&self) -> bool {
        self.dave.is_alive && self.has_cup && self.on_door
    }

    pub fn collect(&mut self, points: u32) {
        self.score += points;
    }
}
