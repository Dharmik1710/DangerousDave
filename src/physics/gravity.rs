use crate::{
    config::{self, DAVE_JUMP_UP_COOLDOWN, DAVE_SPEED},
    game::{
        dave::{self, Dave},
        state::{self, GameState},
    },
    resources::direction::Direction,
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
            let displacement =
                CollisionDetector::check_solid_tile_collision(state, Direction::Down);
            // if collision occurs then set ground to true
            if displacement == 0 {
                if !state.dave.on_ground {
                    state.dave.set_ground(true);
                }
            } else {
                state.dave.set_ground(false);
                state.dave.move_down(DAVE_SPEED);
            }
        } else if !state.dave.on_ground {
            // move up if jump is not 0 and ground is set to false
            let displacement = CollisionDetector::check_solid_tile_collision(state, Direction::Up);
            // if collision occurs, then set jump to 0
            if displacement == 0 {
                state.dave.set_jump(0);
            } else {
                state.dave.move_up(DAVE_SPEED);
            }
        }
    }
}
