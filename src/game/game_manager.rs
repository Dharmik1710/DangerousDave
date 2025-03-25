use crate::game::camera::Camera;
use crate::game::state::GameState;
use crate::input::input_handler::{self, InputHandler};
use crate::physics::physics::PhysicsEngine;

use super::game_rules::GameRules;

pub struct GameManager;

impl GameManager {
    /// ✅ Updates the game logic on each frame
    pub fn update(state: &mut GameState, input_handler: &InputHandler) {
        // ✅ Apply physics (gravity, movement, jumping)
        PhysicsEngine::apply_physics(state, input_handler);

        // Update camera
        Camera::update(state);

        // Handle enemy logic
        state
            .enemies
            .iter_mut()
            .for_each(|enemy| enemy.update_enemy(&state.level.path, &state.dave, state.camera));

        // if dead amd dead timer greater than 0
        state.dave.decr_dead_timer();

        // Apply game rules (e.g., unlock exit door, check for game over)
        // state.if_finsihed_level();

        // check if dave died
        if !state.dave.is_alive && state.dave.dead_timer == 0 {
            if state.lives != 0 {
                state.respawn_dave();
            } else {
                // restart game
                state.init_level();
            }
        }
    }
}
