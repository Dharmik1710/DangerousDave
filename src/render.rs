pub mod render_utils;
pub mod tile_atlas;

use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::{render::WindowCanvas, Sdl};
use tile_atlas::get_offset;

use crate::resources::app_resources::{self, AppResources};
use crate::resources::state::{self, GameState};

/// Initializes SDL2, creates a window, and returns a Canvas to draw on.
pub fn init_sdl(title: &str, width: u32, height: u32) -> (Sdl, WindowCanvas) {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");

    // Initialize the video subsystem
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize video subsystem");

    // Create a window builder
    let window: Window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .resizable()
        .build()
        .expect("Failed to create a window");

    // // Enable Fullscreen Mode
    // window.set_fullscreen(FullscreenType::Desktop).expect("Failed to set fullscreen mode");

    // Create a canvas (hardware-accelerated if available)
    let canvas: Canvas<Window> = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Failed to create a Canvas");

    (sdl_context, canvas)
}

pub fn render_visible_batches(
    app_resources: &mut AppResources,
    state: &GameState,
    texture: &Texture,
) {
    app_resources.canvas.set_draw_color(Color::RGB(0, 0, 0));
    app_resources.canvas.clear();

    let tile_batches: &HashMap<u8, Vec<Rect>> = &state.level.batches;
    for (&tile_index, batch) in tile_batches.iter() {
        let src_rect: Rect = get_offset(tile_index);

        for &dest_rect in batch.iter() {
            app_resources
                .canvas
                .copy(texture, Some(src_rect), dest_rect)
                .unwrap();
        }
    }

    // Future rendering logic here
    app_resources.canvas.present();
}
