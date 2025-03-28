// Globally accessible config
pub mod config;

// Core game components
pub mod game {
    pub mod game;
    pub mod game_loop;
    pub mod game_manager;
    pub mod game_utils;
    // pub mod game_rules;
    pub mod level;
    pub mod state;
}

// Input Handling
pub mod input {
    pub mod input_handler;
    pub mod player_controller;
}

// Physics Engine
pub mod physics {
    pub mod collisions;
    pub mod gravity;
    pub mod physics;
}

// Rendering System
pub mod render {
    pub mod renderer;
    pub mod tile_atlas;
}

// Resource Management
pub mod resources {
    pub mod bullet;
    pub mod camera;
    pub mod dave;
    pub mod direction;
    pub mod enemy;
}

// Animations module
pub mod animation {
    pub mod animation;
    pub mod animation_registry;
}
