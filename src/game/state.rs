use super::camera::{self, Camera};
use super::dave::Dave;
use super::enemy::Enemy;
use super::level::{self, Level};
use std::vec;

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
    pub game_over: bool,
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
            game_over: false,
        }
    }
}

impl GameState {
    pub fn reset(&mut self, is_level_reset: bool) {
        // reset camera
        self.camera.reset();

        // reset dave
        self.dave.reset(is_level_reset);

        // Start dave from original position
        self.dave.init_dave_position(self.current_level);

        // update tiles in hashmap
        self.level.update_visible_tiles(&self.camera);

        // reset enemies bullet
        self.enemies
            .iter_mut()
            .for_each(|enemy| enemy.bullet.deactivate());
    }

    pub fn init_level(&mut self) {
        // load the level
        self.level.load(self.current_level);

        // load enemies
        self.enemies = Enemy::spawn_enemies(self.current_level);

        // reset
        self.reset(true);
    }

    pub fn respawn_dave(&mut self) {
        if self.lives == 0 {
            self.game_over = true;
            return;
        }

        // decrement lives
        self.lives -= 1;

        // reset
        self.reset(false);
    }

    pub fn collect(&mut self, points: u32) {
        self.score += points;
    }
}
