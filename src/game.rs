pub mod gameloop; // Expose the `gameloop` module

pub fn run() {
    // In a bigger project, you might set up logging, parse CLI args, etc. here
    println!("Starting Dangerous Dave...");

    // Delegate to the game loop
    gameloop::start();
}