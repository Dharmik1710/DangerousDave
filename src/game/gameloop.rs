use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::video::FullscreenType;

use crate::graphics::init_sdl;
use crate::graphics::rendering::draw_wall_boundary;


/// Start the main game loop
pub fn start() {
    // 1. Initialize SDL2 and create a canvas
    let (sdl_context, mut canvas) = init_sdl(
        "Dangerous Dave - Static Wall Example",
        800,  // width
        600,  // height
    );

    // full screen(without explicitly creating windows mutable in graphics::init_sdl)
    canvas.window_mut().set_fullscreen(FullscreenType::Desktop).expect("Failed to set fullscreen mode");

    // 2. Create an event pump to poll for events
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not obtain SDL Event Pump");
    
    // 2. Load the sprite sheet as a texture
    //    If you're using sdl2_image, you can do:
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
    .load_texture("assets/dave_tiles.png")
    .expect("Could not load the sprite sheet");

    // 3. Draw wall boundary
    // draw_wall_boundary(&mut canvas, &texture);

    // 4. Main game loop
    'running: loop {
        // Process events
        for event in event_pump.poll_iter() {
            match event {
                // If the user clicks the close window button
                Event::Quit { .. } => {
                    println!("User requested quit.");
                    break 'running;
                }
                // If the user presses `Q`
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    println!("User pressed 'Q'. Exiting...");
                    break 'running;
                }
                _ => {}
            }
        }

        // Clear the screen to black before drawing
        // canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        // canvas.clear();

        // 3. Get the "wall" sprite sub-rectangle
        // draw_wall_boundary(&mut canvas, &texture, &wall_sprite);

        // 3. Draw wall boundary
        draw_wall_boundary(&mut canvas, &texture);

        // Present the final image on screen
        canvas.present();

    }

    println!("Game loop has ended. Cleaning up...");
}
