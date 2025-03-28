use crate::{
    config::DAVE_JUMP_UP_COOLDOWN, game::state::GameState, resources::direction::Direction,
};

use super::collisions::CollisionDetector;

pub struct Gravity;

impl Gravity {
    pub fn apply_gravity(state: &mut GameState) {
        if state.dave.jump == 0 {
            if state.dave.jump_cooldown > DAVE_JUMP_UP_COOLDOWN {
                state.dave.decr_cooldown();
                return;
            }
            // irrespective check if you can move down gravity will act upon it
            let rect = state.dave.get_rect(Direction::Down);
            if CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Down,
            ) {
                // if collision occurs then set ground to true
                if !state.dave.on_ground {
                    state.dave.set_ground(true);
                }
            } else {
                state.dave.set_ground(false);
                state.dave.move_down();
            }
        } else if !state.dave.on_ground {
            // move up if jump is not 0 and ground is set to false
            let rect = state.dave.get_rect(Direction::Up);

            if CollisionDetector::check_solid_tile_collision(
                &state.level,
                state.camera,
                rect,
                Direction::Up,
            ) {
                // if collision occurs, then set jump to 0
                state.dave.set_jump(0);
            } else {
                state.dave.move_up();
            }
        }
    }
}
