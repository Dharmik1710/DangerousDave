use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

/// Represents a static object (e.g., a "wall") for demonstration.
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Wall {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

/// Draw the Wall onto the provided canvas.
pub fn draw_wall(canvas: &mut WindowCanvas, wall: &Wall) {
    // Set draw color (brownish) for the wall
    canvas.set_draw_color(Color::RGB(139, 69, 19));

    let wall_rect = Rect::new(wall.x, wall.y, wall.width, wall.height);

    // Fill the rectangle
    if let Err(e) = canvas.fill_rect(wall_rect) {
        eprintln!("Failed to draw wall: {e}");
    }
}
