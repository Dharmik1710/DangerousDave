pub mod rendering; 

use sdl2::{Sdl, render::WindowCanvas, video::WindowBuilder};
use sdl2::video::Window;
use sdl2::render::Canvas;

/// Initializes SDL2, creates a window, and returns a Canvas to draw on.
pub fn init_sdl(title: &str, width: u32, height: u32) -> (Sdl, WindowCanvas) {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");

    // Initialize the video subsystem
    let video_subsystem = sdl_context.video().expect("Failed to initialize video subsystem");

    // Create a window builder
    let window: Window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .resizable()
        .build()
        .expect("Failed to create a window");

    // Create a canvas (hardware-accelerated if available)
    let canvas: Canvas<Window> = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Failed to create a Canvas");

    (sdl_context, canvas)
}
