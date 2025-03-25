use sdl2::rect::Rect;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::{
    config::{DAVE_DEFAULT_TILE, SCALE},
    resources::direction::{self, Direction},
};

pub static TILE_MAP: LazyLock<HashMap<u8, Rect>> = LazyLock::new(|| {
    [
        (144, Rect::new(0, 0, 112, 47)),
        (145, Rect::new(0, 47, 112, 47)),
        (147, Rect::new(0, 94, 112, 47)),
        (146, Rect::new(0, 141, 112, 47)),
        (104, Rect::new(112, 141, 24, 22)),
        (101, Rect::new(136, 141, 24, 22)),
        (103, Rect::new(0, 188, 24, 22)),
        (102, Rect::new(24, 188, 24, 22)),
        (98, Rect::new(48, 188, 24, 21)),
        (99, Rect::new(72, 188, 24, 21)),
        (100, Rect::new(96, 188, 24, 21)),
        (97, Rect::new(120, 188, 24, 21)),
        (89, Rect::new(144, 188, 28, 20)),
        (118, Rect::new(0, 210, 16, 20)),
        (119, Rect::new(16, 210, 16, 20)),
        (120, Rect::new(32, 210, 16, 20)),
        (117, Rect::new(48, 210, 16, 20)),
        (96, Rect::new(64, 210, 24, 20)),
        (94, Rect::new(88, 210, 24, 20)),
        (95, Rect::new(112, 210, 24, 20)),
        (91, Rect::new(136, 210, 28, 20)),
        (90, Rect::new(0, 230, 28, 20)),
        (92, Rect::new(28, 230, 28, 20)),
        (93, Rect::new(56, 230, 24, 20)),
        (61, Rect::new(80, 230, 20, 16)),
        (75, Rect::new(100, 230, 20, 16)),
        (49, Rect::new(120, 230, 16, 16)),
        (48, Rect::new(136, 230, 16, 16)),
        (74, Rect::new(152, 230, 20, 16)),
        (60, Rect::new(0, 250, 20, 16)),
        (76, Rect::new(20, 250, 20, 16)),
        (62, Rect::new(40, 250, 20, 16)),
        (88, Rect::new(60, 250, 20, 16)),
        (63, Rect::new(80, 250, 20, 16)),
        (77, Rect::new(100, 250, 20, 16)),
        (73, Rect::new(120, 250, 20, 16)),
        (67, Rect::new(140, 250, 20, 16)),
        (66, Rect::new(0, 266, 20, 16)),
        (72, Rect::new(20, 266, 20, 16)),
        (58, Rect::new(40, 266, 20, 16)),
        (64, Rect::new(60, 266, 20, 16)),
        (70, Rect::new(80, 266, 20, 16)),
        (71, Rect::new(100, 266, 20, 16)),
        (65, Rect::new(120, 266, 20, 16)),
        (59, Rect::new(140, 266, 20, 16)),
        (16, Rect::new(160, 266, 16, 16)),
        (6, Rect::new(0, 282, 16, 16)),
        (7, Rect::new(16, 282, 16, 16)),
        (17, Rect::new(32, 282, 16, 16)),
        (15, Rect::new(48, 282, 16, 16)),
        (29, Rect::new(64, 282, 16, 16)),
        (5, Rect::new(80, 282, 16, 16)),
        (4, Rect::new(96, 282, 16, 16)),
        (28, Rect::new(112, 282, 16, 16)),
        (14, Rect::new(128, 282, 16, 16)),
        (38, Rect::new(144, 282, 16, 16)),
        (10, Rect::new(160, 282, 16, 16)),
        (0, Rect::new(0, 298, 16, 16)),
        (1, Rect::new(16, 298, 16, 16)),
        (11, Rect::new(32, 298, 16, 16)),
        (39, Rect::new(48, 298, 16, 16)),
        (13, Rect::new(64, 298, 16, 16)),
        (3, Rect::new(80, 298, 16, 16)),
        (2, Rect::new(96, 298, 16, 16)),
        (12, Rect::new(112, 298, 16, 16)),
        (23, Rect::new(128, 298, 16, 16)),
        (37, Rect::new(144, 298, 16, 16)),
        (36, Rect::new(160, 298, 16, 16)),
        (22, Rect::new(0, 314, 16, 16)),
        (113, Rect::new(16, 314, 16, 16)),
        (34, Rect::new(32, 314, 16, 16)),
        (20, Rect::new(48, 314, 16, 16)),
        (21, Rect::new(64, 314, 16, 16)),
        (35, Rect::new(80, 314, 16, 16)),
        (114, Rect::new(96, 314, 16, 16)),
        (19, Rect::new(112, 314, 16, 16)),
        (31, Rect::new(128, 314, 16, 16)),
        (25, Rect::new(144, 314, 16, 16)),
        (9, Rect::new(160, 314, 16, 16)),
        (8, Rect::new(0, 330, 16, 16)),
        (24, Rect::new(16, 330, 16, 16)),
        (30, Rect::new(32, 330, 16, 16)),
        (18, Rect::new(48, 330, 16, 16)),
        (115, Rect::new(64, 330, 16, 16)),
        (26, Rect::new(80, 330, 16, 16)),
        (32, Rect::new(96, 330, 16, 16)),
        (33, Rect::new(112, 330, 16, 16)),
        (27, Rect::new(128, 330, 16, 16)),
        (116, Rect::new(144, 330, 16, 16)),
        (40, Rect::new(160, 330, 16, 16)),
        (54, Rect::new(0, 346, 9, 16)),
        (68, Rect::new(9, 346, 20, 16)),
        (83, Rect::new(29, 346, 20, 16)),
        (82, Rect::new(49, 346, 20, 16)),
        (69, Rect::new(69, 346, 20, 16)),
        (55, Rect::new(89, 346, 20, 16)),
        (41, Rect::new(109, 346, 16, 16)),
        (57, Rect::new(125, 346, 20, 16)),
        (43, Rect::new(145, 346, 16, 16)),
        (80, Rect::new(0, 362, 20, 16)),
        (81, Rect::new(20, 362, 20, 16)),
        (42, Rect::new(40, 362, 16, 16)),
        (56, Rect::new(56, 362, 20, 16)),
        (52, Rect::new(76, 362, 16, 16)),
        (46, Rect::new(92, 362, 16, 16)),
        (85, Rect::new(108, 362, 20, 16)),
        (84, Rect::new(128, 362, 20, 16)),
        (47, Rect::new(148, 362, 16, 16)),
        (53, Rect::new(0, 378, 20, 16)),
        (79, Rect::new(20, 378, 20, 16)),
        (45, Rect::new(40, 378, 16, 16)),
        (51, Rect::new(56, 378, 16, 16)),
        (86, Rect::new(72, 378, 20, 16)),
        (87, Rect::new(92, 378, 20, 16)),
        (50, Rect::new(112, 378, 16, 16)),
        (44, Rect::new(128, 378, 16, 16)),
        (78, Rect::new(144, 378, 20, 16)),
        (140, Rect::new(0, 394, 68, 15)),
        (139, Rect::new(68, 394, 72, 15)),
        (138, Rect::new(0, 409, 176, 14)),
        (132, Rect::new(0, 423, 20, 13)),
        (130, Rect::new(20, 423, 20, 13)),
        (131, Rect::new(40, 423, 20, 13)),
        (129, Rect::new(60, 423, 20, 13)),
        (143, Rect::new(80, 423, 16, 12)),
        (141, Rect::new(0, 436, 130, 12)),
        (109, Rect::new(130, 436, 16, 12)),
        (112, Rect::new(146, 436, 16, 12)),
        (111, Rect::new(0, 448, 16, 12)),
        (110, Rect::new(16, 448, 16, 12)),
        (150, Rect::new(32, 448, 8, 11)),
        (151, Rect::new(40, 448, 8, 11)),
        (153, Rect::new(48, 448, 8, 11)),
        (152, Rect::new(56, 448, 8, 11)),
        (156, Rect::new(64, 448, 8, 11)),
        (157, Rect::new(72, 448, 8, 11)),
        (155, Rect::new(80, 448, 8, 11)),
        (154, Rect::new(88, 448, 8, 11)),
        (133, Rect::new(96, 448, 62, 11)),
        (135, Rect::new(0, 460, 62, 11)),
        (134, Rect::new(62, 460, 62, 11)),
        (136, Rect::new(0, 471, 62, 11)),
        (137, Rect::new(62, 471, 62, 11)),
        (148, Rect::new(124, 471, 8, 11)),
        (149, Rect::new(132, 471, 8, 11)),
        (108, Rect::new(140, 471, 18, 8)),
        (106, Rect::new(158, 471, 18, 8)),
        (107, Rect::new(0, 482, 18, 8)),
        (105, Rect::new(18, 482, 18, 8)),
        (142, Rect::new(36, 482, 6, 4)),
        (127, Rect::new(42, 482, 12, 3)),
        (126, Rect::new(54, 482, 20, 3)),
        (124, Rect::new(74, 482, 20, 3)),
        (125, Rect::new(94, 482, 20, 3)),
        (121, Rect::new(114, 482, 20, 3)),
        (122, Rect::new(134, 482, 20, 3)),
        (123, Rect::new(154, 482, 20, 3)),
        (128, Rect::new(0, 490, 12, 3)),
    ]
    .into_iter()
    .collect()
});

/// Holds tile mappings from tile ID to Rect on the texture
pub struct TileAtlas;

impl TileAtlas {
    /// ✅ Retrieves a `Rect` for a given tile ID
    pub fn get_rect(tile_id: u8) -> Rect {
        *TILE_MAP.get(&tile_id).unwrap()
    }

    /// ✅ Checks if a tile exists in the map
    pub fn has_tile(tile_id: u8) -> bool {
        TILE_MAP.contains_key(&tile_id)
    }

    /// ✅ get dave tile
    pub fn get_dave() -> Rect {
        let dave_tile = DAVE_DEFAULT_TILE;
        *TILE_MAP.get(&dave_tile).unwrap()
    }

    /// get enemy
    pub fn get_enemy(tile_num: u8) -> Rect {
        *TILE_MAP.get(&tile_num).unwrap()
    }

    /// return width and height
    pub fn get_dimension(tile_id: u8) -> (u32, u32) {
        let rect = *TILE_MAP.get(&tile_id).unwrap();
        (rect.width() * SCALE, rect.height() * SCALE)
    }

    pub fn get_bullet(direction: Direction) -> Rect {
        let tile_num = 126;
        *TILE_MAP.get(&tile_num).unwrap()
    }
}
