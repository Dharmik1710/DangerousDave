use crate::game::dave::Dave;
use crate::game::state::GameState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};
use std::collections::HashSet;

pub struct InputHandler {
    event_pump: EventPump,
    pressed_keys: HashSet<Keycode>,
}

impl InputHandler {
    /// Creates a new `InputHandler`
    pub fn new(sdl_cxt: &Sdl) -> Result<Self, String> {
        let event_pump = sdl_cxt.event_pump()?; // ✅ Handle error properly
        Ok(Self {
            event_pump,
            pressed_keys: HashSet::new(),
        })
    }

    /// Handles input events, updates pressed keys, and returns `true` if the game should quit
    pub fn handle_input(&mut self, state: &mut GameState) -> bool {
        let mut quit = false;

        // ✅ Step 1: Collect events in a temporary vector (Prevents multiple borrows)
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        // ✅ Step 2: Process key events (Update pressed keys)
        for event in &events {
            if Self::is_quit_event(event) {
                quit = true;
            }
            self.update_pressed_keys(event);
        }

        // ✅ Step 3: Process movement **after** handling key presses
        self.process_movement(state);

        quit // ✅ Return `true` only if quit event was detected
    }

    /// Updates the `pressed_keys` HashSet based on key press and release events
    fn update_pressed_keys(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key),
                repeat,
                ..
            } if !repeat => {
                self.pressed_keys.insert(*key);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                self.pressed_keys.remove(key);
            }
            _ => {}
        }
    }

    /// Processes movement based on currently pressed keys
    fn process_movement(&self, state: &mut GameState) {
        if self.is_key_pressed(Keycode::W) || self.is_key_pressed(Keycode::Up) {
            Dave::move_up(state);
        }
        if self.is_key_pressed(Keycode::S) || self.is_key_pressed(Keycode::Down) {
            Dave::move_down(state);
        }
        if self.is_key_pressed(Keycode::A) || self.is_key_pressed(Keycode::Left) {
            Dave::move_left(state);
        }
        if self.is_key_pressed(Keycode::D) || self.is_key_pressed(Keycode::Right) {
            Dave::move_right(state);
        }
    }

    /// Returns `true` if the key is currently pressed
    fn is_key_pressed(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Checks if the event is a quit event (Exit conditions)
    fn is_quit_event(event: &Event) -> bool {
        matches!(
            event,
            Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q | Keycode::Escape),
                    ..
                }
        )
    }
}
