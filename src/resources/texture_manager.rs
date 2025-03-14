use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use std::collections::HashMap;

pub struct TextureManager<'a> {
    textures: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Loads a texture if not already loaded and caches it
    pub fn load_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Result<&Texture<'a>, String> {
        let path="assets/dangerous_dave_game_resources.bmp";
        if !self.textures.contains_key(path) {
            let texture = texture_creator.load_texture(path)?;
            self.textures.insert(path.to_string(), texture);
        }
        Ok(self.textures.get(path).unwrap()) // Safe unwrap since we just inserted it
    }

    /// Retrieves an already loaded texture
    pub fn get_texture(&self, path: &str) -> Option<&Texture<'a>> {
        self.textures.get(path)
    }
}
