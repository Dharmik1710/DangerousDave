use super::collisions::CollisionDetector;
use super::gravity::Gravity;
use crate::game::state::{self, GameState};
use crate::input::input_handler::InputHandler;
use crate::resources::direction::{self, Direction};
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn apply_physics(state: &mut GameState, input_handler: &InputHandler) {
        // Update dave state as per collision
        if input_handler.is_key_pressed(Keycode::W) || input_handler.is_key_pressed(Keycode::Up) {
            state.dave.jump();
        }
        if input_handler.is_key_pressed(Keycode::A) || input_handler.is_key_pressed(Keycode::Left) {
            let displacement = CollisionDetector::check_collision(state, Direction::Left);
            state.dave.move_left(displacement);
        }
        if input_handler.is_key_pressed(Keycode::D) || input_handler.is_key_pressed(Keycode::Right)
        {
            let displacement = CollisionDetector::check_collision(state, Direction::Right);
            state.dave.move_right(displacement);
        }

        // Apply gravity
        Gravity::apply_gravity(state);
    }
}
