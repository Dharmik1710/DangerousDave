use dangerous_dave::game::game::Game;
use log::{error, info};

fn main() {
    env_logger::init();
    info!("Starting Dangerous Dave...");

    let mut game = match Game::new() {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Error initializing game: {}", e);
            return;
        }
    };

    if let Err(e) = game.run() {
        error!("Game encountered an error: {}", e);
        std::process::exit(1);
    }
}
