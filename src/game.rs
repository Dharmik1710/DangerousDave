pub mod game_utils;
pub mod gameloop; // Expose the `gameloop` module
pub mod level_utils;

use crate::resources::state::GameState;

pub fn run() {
    // In a bigger project, you might set up logging, parse CLI args, etc. here
    println!("Starting Dangerous Dave...");

    // 1. Init Game state
    let mut game_state = GameState::default();

    // Delegate to the game loop
    gameloop::start(&mut game_state);
}
