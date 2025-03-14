use sdl2::{render::Canvas, video::Window, EventPump, Sdl};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

pub struct AppResources<'a> {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture: Option<Texture<'a>>, // ✅ Corrected Option syntax
}

impl<'a> AppResources<'a> {
    /// ✅ Initializes SDL2 correctly
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Dangerous Dave", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump().expect("Failed to initialize event pump");

        Ok(Self {
            sdl_context,
            canvas,
            event_pump,
            texture: None, // ✅ Initially no texture
        })
    }

    /// ✅ Corrected `load_texture()` to properly assign texture
    pub fn load_texture(&mut self, texture_creator: &'a TextureCreator<WindowContext>) -> Result<(), String> {
        let texture = texture_creator.load_texture("assets/dangerous_dave_game_resources.bmp")?;
        self.texture = Some(texture); // ✅ Wrap in Some()
        Ok(())
    }

    /// ✅ Safely retrieve the texture
    pub fn get_texture(&self) -> Option<&Texture> {
        self.texture.as_ref()
    }
}
