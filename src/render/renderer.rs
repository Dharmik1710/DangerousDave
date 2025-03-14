use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use super::tile_atlas::TileAtlas;
use crate::game::dave::Dave;
use crate::game::level::Level;
use crate::game::state::GameState;

pub struct Renderer {
    pub canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(sdl_cxt: &Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_cxt.video()?;
        let window = video_subsystem
            .window("Dangerous Dave", 800, 600)
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

        Ok(Self { canvas })
    }

    // /// ✅ Corrected `load_texture()` to properly assign texture
    // pub fn load_texture(&mut self, texture: &Texture) -> Result<(), String> {
    //     let texture_creator = &self.texture_creator;
    //     let texture = texture_creator.load_texture("assets/dangerous_dave_game_resources.bmp")?;
    //     self.texture = Some(texture); // ✅ Wrap in Some()
    //     Ok(())
    // }

    pub fn render(&mut self, state: &GameState, texture: &Texture) {
        self.canvas.clear();

        self.render_tiles(&state.level, texture);
        self.render_dave(&state.dave, texture);

        // Self::render_enemies(&state.enemies, canvas, texture);
        // Self::render_bullets(&state.bullets, canvas, texture);

        self.canvas.present();
    }

    fn render_tiles(&mut self, level: &Level, texture: &Texture) {
        for (tile_id, rects) in &level.batches {
            let src_rect = TileAtlas::get_offset(*tile_id);
            for rect in rects {
                if let Err(e) = self.canvas.copy(texture, src_rect, *rect) {
                    eprintln!("Failed to render tile: {}", e);
                }
            }
        }
    }

    fn render_dave(&mut self, dave: &Dave, texture: &Texture) {
        let src_rect = TileAtlas::get_dave();
        let dest_rect = Rect::new(dave.px, dave.py, dave.w as u32, dave.h as u32);
        if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
            eprintln!("Failed to render dave: {}", e);
        }
    }

    // fn render_enemies(enemies: &[Enemy], canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &Texture) {
    //     for enemy in enemies {
    //         let dest_rect = Rect::new(enemy.x, enemy.y, 32, 32);
    //         if let Err(e) = canvas.copy(texture, None, dest_rect) {
    //             eprintln!("Failed to render enemy: {}", e);
    //         }
    //     }
    // }

    // fn render_bullets(bullets: &[Bullet], canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &Texture) {
    //     for bullet in bullets.iter().filter(|b| b.active) {
    //         let dest_rect = Rect::new(bullet.x, bullet.y, 10, 10);
    //         if let Err(e) = canvas.copy(texture, None, dest_rect) {
    //             eprintln!("Failed to render bullet: {}", e);
    //         }
    //     }
    // }
}
