use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use super::tile_atlas::TileAtlas;
use crate::config::{DAVE_CHILL_H, DAVE_CHILL_W, GAME_TILE_SIZE, SCALE};
use crate::game::camera::Camera;
use crate::game::dave::Dave;
use crate::game::enemy::{self, Enemy};
use crate::game::level::Level;
use crate::game::state::GameState;

pub struct Renderer {
    pub canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(sdl_cxt: &Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_cxt.video()?;
        let window = video_subsystem
            .window("Dangerous Dave", 800, 600) // Initial resolution
            .position_centered()
            .resizable() // Allow resizing
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        // Set nearest-neighbor filtering to prevent blurry textures
        sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "0");

        // Set logical size to maintain proper scaling
        canvas
            .set_logical_size(960, 600)
            .map_err(|e| e.to_string())?;

        // Set fullscreen mode (optional, can be removed if not needed)
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
        self.render_enemies(&state.enemies, texture, &state.camera);

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
        let dest_rect = Rect::new(dave.px as i32, dave.py as i32, DAVE_CHILL_W, DAVE_CHILL_H);
        if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
            eprintln!("Failed to render dave: {}", e);
        }
    }

    fn render_enemies(&mut self, enemies: &[Enemy], texture: &Texture, camera: &Camera) {
        for enemy in enemies.iter() {
            if enemy.is_enemy_on_screen(camera) {
                let src_rect = TileAtlas::get_enemy(enemy.enemy_tile);
                let enemy_px = enemy.x - camera.x * GAME_TILE_SIZE;
                let enemy_py = enemy.y;
                let dest_rect = Rect::new(
                    enemy_px as i32,
                    enemy_py as i32,
                    src_rect.width() * SCALE,
                    src_rect.height() * SCALE,
                );
                if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
                    eprintln!("Failed to render enemy: {}", e);
                }
            }
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
