use crate::config;
use crate::game::state::GameState;

pub struct CollectibleManager;

impl CollectibleManager {
    // /// ✅ Checks for collectibles and updates state
    // pub fn handle_collectibles(state: &mut GameState) {
    //     let tile_size = config::GAME_TILE_SIZE as f32;
    //     let tile_x = (state.dave.px as f32 / tile_size).round() as i32;
    //     let tile_y = (state.dave.py as f32 / tile_size).round() as i32;
    //     let tile = state.level.get_tile(state.camera.x, tile_x, tile_y);
    //     // ✅ First check if tile is collectible
    //     if CollectibleManager::is_collectible(&tile) {
    //         // ✅ Then check if we can get collectible points
    //         if let Some(points) = config::COLLECTIBLES.get(&tile) {
    //             state.dave.collect(*points);

    //             // ✅ Remove collectible from level (set tile to 0)
    //             state.level.update_tile(state.camera.x, tile_x, tile_y, 0);
    //         }
    //     }
    // }
}
