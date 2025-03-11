use crate::game::level_utils::init_level;
use crate::input::handle_input;
use crate::render::render_visible_batches;
use crate::resources::{
    app_resources::AppResources,
    state::GameState,
};
use sdl2::keyboard::Keycode;
use sdl2::{
    event::Event,
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
// use std::time::{Duration, Instant};

// const TARGET_FPS: u32 = 60; // Adjust to your desired FPS
// const FRAME_DELAY: Duration = Duration::from_millis(1000 / TARGET_FPS as u64);

/// Start the main game loop
pub fn start(state: &mut GameState) -> Result<(), String> {
    /// 2. Init assets
    let mut app_resources = AppResources::new()?;

    // load the texture
    let texture_creator: TextureCreator<WindowContext> = app_resources.canvas.texture_creator();
    let texture: Texture = texture_creator
        .load_texture("assets/dangerous_dave_game_resources.bmp")
        .map_err(|e| format!("Texture loading error: {}", e))?;

    // load level
    init_level(state, &app_resources);

    // 4. Main game loop
    'running: loop {

        // // This is done to control the fps
        // let frame_start = Instant::now(); // Record frame start time


        // 3. Handle input
        if handle_input(&mut app_resources, state) {
            break; // Exit loop if `handle_input()` returns `true`
        }

        // 4. Update game
        // update_game();


        // 5. Render
        render_visible_batches(&mut app_resources, state, &texture);

        // // Calculate frame duration and delay if necessary
        // let frame_time = frame_start.elapsed();
        // if frame_time < FRAME_DELAY {
        //     std::thread::sleep(FRAME_DELAY - frame_time);
        // }

    }

    println!("Game loop has ended. Cleaning up...");

    Ok(())
}
