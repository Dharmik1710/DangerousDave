use crate::game::state::GameState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};

pub struct InputHandler {
    pub event_pump: EventPump,
}

impl InputHandler {
    pub fn new(sdl_cxt: &Sdl) -> Result<Self, String> {
        let event_pump = sdl_cxt
            .event_pump()
            .expect("Failed to initialize event pump");

        Ok(Self { event_pump })
    }

    pub fn handle_input(&mut self, state: &mut GameState) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("User requested quit.");
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("User pressed 'Q' or 'Escape'. Exiting...");
                    return true;
                }
                // Event::KeyDown {
                //     keycode: Some(Keycode::A),
                //     ..
                // }
                // | Event::KeyDown {
                //     keycode: Some(Keycode::Left),
                //     ..
                // } => {
                //     state.camera.move_left();
                // }
                // Event::KeyDown {
                //     keycode: Some(Keycode::D),
                //     ..
                // }
                // | Event::KeyDown {
                //     keycode: Some(Keycode::Right),
                //     ..
                // } => {
                //     state.camera.move_right();
                // }
                _ => {} // Ignore other events
            }
        }
        false // Continue game loop if no quit signal
    }
}
