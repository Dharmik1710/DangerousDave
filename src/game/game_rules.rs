use crate::config::{
    COLLECTIBLES, CUP_TILES, DANGER_TILES, DOOR_TILE, GAME_TILE_SIZE, GUN_TILE, JETPACK_FUEL,
    JETPACK_TILE,
};
use crate::game::state::GameState;
use crate::physics::collisions::CollisionDetector;
use crate::resources::direction::Direction;
use sdl2::rect::Rect;

pub struct GameRules;

impl GameRules {
    /// ✅ Applies game rules (exit door, hazards, win/lose conditions)
    pub fn apply_rules(state: &mut GameState) {
        // update dave bullet
        Self::update_dave_bullet(state);

        // update enemy and shoot bullets(if can_shoot)
        Self::update_enemy(state);

        // handle tile interactions
        // 1. collectibles
        // 2. Danger tiles
        // 3. door collisions
        Self::handle_tile_interactions(state);

        // handle dave or its bullets collides with enemy or its bullet, dave dies
        Self::handle_dave_enemy_collision(state);
        // TODO: check if bullet collides with solid tiles
        // Self::handle_bullet_solid_collision(state.enemies);

        // if dead amd dead timer greater than 0
        Self::handle_dave_dead(state);

        // // check if level is complete
        // Self::handle_level_completion(state);
    }

    pub fn handle_tile_interactions(state: &mut GameState) {
        let dave_rect = state.dave.get_rect(Direction::Chill);
        let corners = CollisionDetector::get_corners(dave_rect, Direction::Chill);

        for corner in corners {
            let tile_x = (corner.x as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile_y = (corner.y as f32 / GAME_TILE_SIZE as f32).floor() as u32;
            let tile_rect = Self::get_game_tile_rect(tile_x, tile_y);

            let is_collision = CollisionDetector::check_collision(dave_rect, tile_rect);

            let tile = state.level.get_tile(state.camera.x, tile_x, tile_y);

            Self::handle_collectibles(state, tile_x, tile_y, tile, is_collision);
            Self::handle_danger_tile(state, tile, is_collision);
            Self::handle_gun_pickup(state, tile, is_collision);
            Self::handle_jetpack_pickup(state, tile, is_collision);
            Self::handle_door_collision(state, tile, is_collision);
        }
    }

    pub fn handle_collectibles(
        state: &mut GameState,
        tile_x: u32,
        tile_y: u32,
        tile: u8,
        is_collision: bool,
    ) {
        // todo!("Remove this is_collectible check as it is not required, handled by next if check");
        // ✅ Then check if we can get collectible points
        if is_collision {
            if let Some(points) = COLLECTIBLES.get(&tile) {
                state.collect(*points);

                // ✅ Remove collectible from level (set tile to 0)
                state
                    .level
                    .update_tile(state.camera.x, tile_x, tile_y, tile);
            }

            // update has trophy if collectible is cup
            if CUP_TILES.contains(&tile) {
                state.dave.has_trophy = true;
            }
        }
    }

    pub fn handle_gun_pickup(state: &mut GameState, tile: u8, is_collision: bool) {
        if is_collision && tile == GUN_TILE {
            state.dave.has_gun = true;
        }
    }

    pub fn handle_jetpack_pickup(state: &mut GameState, tile: u8, is_collision: bool) {
        if is_collision && tile == JETPACK_TILE {
            state.dave.is_jetpack_active = true;
            state.dave.jetpack = std::cmp::max(state.dave.jetpack + JETPACK_FUEL, JETPACK_FUEL);
        }
    }

    pub fn handle_danger_tile(state: &mut GameState, tile: u8, is_collision: bool) {
        if DANGER_TILES.contains(&tile) && is_collision {
            state.dave.dead();
        }
    }

    pub fn get_game_tile_rect(tile_x: u32, tile_y: u32) -> Rect {
        Rect::new(
            (tile_x * GAME_TILE_SIZE) as i32,
            (tile_y * GAME_TILE_SIZE) as i32,
            GAME_TILE_SIZE,
            GAME_TILE_SIZE,
        )
    }

    pub fn handle_door_collision(state: &mut GameState, tile: u8, is_collision: bool) {
        if is_collision {
            state.dave.on_door = tile == DOOR_TILE;
            Self::handle_level_completion(state);
        }
    }

    pub fn handle_dave_enemy_collision(state: &mut GameState) {
        let dave_rect = state.dave.get_rect(Direction::Chill);
        let dave_bullet_rect = state.dave.bullet.get_rect(state.dave.direction);

        for enemy in state.enemies.iter_mut() {
            let enemy_rect = enemy.get_rect(state.camera);
            let enemy_bullet_rect = enemy.bullet.get_rect(Direction::Chill);
            // dave bullet - enemy bullet collision
            if state.dave.bullet.is_active
                && enemy.bullet.is_active
                && CollisionDetector::check_collision(dave_bullet_rect, enemy_bullet_rect)
            {
                state.dave.bullet.is_active = false;
                enemy.bullet.is_active = false;
            }

            // dave - enemy bullet collision
            if state.dave.is_alive
                && enemy.bullet.is_active
                && CollisionDetector::check_collision(dave_rect, enemy_bullet_rect)
            {
                state.dave.dead();
                enemy.bullet.is_active = false;
            }

            // dave - enemy collision
            if state.dave.is_alive
                && enemy.is_enemy_on_screen(state.camera)
                && CollisionDetector::check_collision(enemy_rect, dave_rect)
            {
                enemy.dead();
                state.dave.dead();
            }

            // dave bullet - enemy collsion
            if state.dave.bullet.is_active
                && enemy.is_enemy_on_screen(state.camera)
                && CollisionDetector::check_collision(dave_bullet_rect, enemy_rect)
            {
                enemy.dead();
                state.dave.bullet.is_active = false;
            }
        }
    }

    pub fn handle_level_completion(state: &mut GameState) {
        if state.dave.is_level_complete() {
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

    pub fn update_dave_bullet(state: &mut GameState) {
        state.dave.update_bullet(&state.level, state.camera);
    }

    pub fn update_enemy(state: &mut GameState) {
        state
            .enemies
            .iter_mut()
            .for_each(|enemy| enemy.update_enemy(&state.level, &state.dave, state.camera));
    }
}
