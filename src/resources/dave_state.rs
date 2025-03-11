// dave_state.rs
use crate::resources::direction::Direction;

#[derive(Debug, Clone)]
pub struct DaveState {
    // Position and animation timers (pixel and grid)
    pub px: i32,         // Dave's pixel X position
    pub py: i32,         // Dave's pixel Y position
    pub grid_x: u32,     // Dave's grid X position
    pub grid_y: u32,     // Dave's grid Y position
    pub tick: u32,       // Animation timer for Dave
    pub dead_timer: u32, // Countdown timer when Dave dies

    // Abilities and state flags
    pub can_climb: bool, // Flag: standing on climbable tile
    pub on_ground: bool, // Flag: Dave is on the ground
    pub gun: bool,       // Flag: Dave has a weapon
    pub trophy: bool,    // Flag: Dave has collected the level trophy

    // Jetpack info: fuel level and delay timer
    pub jetpack: u32,       // Fuel level (0 if not active)
    pub jetpack_delay: u32, // Delay to prevent rapid toggling
    pub jump_timer: u32,    // Timer controlling jump characteristics

    // Last known direction Dave is facing
    pub last_dir: Direction,
}

impl Default for DaveState {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            grid_x: 0,
            grid_y: 0,
            tick: 0,
            dead_timer: 0,
            can_climb: false,
            on_ground: false,
            gun: false,
            trophy: false,
            jetpack: 0,
            jetpack_delay: 0,
            jump_timer: 0,
            last_dir: Direction::Right,
        }
    }
}
