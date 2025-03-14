use crate::game::state::GameState;
use crate::game::player::Player;

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn apply_gravity(player: &mut Player) {
        if !player.on_ground {
            player.y += 2; // Simulate falling
        }
    }

    pub fn jump(player: &mut Player) {
        if player.on_ground {
            player.y -= 10;
            player.on_ground = false;
        }
    }

    pub fn check_collision(player: &mut Player, state: &GameState) {
        if player.y >= 500 {
            player.on_ground = true;
        }
    }
}
