use crate::game::camera::Camera;
use crate::game::state::GameState;
use crate::input::input_handler::{self, InputHandler};
use crate::input::player_controller::PlayerController;
use crate::physics::physics::PhysicsEngine;

use super::game_rules::GameRules;

pub struct GameManager;

impl GameManager {
    /// ✅ Updates the game logic on each frame
    pub fn update(state: &mut GameState, input_handler: &InputHandler) {
        //  handle movement & jumping
        PlayerController::handle_input(state, input_handler);

        // ✅ Apply physics
        PhysicsEngine::apply_gravity(state);

        // Update camera
        Camera::update(state);

        // apply game rules
        GameRules::apply_rules(state);
    }
}
