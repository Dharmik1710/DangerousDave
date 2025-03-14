// Core game components
pub mod game {
    pub mod actions;
    pub mod bullet;
    pub mod camera;
    pub mod dave;
    pub mod enemy;
    pub mod game;
    pub mod game_loop;
    pub mod level;
    pub mod state;
}

// Input Handling
pub mod input {
    pub mod gamepad;
    pub mod input_handler;
    pub mod keyboard;
}

// Physics Engine
pub mod physics {
    pub mod collisions;
    pub mod gravity;
    pub mod physics;
}

// Rendering System
pub mod render {
    pub mod render_utils;
    pub mod renderer;
    pub mod tile_atlas;
}

// Resource Management
pub mod resources {
    pub mod direction;
}

// Utility Functions
pub mod utils {
    pub mod file;
    pub mod logger;
    pub mod math;
}
