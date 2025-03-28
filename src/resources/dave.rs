use std::collections::HashMap;

use super::{bullet::Bullet, camera::Camera};
use crate::{
    animation::{
        animation::{Animation, AnimationState},
        animation_registry::AnimationRegistry,
    },
    config::{
        DAVE_BULLET_TILE, DAVE_CHILL_H, DAVE_CHILL_W, DAVE_DEFAULT_TILE, DAVE_JUMP,
        DAVE_JUMP_COOLDOWN, DAVE_SPEED, DAVE_SPEED_X, DEAD_TIMER, GAME_TILE_SIZE, SCALE,
    },
    game::level::Level,
    physics::collisions::CollisionDetector,
    render::tile_atlas::TileAtlas,
    resources::direction::Direction,
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
    pub is_alive: bool,
    pub bullet: Bullet,
    pub dead_timer: i8,
    pub tile: u8,
    pub is_jetpack_active: bool,
    pub jetpack: u32,
    pub has_trophy: bool,
    pub has_gun: bool,
    pub on_door: bool,
    pub current_animation: AnimationState,
    pub animations: HashMap<AnimationState, Animation>, // Stores all animations - jumping, running, climbing etc.
    pub idle: bool,
}

impl Default for Dave {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            jump: 0,
            jump_cooldown: DAVE_JUMP_COOLDOWN,
            direction: Direction::Chill,
            on_ground: true,
            is_alive: true,
            dead_timer: DEAD_TIMER,
            tile: DAVE_DEFAULT_TILE,
            bullet: Bullet::new(DAVE_BULLET_TILE),
            jetpack: 0,
            is_jetpack_active: false,
            has_trophy: false,
            has_gun: false,
            on_door: false,
            animations: AnimationRegistry::initialize_dave_animations(),
            current_animation: AnimationState::default(),
            idle: true,
        }
    }
}

impl Dave {
    pub fn reset(&mut self, is_level_reset: bool) {
        if is_level_reset {
            *self = Self::default()
        } else {
            self.px = 0;
            self.py = 0;
            self.jump = 0;
            self.jump_cooldown = DAVE_JUMP_COOLDOWN;
            self.direction = Direction::Chill;
            self.on_ground = true;
            self.is_alive = true;
            self.dead_timer = DEAD_TIMER;
            self.tile = DAVE_DEFAULT_TILE;
            self.bullet.deactivate();
            self.is_jetpack_active = false;
            self.on_door = false;
            self.animations = AnimationRegistry::initialize_dave_animations();
            self.current_animation = AnimationState::default();
            self.idle = true;
        }
    }

    pub fn get_dave_init_pos(level_num: u8) -> (u32, u32) {
        let dave_init_pos = match level_num {
            1 => (2, 8),
            2 => (1, 8),
            3 => (2, 5),
            4 => (1, 5),
            5 => (2, 8),
            6 => (2, 8),
            7 => (1, 2),
            8 => (2, 8),
            9 => (6, 1),
            10 => (2, 8),
            _ => (0, 0), // Default fallback
        };
        return dave_init_pos;
    }

    pub fn move_left(&mut self) {
        if self.is_alive {
            self.direction = Direction::Left;
            self.px = self.px.saturating_sub(DAVE_SPEED_X);
        }
    }

    pub fn move_right(&mut self) {
        if self.is_alive {
            self.direction = Direction::Right;
            self.px += DAVE_SPEED_X;
        }
    }

    pub fn set_idle(&mut self, is_idle: bool) {
        self.idle = is_idle;
    }

    pub fn jump(&mut self) {
        if self.on_ground && self.jump == 0 {
            self.jump = DAVE_JUMP; // Set jump height
            self.on_ground = false; // Dave is no longer on the ground
        }
    }

    pub fn dead(&mut self) {
        self.is_alive = false;
        self.set_animation();
    }

    pub fn move_up(&mut self) {
        if self.is_alive {
            self.py -= DAVE_SPEED;
            self.jump = std::cmp::max(self.jump as i32 - DAVE_SPEED as i32, 0) as u32;
        }
    }

    pub fn move_down(&mut self) {
        if self.is_alive {
            self.py += DAVE_SPEED;
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
        self.dead_timer -= 1;
    }

    pub fn shoot(&mut self) {
        // shoot only when all of these are true
        //  - bullet is not active
        //  - when ctrl key is pressed
        if self.has_gun && !self.bullet.is_active {
            self.bullet
                .fire(self.px as i32, self.py as i32, self.direction, self.tile);
        }
    }

    pub fn toggle_jetpack(&mut self) {
        if self.jetpack > 0 {
            self.is_jetpack_active = !self.is_jetpack_active;
            self.set_animation();
        }
    }

    pub fn init_dave_position(&mut self, level_num: u8) {
        let dave_init_position = Self::get_dave_init_pos(level_num);
        self.px = dave_init_position.0 * GAME_TILE_SIZE;
        self.py = (dave_init_position.1 + 1) * GAME_TILE_SIZE;
    }

    pub fn get_animation_state(&mut self) -> AnimationState {
        if !self.is_alive {
            return AnimationState::Dying;
        }

        // when jetpack is active
        if self.is_jetpack_active && self.jetpack > 0 {
            if self.direction.is_left() {
                return AnimationState::JetpackingLeft;
            } else {
                return AnimationState::JetpackingRight;
            }
        }

        // in air while jumping up
        if !self.on_ground || self.jump > 0 {
            if self.direction.is_left() {
                return AnimationState::JumpingLeft;
            } else {
                return AnimationState::JumpingRight;
            }
        }

        // if not doing anything
        if self.idle {
            if self.direction.is_left() {
                return AnimationState::IdleLeft;
            } else if self.direction.is_right() {
                return AnimationState::IdleRight;
            } else {
                return AnimationState::Idle;
            }
        }

        // on ground, movement in left or right direction
        if self.on_ground && self.jump == 0 {
            if self.direction.is_left() {
                return AnimationState::RunningLeft;
            } else {
                return AnimationState::RunningRight;
            }
        }
        AnimationState::Idle
    }

    pub fn set_animation(&mut self) {
        let animation_state = self.get_animation_state();
        if self.current_animation != animation_state {
            self.current_animation = animation_state;
            self.animations
                .get_mut(&animation_state)
                .unwrap()
                .reset_frame_timer(); // Reset frame timer
        }
    }

    pub fn update_animation(&mut self) {
        self.set_animation();
        self.animations
            .get_mut(&self.current_animation)
            .unwrap()
            .update();
    }

    pub fn update_position(&mut self, x_shift: i32) {
        // x_shift is bounded and handled by default
        self.px = ((self.px as i32) - (x_shift * GAME_TILE_SIZE as i32)) as u32;
    }

    pub fn get_rect(&self, direction: Direction) -> Rect {
        // let dave_rect = TileAtlas::get_animation_tile(self);
        // let hitbox_w = dave_rect.width() * SCALE;
        // let hitbox_h = dave_rect.height() * SCALE;
        let hitbox_w = DAVE_CHILL_W;
        let hitbox_h = DAVE_CHILL_H;

        // be extra careful with the u32 to i32 conversions
        match direction {
            Direction::Up => Rect::new(
                self.px as i32,
                self.py.saturating_sub(DAVE_SPEED) as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Down => Rect::new(
                self.px as i32,
                (self.py + DAVE_SPEED) as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Left => Rect::new(
                self.px.saturating_sub(DAVE_SPEED_X) as i32,
                self.py as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Right => Rect::new(
                (self.px + DAVE_SPEED_X) as i32,
                self.py as i32,
                hitbox_w,
                hitbox_h,
            ),
            Direction::Chill => Rect::new(self.px as i32, self.py as i32, hitbox_w, hitbox_h),
        }
    }

    pub fn is_level_complete(&self) -> bool {
        self.is_alive && self.has_trophy && self.on_door
    }

    pub fn update_bullet(&mut self, level: &Level, camera: Camera) {
        if self.has_gun && self.bullet.is_active {
            // check bullet - solid tiles collision
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
        }
    }

    pub fn use_jetpack(&mut self) {
        if self.is_jetpack_active {
            if self.jetpack > 0 {
                self.jetpack -= 1;
            } else {
                self.is_jetpack_active = false;
            }
        }
    }
}
