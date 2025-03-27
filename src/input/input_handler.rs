use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};
use std::collections::HashSet;

pub struct InputHandler {
    event_pump: EventPump,
    current_game_keys: HashSet<Keycode>,
    just_pressed_game_keys: HashSet<Keycode>,
}

impl InputHandler {
    /// Create a new InputHandler
    pub fn new(sdl_ctx: &Sdl) -> Result<Self, String> {
        let event_pump = sdl_ctx.event_pump()?;
        Ok(Self {
            event_pump,
            current_game_keys: HashSet::new(),
            just_pressed_game_keys: HashSet::new(),
        })
    }

    /// Call this every frame to process input events
    pub fn handle_input(&mut self) -> bool {
        let mut quit = false;
        self.just_pressed_game_keys.clear(); // reset one-shot keys

        let events: Vec<Event> = self.event_pump.poll_iter().collect();
        for event in &events {
            if Self::is_quit_event(event) {
                quit = true;
            }
            self.update_game_key_state(event);
        }

        return quit;

        // (events, quit)
    }

    fn update_game_key_state(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } if Self::is_game_key(*key) => {
                if !self.current_game_keys.contains(key) {
                    self.just_pressed_game_keys.insert(*key);
                }
                self.current_game_keys.insert(*key);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } if Self::is_game_key(*key) => {
                self.current_game_keys.remove(key);
            }
            _ => {}
        }
    }

    /// Keys held down this frame
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.current_game_keys.contains(&key)
    }

    /// Keys pressed **this frame only**
    pub fn is_key_just_pressed(&self, key: Keycode) -> bool {
        self.just_pressed_game_keys.contains(&key)
    }

    fn is_game_key(key: Keycode) -> bool {
        matches!(
            key,
            Keycode::W
                | Keycode::A
                | Keycode::S
                | Keycode::D
                | Keycode::Up
                | Keycode::Down
                | Keycode::Left
                | Keycode::Right
                | Keycode::LCtrl
                | Keycode::RCtrl
                | Keycode::LAlt
                | Keycode::RAlt
        )
    }

    fn is_quit_event(event: &Event) -> bool {
        matches!(
            event,
            Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::F10),
                    ..
                }
        )
    }
}
