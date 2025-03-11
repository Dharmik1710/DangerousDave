use sdl2::{render::Canvas, video::Window, EventPump, Sdl};

use super::state::{self, GameState};

pub struct AppResources {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl<'a> AppResources {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("App", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        canvas
            .window_mut()
            .set_fullscreen(sdl2::video::FullscreenType::Desktop)
            .expect("Failed to set fullscreen mode");

        // set the eventpump for input handling
        let mut event_pump = sdl_context
            .event_pump()
            .expect("Could not obtain SDL Event Pump");

        Ok(Self {
            sdl_context,
            canvas,
            event_pump,
        })
    }
}
