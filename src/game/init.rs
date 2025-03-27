pub struct Initialize;

impl Initialize {
    pub fn get_dave_init_pos(level_num: u8) -> (u32, u32) {
        let dave_init_pos = match level_num {
            1 => (2, 8),
            2 => (1, 8),
            3 => (2, 5),
            4 => (1, 5),
            5 => (2, 8),
            6 => (2, 8),
            7 => (1, 2),
            8 => (2, 8),
            9 => (6, 1),
            10 => (2, 8),
            _ => (0, 0), // Default fallback
        };
        return dave_init_pos;
    }

    pub fn get_monsters_for_level(level: u32) -> (u32, Vec<(u32, u32)>) {
        match level {
            2 => (89, vec![(44, 4), (59, 4)]),
            3 => (93, vec![(32, 2)]),
            4 => (97, vec![(15, 3), (33, 3), (49, 3)]),
            5 => (101, vec![(10, 8), (28, 8), (45, 2), (40, 8)]),
            6 => (105, vec![(5, 2), (16, 1), (46, 2), (56, 3)]),
            7 => (109, vec![(53, 5), (72, 2), (84, 1)]),
            8 => (113, vec![(35, 8), (41, 8), (49, 8), (65, 8)]),
            9 => (117, vec![(45, 8), (51, 2), (65, 3), (82, 5)]),
            _ => (0, vec![]), // Default case: No monsters
        }
    }
}
