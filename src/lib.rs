// Core game components
pub mod game {
    pub mod game;
    pub mod game_loop;
    pub mod level;
    pub mod player;
    pub mod enemy;
    pub mod bullet;
    pub mod camera;
    pub mod state;
    pub mod actions;
}

// Input Handling
pub mod input {
    pub mod keyboard;
    pub mod gamepad;
    pub mod input_handler;
}

// Physics Engine
pub mod physics {
    pub mod physics;
    pub mod collisions;
    pub mod gravity;
}

// Rendering System
pub mod render {
    pub mod renderer;
    pub mod tile_atlas;
    pub mod render_utils;
}

// Resource Management
pub mod resources {
    pub mod app_resources;
    pub mod textures;
    pub mod direction;
    pub mod texture_manager;
}

// Utility Functions
pub mod utils {
    pub mod logger;
    pub mod file;
    pub mod math;
}
