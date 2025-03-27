use super::gravity::Gravity;
use crate::game::state::GameState;

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn apply_gravity(state: &mut GameState) {
        // Apply gravity
        if !state.dave.is_jetpack_active {
            Gravity::apply_gravity(state);
        }
    }
}
