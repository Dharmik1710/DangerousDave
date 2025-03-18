use super::state::GameState;
use crate::{
    config,
    physics::collisions::CollisionDetector,
    resources::direction::{self, Direction},
};
use sdl2::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub enum DaveState {
    Chilling,
    Running,
    Jumping,
    Jetpack,
}

#[derive(Debug, Clone)]
pub struct Dave {
    pub px: i32,
    pub py: i32,
    pub jump: i32,
    pub direction: Direction,
    pub on_ground: bool,
    pub jetpack: bool,
    pub dave_state: DaveState,
}

impl Default for Dave {
    fn default() -> Self {
        Self {
            px: 0,
            py: 0,
            jump: 0,
            direction: Direction::Chill,
            jetpack: false,
            on_ground: true,
            dave_state: DaveState::Chilling,
        }
    }
}

impl Dave {
    pub fn move_left(state: &mut GameState) {
        let displacement = CollisionDetector::check_collision(state, Direction::Left);
        state.dave.px -= displacement;
    }

    pub fn move_right(state: &mut GameState) {
        let displacement = CollisionDetector::check_collision(state, Direction::Right);
        state.dave.px += displacement;
    }

    pub fn move_up(state: &mut GameState) {
        let displacement = CollisionDetector::check_collision(state, Direction::Up);
        state.dave.py -= displacement;
    }

    pub fn move_down(state: &mut GameState) {
        let displacement = CollisionDetector::check_collision(state, Direction::Down);
        state.dave.py += displacement;
    }

    pub fn init_dave_position(&mut self, pos: (u16, u16)) {
        self.px = (pos.0 * config::GAME_TILE_SIZE) as i32;
        self.py = (pos.1 * config::GAME_TILE_SIZE) as i32;
    }
}
