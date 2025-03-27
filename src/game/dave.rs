use super::{
    bullet::Bullet,
    camera::{self, Camera},
    init::Initialize,
    level::{self, Level},
    state::GameState,
};
use crate::{
    config::{
        self, DAVE_BULLET_TILE, DAVE_CHILL_H, DAVE_CHILL_W, DAVE_DEFAULT_TILE, DAVE_JUMP,
        DAVE_JUMP_COOLDOWN, DAVE_SPEED, DAVE_SPEED_X, DEAD_TIMER, GAME_TILE_SIZE, JETPACK_FUEL,
    },
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
    pub is_alive: bool,
    pub bullet: Bullet,
    pub dead_timer: i8,
    pub tile: u8,
    pub is_jetpack_active: bool,
    pub jetpack: u32,
    pub has_trophy: bool,
    pub has_gun: bool,
    pub on_door: bool,
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
        }
    }

    pub fn move_left(&mut self) {
        if self.is_alive {
            self.direction = Direction::Left;
            self.px -= DAVE_SPEED_X;
        }
    }

    pub fn move_right(&mut self) {
        if self.is_alive {
            self.direction = Direction::Right;
            self.px += DAVE_SPEED_X;
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
        //  - the enemy is on screen
        //  - enemy can shoot
        if self.has_gun && !self.bullet.is_active {
            self.bullet
                .fire(self.px as i32, self.py as i32, self.direction, self.tile);
        }
    }

    pub fn toggle_jetpack(&mut self) {
        if self.jetpack > 0 {
            self.is_jetpack_active = !self.is_jetpack_active;
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

    pub fn get_rect(&self, direction: Direction) -> Rect {
        let hitbox_w = DAVE_CHILL_W;
        let hitbox_h = DAVE_CHILL_H;

        // be extra careful with the u32 to i32 conversions
        match direction {
            Direction::Up => Rect::new(
                self.px as i32,
                (self.py - DAVE_SPEED) as i32,
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
                (self.px - DAVE_SPEED_X) as i32,
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
