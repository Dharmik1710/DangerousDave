use crate::{
    game::state::GameState, input::input_handler::InputHandler,
    physics::collisions::CollisionDetector, resources::direction::Direction,
};
use sdl2::keyboard::Keycode;

pub struct PlayerController;

impl PlayerController {
    pub fn handle_input(state: &mut GameState, input: &InputHandler) {
        if !state.dave.is_jetpack_active {
            Self::handle_movement(state, input);
        } else {
            Self::handle_elevated_movement(state, input);
        }
        Self::handle_actions(state, input);
    }

    fn handle_movement(state: &mut GameState, input: &InputHandler) {
        if input.is_key_pressed(Keycode::A) || input.is_key_pressed(Keycode::Left) {
            let rect = state.dave.get_rect(Direction::Left);
            if !CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Left,
            ) {
                state.dave.move_left();
            }
        }

        if input.is_key_pressed(Keycode::D) || input.is_key_pressed(Keycode::Right) {
            let rect = state.dave.get_rect(Direction::Right);
            if !CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Right,
            ) {
                state.dave.move_right();
            }
        }

        if input.is_key_pressed(Keycode::W) || input.is_key_pressed(Keycode::Up) {
            state.dave.jump();
        }
    }

    fn handle_elevated_movement(state: &mut GameState, input: &InputHandler) {
        if input.is_key_pressed(Keycode::A) || input.is_key_pressed(Keycode::Left) {
            let rect = state.dave.get_rect(Direction::Left);
            if !CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Left,
            ) {
                state.dave.move_left();
            }
        }

        if input.is_key_pressed(Keycode::D) || input.is_key_pressed(Keycode::Right) {
            let rect = state.dave.get_rect(Direction::Right);
            if !CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Right,
            ) {
                state.dave.move_right();
            }
        }

        if input.is_key_pressed(Keycode::W) || input.is_key_pressed(Keycode::Up) {
            let rect = state.dave.get_rect(Direction::Up);
            if !CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Up,
            ) {
                state.dave.move_up();
            }
        }

        if input.is_key_pressed(Keycode::S) || input.is_key_pressed(Keycode::Down) {
            let rect = state.dave.get_rect(Direction::Down);
            if !CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Down,
            ) {
                state.dave.move_down();
            }
        }

        state.dave.use_jetpack();
    }

    fn handle_actions(state: &mut GameState, input: &InputHandler) {
        if input.is_key_just_pressed(Keycode::LCtrl) || input.is_key_just_pressed(Keycode::RCtrl) {
            state.dave.shoot();
        }

        if input.is_key_just_pressed(Keycode::LAlt) || input.is_key_just_pressed(Keycode::RAlt) {
            state.dave.toggle_jetpack();
        }
    }
}
