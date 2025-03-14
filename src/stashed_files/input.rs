use crate::resources::app_resources::AppResources;
use crate::resources::state::GameState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// pub fn handle_input(app_resources: &mut AppResources, state: &mut GameState) -> bool {
//     // Process events
//     for event in app_resources.event_pump.poll_iter() {
//         match event {
//             // Quit the game if the user closes the window
//             Event::Quit { .. } => {
//                 println!("User requested quit.");
//                 return false; // Signal to exit the game loop
//             }
//             // Quit on 'Q' or 'Escape'
//             Event::KeyDown { keycode: Some(Keycode::Q), .. }
//             | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                 println!("User pressed 'Q' or 'Esc'. Exiting...");
//                 return false;
//             }
//             // Move camera left ('A' or Left Arrow)
//             Event::KeyDown { keycode: Some(Keycode::A), .. }
//             | Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
//                 state.camera.x_offset = state.camera.x_offset.saturating_sub(1);
//                 println!("Moved camera left: x_offset = {}", state.camera.x_offset);
//             }
//             // Move camera right ('D' or Right Arrow)
//             Event::KeyDown { keycode: Some(Keycode::D), .. }
//             | Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
//                 state.camera.x_offset = state.camera.x_offset.saturating_add(1);
//                 println!("Moved camera right: x_offset = {}", state.camera.x_offset);
//             }
//             _ => {} // Ignore other events
//         }
//     }
//     true // Continue running
// }
pub fn handle_input(app_resources: &mut AppResources, state: &mut GameState) -> bool {
    for event in app_resources.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                println!("User requested quit.");
                return true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => {
                println!("User pressed 'Q'. Exiting...");
                return true;
            }
            _ => {} // Ignore other events
        }
    }
    false // Continue game loop if no quit signal
}
