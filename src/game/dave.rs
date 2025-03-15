use crate::{config, resources::direction::Direction};

#[derive(Debug, Clone)]
pub struct Dave {
    pub px: i32,
    pub py: i32,
    pub direction: Direction,
    pub on_ground: bool,
    pub jumping: bool,
}

impl Default for Dave {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            direction: Direction::Chill,
            on_ground: true,
            jumping: false,
        }
    }
}

impl Dave {
    pub fn move_left(&mut self) {
        self.px -= config::DAVE_SPEED;
        self.direction = Direction::Left;
    }

    pub fn move_right(&mut self) {
        self.px += config::DAVE_SPEED;
        self.direction = Direction::Right;
    }

    pub fn move_up(&mut self) {
        self.py -= config::DAVE_SPEED;
        self.direction = Direction::Up;
    }

    pub fn move_down(&mut self) {
        self.py += config::DAVE_SPEED;
        self.direction = Direction::Down;
    }

    // pub fn jump(&mut self) {
    //     if self.on_ground {
    //         self.jumping = true;
    //         self.jump_timer = 15; // Number of frames to move up
    //         self.on_ground = false;
    //     }
    // }

    pub fn init_dave_position(&mut self, pos: (u16, u16)) {
        self.px = (pos.0 * *config::GAME_TILE_SIZE) as i32;
        self.py = (pos.1 * *config::GAME_TILE_SIZE) as i32;
    }

    // pub fn update(&mut self) {
    //     if self.jumping {
    //         self.y -= 5; // Jump up
    //         self.jump_timer -= 1;
    //         if self.jump_timer == 0 {
    //             self.jumping = false;
    //         }
    //     } else {
    //         PhysicsEngine::apply_gravity(self);
    //     }
    // }
}
