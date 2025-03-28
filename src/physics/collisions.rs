use crate::config::{COLLECTIBLES, GAME_TILE_SIZE, SOLID_TILES};
use crate::game::level::Level;
use crate::resources::camera::Camera;
use crate::resources::direction::Direction;
use sdl2::rect::{Point, Rect};

pub struct CollisionDetector;

impl CollisionDetector {
    /// Checks if any corner of rect collides with a solid tile
    pub fn check_solid_tile_collision(
        level: &Level,
        camera: Camera,
        rect: Rect,
        direction: Direction,
    ) -> bool {
        // Extract all relevant corners
        let corners = Self::get_corners(rect, direction);

        for corner in &corners {
            // Convert pixel coordinates to tile index (floor division)
            let tile_x = (corner.x as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile_y = (corner.y as f32 / GAME_TILE_SIZE as f32).floor() as u32;

            // Retrieve the tile rectangle from TileAtlas
            let tile = level.get_tile(camera.x, tile_x, tile_y);

            // Check for intersection with Dave’s rectangle
            if Self::is_solid(tile) {
                let tile_rect = Rect::new(
                    (tile_x * GAME_TILE_SIZE) as i32,
                    (tile_y * GAME_TILE_SIZE) as i32,
                    GAME_TILE_SIZE,
                    GAME_TILE_SIZE,
                );
                // Use strict overlap condition instead of `has_intersection()`
                if Self::is_strict_overlap(rect, tile_rect) {
                    return true; // Collision detected!
                }
            }
        }
        false
    }

    pub fn get_corners(rect: Rect, direction: Direction) -> Vec<(Point)> {
        match direction {
            Direction::Up => vec![rect.top_left(), rect.top_right()],
            Direction::Down => vec![rect.bottom_left(), rect.bottom_right()],
            Direction::Left => vec![rect.top_left(), rect.bottom_left()],
            Direction::Right => vec![rect.top_right(), rect.bottom_right()],
            Direction::Chill => vec![
                rect.top_left(),
                rect.top_right(),
                rect.bottom_left(),
                rect.bottom_right(),
            ], // return all four corners
        }
    }

    fn is_strict_overlap(rect1: Rect, rect2: Rect) -> bool {
        rect1.x < rect2.x + rect2.width() as i32
            && rect1.x + rect1.width() as i32 > rect2.x
            && rect1.y < rect2.y + rect2.height() as i32
            && rect1.y + rect1.height() as i32 > rect2.y
    }

    /// Check collision in general, given two rects
    pub fn check_collision(rect1: Rect, rect2: Rect) -> bool {
        // check for enemy tile collision
        if Self::is_strict_overlap(rect1, rect2) {
            return true;
        }
        false
    }

    // pub fn get_displacement(tile_x: u32, tile_y: u32, dave: &Dave, direction: Direction) -> u32 {
    //     let tile_size = GAME_TILE_SIZE as i32;
    //     let dave_h = DAVE_CHILL_H as i32;
    //     let dave_w = DAVE_CHILL_W as i32;

    //     // be extra careful with the u32 to i32 conversions
    //     match direction {
    //         Direction::Left => dave.px - (tile_x + 1) * tile_size,
    //         Direction::Right => (tile_x * tile_size) - (dave.px + dave_w),
    //         Direction::Up => dave.py - (tile_y + 1) * tile_size,
    //         Direction::Down => tile_y * tile_size - (dave.py + dave_h),
    //         Direction::Chill => 0,
    //     }
    // }

    pub fn is_solid(tile: u8) -> bool {
        SOLID_TILES.contains(&tile)
    }

    pub fn is_collectible(tile: &u8) -> bool {
        COLLECTIBLES.contains_key(tile)
    }
}
