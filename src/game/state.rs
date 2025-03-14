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
    pub lives: u8,
    pub camera: Camera,
    pub dave: Dave,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub level: Level,
    pub quit: bool,
}

impl Default for GameState {
    fn default() -> Self {
        let scale: f32 = 4.5;
        Self {
            current_level: 1,
            score: 0,
            lives: 3,
            camera: Camera::new(scale),
            dave: Dave::new(scale),
            enemies: vec![],
            bullets: vec![],
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
        self.dave
            .update_position(self.level.dave_start_pos, self.camera.tile_size);
    }
}
