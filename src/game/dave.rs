use sdl2::libc::SAE_CONNID_ALL;

use crate::resources::direction::Direction;

#[derive(Debug, Clone)]
pub struct Dave {
    pub x: u16,
    pub y: u16,
    pub px: i32,
    pub py: i32,
    pub h: u16,
    pub w: u16,
    pub speed: u8,
    pub direction: Direction,
    pub on_ground: bool,
    pub jumping: bool,
}

impl Dave {
    pub fn new(scale: f32) -> Self {
        Self {
            x: 0,
            y: 0,
            px: 0,
            py: 0,
            h: (16.0 * scale).round() as u16,
            w: (20.0 * scale).round() as u16,
            speed: 0,
            direction: Direction::Chill,
            on_ground: true,
            jumping: false,
        }
    }
    // pub fn move_left(&mut self) {
    //     self.x -= self.speed;
    //     self.direction = Direction::Left;
    // }

    // pub fn move_right(&mut self) {
    //     self.x += self.speed;
    //     self.direction = Direction::Right;
    // }

    // pub fn jump(&mut self) {
    //     if self.on_ground {
    //         self.jumping = true;
    //         self.jump_timer = 15; // Number of frames to move up
    //         self.on_ground = false;
    //     }
    // }

    pub fn update_position(&mut self, pos: (u16, u16), tile_size: u16) {
        self.x = pos.0;
        self.y = pos.1;
        self.px = (pos.0 * tile_size) as i32;
        self.py = (pos.1 * tile_size) as i32;
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
