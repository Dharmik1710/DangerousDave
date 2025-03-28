use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use super::tile_atlas::TileAtlas;
use crate::config::{
    DAVE_CHILL_H, DAVE_CHILL_W, DEAD_TILE, GAME_TILE_SIZE, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use crate::game::bullet;
use crate::game::camera::Camera;
use crate::game::dave::Dave;
use crate::game::enemy::{self, Enemy};
use crate::game::level::Level;
use crate::game::state::GameState;
use crate::resources::direction::Direction;

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
            .set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT)
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

        self.render_top_row(&state, texture);
        self.render_tiles(&state.level, texture);
        self.render_dave(&state.dave, texture);
        self.render_enemies(&state.enemies, texture, state.camera);

        // Self::render_enemies(&state.enemies, canvas, texture);
        // Self::render_bullets(&state.bullets, canvas, texture);

        self.canvas.present();
    }

    fn render_tiles(&mut self, level: &Level, texture: &Texture) {
        for (tile_id, rects) in &level.batches {
            let src_rect = TileAtlas::get_rect(*tile_id);
            for rect in rects {
                if let Err(e) = self.canvas.copy(texture, src_rect, *rect) {
                    eprintln!("Failed to render tile: {}", e);
                }
            }
        }
    }

    fn render_dave(&mut self, dave: &Dave, texture: &Texture) {
        // render dave
        if dave.is_alive {
            let src_rect = TileAtlas::get_animation_tile(dave);
            let dest_rect = Rect::new(
                dave.px as i32,
                dave.py as i32,
                src_rect.width() * SCALE,
                src_rect.height() * SCALE,
            );
            if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
                eprintln!("Failed to render dave: {}", e);
            }
        } else if dave.dead_timer > 0 {
            let src_rect = TileAtlas::get_rect(DEAD_TILE);
            let (dead_w, dead_h) = TileAtlas::get_dimension(DEAD_TILE);
            let dest_rect = Rect::new(dave.px as i32, dave.py as i32, dead_w, dead_h);
            if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
                eprintln!("Failed to render dave: {}", e);
            }
        }
        // render bullet
        if dave.has_gun && dave.bullet.is_active {
            let src_rect_bullet = TileAtlas::get_rect(dave.bullet.tile);
            let dest_rect_bullet = dave.bullet.get_rect(Direction::Chill);
            if let Err(e) = self.canvas.copy(texture, src_rect_bullet, dest_rect_bullet) {
                eprintln!("Failed to render enemy nullet: {}", e);
            }
        }
    }

    fn render_enemies(&mut self, enemies: &[Enemy], texture: &Texture, camera: Camera) {
        for enemy in enemies.iter() {
            // render enemy
            if enemy.is_enemy_on_screen(camera) {
                let src_rect_enemy = TileAtlas::get_enemy(enemy.tile);
                let dest_rect_enemy = enemy.get_rect(camera);
                if let Err(e) = self.canvas.copy(texture, src_rect_enemy, dest_rect_enemy) {
                    eprintln!("Failed to render enemy: {}", e);
                }
            } else if enemy.dead_timer > 0 {
                let src_rect = TileAtlas::get_rect(DEAD_TILE);
                let (dead_w, dead_h) = TileAtlas::get_dimension(DEAD_TILE);
                let mut dest_rect = enemy.get_rect(camera);
                dest_rect.set_width(dead_w);
                dest_rect.set_height(dead_h);
                if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
                    eprintln!("Failed to render dave: {}", e);
                }
            }
            // render bullet
            if enemy.can_shoot && enemy.bullet.is_active {
                let src_rect_bullet = TileAtlas::get_bullet(enemy.bullet.direction);
                let dest_rect_bullet = enemy.bullet.get_rect(Direction::Chill);
                if let Err(e) = self.canvas.copy(texture, src_rect_bullet, dest_rect_bullet) {
                    eprintln!("Failed to render enemy nullet: {}", e);
                }
            }
        }
    }

    pub fn render_top_row(&mut self, state: &GameState, texture: &Texture) {
        let base_y = 2 * SCALE as i32;
        let mut x_offset = 3 * SCALE as i32; // Left padding

        // 🚀 Render "SCORE" label
        let score_src_rect = TileAtlas::get_score_text();
        x_offset = self.render_text_tile(texture, score_src_rect, &mut x_offset, base_y);

        // 🎯 Render Score (5 digits)
        x_offset = self.render_digits(texture, &mut x_offset, state.score, 5, base_y);

        x_offset += 10 * SCALE as i32; // Extra padding

        // 🏆 Render "LEVEL" label
        let level_src_rect = TileAtlas::get_level_text();
        x_offset = self.render_text_tile(texture, level_src_rect, &mut x_offset, base_y);

        // 🔢 Render Level (2 digits)
        x_offset = self.render_digits(
            texture,
            &mut x_offset,
            state.current_level as u32,
            2,
            base_y,
        );

        x_offset += 10 * SCALE as i32; // Extra padding

        // ❤️ Render "DAVES" label
        let dave_src_rect = TileAtlas::get_dave_text();
        x_offset = self.render_text_tile(texture, dave_src_rect, &mut x_offset, base_y);

        // 😀 Render Lives (Dave Faces)
        self.render_lives(texture, &mut x_offset, state.lives, base_y);
    }

    fn render_text_tile(
        &mut self,
        texture: &Texture,
        src_rect: Rect,
        x_offset: &mut i32,
        base_y: i32,
    ) -> i32 {
        let dest_rect = Rect::new(
            *x_offset,
            base_y,
            src_rect.width() * SCALE,
            src_rect.height() * SCALE,
        );
        self.canvas.copy(texture, src_rect, dest_rect).ok();
        *x_offset += (src_rect.width() as i32 + 1) * SCALE as i32; // Update x_offset
        *x_offset
    }

    fn render_digits(
        &mut self,
        texture: &Texture,
        x_offset: &mut i32,
        value: u32,
        digits: usize,
        base_y: i32,
    ) -> i32 {
        let formatted_value = format!("{:0width$}", value, width = digits);

        for digit in formatted_value.chars() {
            let digit_index = digit.to_digit(10).unwrap() as i32;
            let src_rect = TileAtlas::get_digit(digit_index);
            let dest_rect = Rect::new(
                *x_offset,
                base_y,
                src_rect.width() * SCALE,
                src_rect.height() * SCALE,
            );
            self.canvas.copy(texture, src_rect, dest_rect).ok();
            *x_offset += src_rect.width() as i32 * SCALE as i32;
        }
        *x_offset
    }

    fn render_lives(
        &mut self,
        texture: &Texture,
        x_offset: &mut i32,
        lives: u32,
        base_y: i32,
    ) -> i32 {
        for _ in 0..lives {
            let src_rect = TileAtlas::get_dave_face();
            let dest_rect = Rect::new(
                *x_offset,
                base_y,
                src_rect.width() * SCALE,
                src_rect.height() * SCALE,
            );
            self.canvas.copy(texture, src_rect, dest_rect).ok();
            *x_offset += (src_rect.width()) as i32 * SCALE as i32;
        }
        *x_offset
    }
}
