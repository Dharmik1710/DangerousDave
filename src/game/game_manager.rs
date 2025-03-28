use super::game_utils::GameUtils;
use crate::game::state::GameState;
use crate::input::input_handler::InputHandler;
use crate::input::player_controller::PlayerController;
use crate::physics::physics::PhysicsEngine;
use crate::resources::camera::Camera;

pub struct GameManager;

impl GameManager {
    /// Updates the game logic on each frame
    pub fn update(state: &mut GameState, input_handler: &InputHandler) {
        //  handle movement & jumping
        PlayerController::handle_input(state, input_handler);

        // Apply physics
        PhysicsEngine::apply_gravity(state);

        // Update camera
        Camera::update(state);

        // update dave's animation
        state.dave.update_animation();

        // update dave bullet
        GameUtils::update_dave_bullet(state);

        // update enemy and shoot bullets(if can_shoot)
        GameUtils::update_enemy(state);

        // handle tile interactions
        // 1. collectibles
        // 2. Danger tiles
        // 3. door collisions
        GameUtils::handle_tile_interactions(state);

        // handle dave or its bullets collides with enemy or its bullet, dave dies
        GameUtils::handle_dave_enemy_collision(state);

        // if dead amd dead timer greater than 0
        GameUtils::handle_dave_dead(state);
    }
}
