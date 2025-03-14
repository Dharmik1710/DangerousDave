use crate::game::dave::Dave;
use crate::game::state::GameState;

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn apply_gravity(dave: &mut Dave) {
        if !dave.on_ground {
            dave.y += 2; // Simulate falling
        }
    }

    pub fn jump(dave: &mut Dave) {
        if dave.on_ground {
            dave.y -= 10;
            dave.on_ground = false;
        }
    }

    pub fn check_collision(dave: &mut Dave, state: &GameState) {
        if dave.y >= 500 {
            dave.on_ground = true;
        }
    }
}
