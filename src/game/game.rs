use crate::game::game_loop::GameLoop;
use crate::game::state::GameState;
use sdl2::Sdl;

pub struct Game {
    state: GameState,
    sdl_cxt: Sdl,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let sdl_cxt = sdl2::init()?;
        let state = GameState::default();
        Ok(Self { state, sdl_cxt })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting Dangerous Dave...");
        GameLoop::start(&mut self.state, &self.sdl_cxt)
    }
}
