#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    IdleLeft,
    IdleRight,
    RunningLeft,
    RunningRight,
    JumpingLeft,
    JumpingRight,
    JetpackingRight,
    JetpackingLeft,
    Climbing,
    Dying,
}

#[derive(Debug, Clone)]
pub struct Animation {
    frames: Vec<u8>, // Tile numbers of animation frames
    current_frame: usize,
    frame_timer: u32, // Frame delay timer
    frame_delay: u32, // Time per frame before switching
}

impl Animation {
    pub fn new(frames: Vec<u8>, frame_delay: u32) -> Self {
        Self {
            frames,
            current_frame: 0,
            frame_timer: 0,
            frame_delay,
        }
    }

    pub fn update(&mut self) {
        self.frame_timer += 1;
        if self.frame_timer >= self.frame_delay {
            self.frame_timer = 0;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    pub fn current_tile(&self) -> u8 {
        self.frames[self.current_frame]
    }

    pub fn reset_frame_timer(&mut self) {
        self.frame_timer = 0;
    }
}
