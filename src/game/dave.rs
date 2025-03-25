use super::{init::Initialize, level, state::GameState};
use crate::{
    config::{self, DAVE_DEFAULT_TILE, DAVE_JUMP, DAVE_JUMP_COOLDOWN, DEAD_TIMER, GAME_TILE_SIZE},
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
    pub px: u32,
    pub py: u32,
    pub jump: u32,
    pub jump_cooldown: u32,
    pub direction: Direction,
    pub on_ground: bool,
    pub score: u32,
    pub is_alive: bool,
    pub dead_timer: i8,
    pub tile: u8,
}

impl Default for Dave {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            jump: 0,
            jump_cooldown: 0,
            direction: Direction::Chill,
            on_ground: true,
            score: 0,
            is_alive: true,
            dead_timer: DEAD_TIMER,
            tile: DAVE_DEFAULT_TILE,
        }
    }
}

impl Dave {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn move_left(&mut self, displacement: u32) {
        if self.is_alive {
            self.px -= displacement;
        }
    }

    pub fn move_right(&mut self, displacement: u32) {
        if self.is_alive {
            self.px += displacement;
        }
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

    pub fn dead(&mut self) {
        self.is_alive = false;
    }

    pub fn collect(&mut self, points: u32) {
        self.score += points;
    }

    pub fn move_up(&mut self, displacement: u32) {
        if self.is_alive {
            self.py -= displacement;
            self.jump = std::cmp::max(self.jump - displacement, 0);
        }
    }

    pub fn move_down(&mut self, displacement: u32) {
        if self.is_alive {
            self.py += displacement;
        }
    }

    pub fn set_jump(&mut self, jump_force: u32) {
        if self.is_alive {
            self.jump = jump_force;
        }
    }

    pub fn set_ground(&mut self, is_on_ground: bool) {
        self.on_ground = is_on_ground;
    }

    pub fn decr_cooldown(&mut self) {
        self.jump_cooldown -= 1;
    }

    pub fn decr_dead_timer(&mut self) {
        if self.dead_timer > 0 && !self.is_alive {
            self.dead_timer -= 1;
        }
    }

    pub fn init_dave_position(&mut self, level_num: u8) {
        let dave_init_position = Initialize::get_dave_init_pos(level_num);
        self.px = dave_init_position.0 * GAME_TILE_SIZE;
        self.py = dave_init_position.1 * GAME_TILE_SIZE;
    }

    pub fn update_position(&mut self, x_shift: i32) {
        // x_shift is bounded and handled by default
        self.px = ((self.px as i32) - (x_shift * GAME_TILE_SIZE as i32)) as u32;
    }
}
