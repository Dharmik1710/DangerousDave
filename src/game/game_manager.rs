use crate::game::camera::Camera;
use crate::game::state::GameState;
use crate::input::input_handler::{self, InputHandler};
use crate::physics::physics::PhysicsEngine;

pub struct GameManager;

impl GameManager {
    /// ✅ Updates the game logic on each frame
    pub fn update(state: &mut GameState, input_handler: &InputHandler) {
        // ✅ Apply physics (gravity, movement, jumping)
        PhysicsEngine::apply_physics(state, input_handler);

        // Handle enemy logic
        state
            .enemies
            .iter_mut()
            .for_each(|enemy| enemy.update_enemy(&state.level.path, &state.dave, &state.camera));

        // Apply game rules (e.g., unlock exit door, check for game over)
        // GameRules::apply_rules(state);

        // Update camera
        Camera::update(state);
    }
}
