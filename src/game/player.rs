use crate::resources::direction::Direction;

#[derive(Debug, Clone)]
pub struct Player {
    pub x: u16,
    pub y: u16,
    pub speed: u8,
    pub direction: Direction,
    pub on_ground: bool,
    pub jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0, // Initial spawn position
            speed: 0,
            direction: Direction::Right,
            on_ground: true,
            jumping: false,
        }
    }
}

impl Player {
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

    pub fn update_position(&mut self, pos: (u16, u16)) {
        self.x = pos.0;
        self.y = pos.1;
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
