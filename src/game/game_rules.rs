use crate::config;
use crate::game::state::GameState;
use crate::physics::collisions::CollisionDetector;

pub struct GameRules;

impl GameRules {
    /// ✅ Applies game rules (exit door, hazards, win/lose conditions)
    pub fn apply_rules(state: &mut GameState) {

        // // Open exit if cup is collected
        // if state.dave.has_cup {
        //     state.level.open_exit();
        // }

        // // Check if level completed (Dave reaches exit)
        // if state.level.is_exit(state.dave.px, state.dave.py) {
        //     state.advance_level();
        // }

        // // Check if game over
        // if state.dave.lives == 0 {
        //     state.quit = true; // End game
        // }
    }
}
