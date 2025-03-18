use sdl2::image::LoadTexture;
use sdl2::Sdl;
use std::time::{Duration, Instant};

use crate::game::state::GameState;
use crate::input::input_handler::InputHandler;
use crate::physics::physics::PhysicsEngine;
use crate::render::renderer::Renderer;

const FRAME_TIME_MS: u64 = 1000 / 60; // 60 FPS → 16ms per frame

pub struct GameLoop;

impl GameLoop {
    pub fn start(state: &mut GameState, sdl_cxt: &Sdl) -> Result<(), Box<dyn std::error::Error>> {
        // SDL Timer
        let mut timer_subsystem = sdl_cxt.timer()?;

        // Renderer init
        let mut renderer = Renderer::new(sdl_cxt)?;

        // load texture
        let texture_creator = renderer.canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/dangerous_dave_game_resources.png")?;

        // Initialize input handler
        let mut input_handler = InputHandler::new(sdl_cxt)?;

        // Initialize level
        state.init_level(&renderer);

        // Main game loop
        'running: loop {
            let frame_start = Instant::now(); // Start frame timer

            if input_handler.handle_input(state) {
                break 'running;
            }

            // update game
            Self::game_update(state, &input_handler);

            // Game logic update goes here (player movement, collision handling, etc.)
            renderer.render(state, &texture);

            // Calculate frame duration
            let frame_time = frame_start.elapsed();

            // Sleep if frame is too fast
            if frame_time < Duration::from_millis(FRAME_TIME_MS) {
                let delay = (FRAME_TIME_MS - frame_time.as_millis() as u64) as u32;
                timer_subsystem.delay(delay);
            }
        }

        println!("Game loop has ended. Cleaning up...");
        Ok(())
    }

    pub fn game_update(state: &mut GameState, input_handler: &InputHandler) {
        // ✅ Apply physics (gravity, movement, jumping)
        PhysicsEngine::apply_physics(state, &input_handler);
    }
}
