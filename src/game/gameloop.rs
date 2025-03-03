use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::graphics::init_sdl;
use crate::graphics::rendering::{Wall, draw_wall};

/// Start the main game loop
pub fn start() {
    // 1. Initialize SDL2 and create a canvas
    let (sdl_context, mut canvas) = init_sdl(
        "Dangerous Dave - Static Wall Example",
        800,  // width
        600,  // height
    );

    // 2. Create an event pump to poll for events
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not obtain SDL Event Pump");

    // 3. Set up a "Wall" object
    let wall = Wall::new(100, 100, 200, 50);

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
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the wall
        draw_wall(&mut canvas, &wall);

        // Present the final image on screen
        canvas.present();
    }

    println!("Game loop has ended. Cleaning up...");
}
