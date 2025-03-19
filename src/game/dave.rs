use super::state::GameState;
use crate::{
    config::{self, DAVE_JUMP, DAVE_JUMP_COOLDOWN, GAME_TILE_SIZE},
    physics::collisions::CollisionDetector,
    resources::direction::{self, Direction},
};
use sdl2::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub enum DaveState {
    Chilling,
    Running,
    Jumping,
    Jetpack,
}

#[derive(Debug, Clone)]
pub struct Dave {
    pub px: i32,
    pub py: i32,
    pub jump: i32,
    pub jump_cooldown: u8,
    pub direction: Direction,
    pub on_ground: bool,
    pub jetpack: bool,
    pub dave_state: DaveState,
    pub score: i32,
}

impl Default for Dave {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            jump: 0,
            jump_cooldown: 0,
            direction: Direction::Chill,
            jetpack: false,
            on_ground: true,
            dave_state: DaveState::Chilling,
            score: 0,
        }
    }
}

impl Dave {
    pub fn move_left(&mut self, displacement: i32) {
        self.px -= displacement;
    }

    pub fn move_right(&mut self, displacement: i32) {
        self.px += displacement;
    }

    pub fn jump(&mut self) {
        if self.on_ground && self.jump == 0 {
            if self.jump_cooldown != 0 {
                self.decr_cooldown();
                return;
            }
            self.jump = DAVE_JUMP; // Set jump height
            self.on_ground = false; // Dave is no longer on the ground
            self.jump_cooldown = DAVE_JUMP_COOLDOWN;
        }
    }

    pub fn collect(&mut self, points: i32) {
        self.score += points;
    }

    pub fn move_up(&mut self, displacement: i32) {
        self.py -= displacement;
        self.jump = std::cmp::max(self.jump - displacement, 0);
    }

    pub fn move_down(&mut self, displacement: i32) {
        self.py += displacement;
    }

    pub fn set_jump(&mut self, jump_force: i32) {
        self.jump = jump_force;
    }

    pub fn set_ground(&mut self, is_on_ground: bool) {
        self.on_ground = is_on_ground;
    }

    pub fn decr_cooldown(&mut self) {
        self.jump_cooldown -= 1;
    }

    pub fn init_dave_position(&mut self, pos: (u16, u16)) {
        self.px = (pos.0 * config::GAME_TILE_SIZE) as i32;
        self.py = (pos.1 * config::GAME_TILE_SIZE) as i32;
    }
}
