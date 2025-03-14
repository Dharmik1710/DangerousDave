use crate::render::renderer::{self, Renderer};

use super::level::Level;
use super::player::Player;
use super::enemy::Enemy;
use super::bullet::Bullet;
use super::camera::Camera;

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_level: u8,
    pub score: u32,
    pub lives: u8,
    pub camera: Camera,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub level: Level,
    pub quit: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_level: 1,
            score: 0,
            lives: 3,
            camera: Camera::default(),
            player: Player::default(),
            enemies: vec![],
            bullets: vec![],
            level: Level::default(),
            quit: false,
        }
    }
}

impl GameState {
    pub fn init_level(&mut self, renderer: &Renderer){
        
        // load the level
        self.level.load(self.current_level);

        // update camera
        self.camera.setup(renderer);

        // create batches
        self.level.update_visible_tiles(&self.camera);

        // update player init position
        self.player.update_position(self.level.start_position);      

    }
}