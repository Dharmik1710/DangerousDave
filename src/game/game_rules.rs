use crate::config::{self, COLLECTIBLES, CUP_TILES, DANGER_TILES, DOOR_TILE, GAME_TILE_SIZE};
use crate::game::state::GameState;
use crate::physics::collisions::CollisionDetector;
use crate::resources::direction::Direction;

use super::camera::Camera;
use super::dave::Dave;
use super::enemy::{self, Enemy};

pub struct GameRules;

impl GameRules {
    /// ✅ Applies game rules (exit door, hazards, win/lose conditions)
    pub fn apply_rules(state: &mut GameState) {
        // handle tile interactions
        // 1. collectibles
        // 2. Danger tiles
        // 3. door collisions
        Self::handle_tile_interactions(state);

        // if dave collides with enemy or its bullet, dave dies
        Self::handle_dave_enemy(&mut state.dave, state.camera, &mut state.enemies);

        // TODO: check if bullet collides with solid tiles
        // Self::handle_bullet_solid_collision(state.enemies);

        // // Check if level completed (Dave reaches exit)
        // if state.level.is_exit(state.dave.px, state.dave.py) {
        //     state.advance_level();
        // }

        // // Check if game over
        // if state.dave.lives == 0 {
        //     state.quit = true; // End game
        // }

        // if dead amd dead timer greater than 0
        Self::handle_dave_dead(state);
    }

    pub fn handle_tile_interactions(state: &mut GameState) {
        let dave_rect = state.dave.get_rect(Direction::Chill);
        let corners = CollisionDetector::get_corners(&dave_rect, Direction::Chill);

        for corner in corners {
            let tile_x = (corner.x as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile_y = (corner.y as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile = state.level.get_tile(state.camera.x, tile_x, tile_y);

            Self::handle_collectibles(state, tile_x, tile_y, tile);
            Self::handle_danger_tile(state, tile);
            Self::handle_door_collision(state, tile);
        }
    }

    pub fn handle_collectibles(state: &mut GameState, tile_x: u32, tile_y: u32, tile: u8) {
        // todo!("Remove this is_collectible check as it is not required, handled by next if check");
        // if Self::is_collectible(&tile) {
        // ✅ Then check if we can get collectible points
        if let Some(points) = COLLECTIBLES.get(&tile) {
            state.collect(*points);

            // ✅ Remove collectible from level (set tile to 0)
            state
                .level
                .update_tile(state.camera.x, tile_x, tile_y, tile);
        }

        // update has_cup if collectible is cup
        if CUP_TILES.contains(&tile) {
            state.has_cup = true;
        }

        // }
    }

    pub fn handle_danger_tile(state: &mut GameState, tile: u8) {
        if DANGER_TILES.contains(&tile) {
            state.dave.dead();
        }
    }

    pub fn handle_door_collision(state: &mut GameState, tile: u8) {
        state.on_door = tile == DOOR_TILE;
        // this event will trigger completion of level
        if state.on_door {
            // Check if level completed
            Self::handle_level_completion(state);
        }
    }

    pub fn handle_dave_enemy(dave: &mut Dave, camera: Camera, enemies: &mut [Enemy]) {
        let dave_rect = dave.get_rect(Direction::Chill);
        for enemy in enemies.iter_mut() {
            if CollisionDetector::check_collision_with_enemy(camera, enemy, dave_rect) {
                enemy.dead();
                dave.dead();
            }

            if CollisionDetector::check_collision_with_bullet(enemy, dave_rect) {
                dave.dead();
                enemy.bullet.is_active = false;
            }
        }
    }

    pub fn handle_level_completion(state: &mut GameState) {
        if state.is_level_complete() {
            // level has been successfully completed
            state.current_level += 1;
            state.init_level();

            // TODO play the transition video
        }
    }

    pub fn handle_dave_dead(state: &mut GameState) {
        if !state.dave.is_alive {
            if state.dave.dead_timer > 0 {
                state.dave.decr_dead_timer();
                return;
            }
            // restart level with the current state
            state.respawn_dave();
        }
    }

    // TODO
    // pub fn handle_bullet_solid_collision(enemies: &mut [Enemy]){
    //     for enemy in enemies.iter_mut(){
    //         if
    //     }

    // }
}
