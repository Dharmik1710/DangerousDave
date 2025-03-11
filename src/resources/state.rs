use super::{action_flags::ActionFlags, bullet::Bullet, camera::Camera, dave_state::DaveState, level::Level, monster_state::MonsterState};

#[derive(Debug, Clone)]
pub struct GameState {
    // World and level-related
    pub current_level: u8,  // Level identifier or index
    pub score: u32,          // Player's current score
    pub lives: u8,           // Lives left
    pub scroll_x: i32,       // Amount of horizontal scroll
    pub tick: u32,           // Global game timer (for animations, etc.)
    pub view_x: i32,         // World grid X position of the screen view
    pub view_y: i32,         // World grid Y position of the screen view

    // Input / triggers and collision
    pub check_door: bool,      // True if Dave is on the exit door
    pub check_pickup_x: u32,   // Grid X position for triggering a pickup
    pub check_pickup_y: u32,   // Grid Y position for triggering a pickup
    pub collision_points: [bool; 9], // Nine collision-check points (true if colliding)

    // Dave's state
    pub dave: DaveState,

    // Bullets: Using Option to indicate active/inactive state
    pub dave_bullet: Option<Bullet>,
    pub enemy_bullet: Option<Bullet>,

    // Monster state
    pub monster: MonsterState,

    // Input action flags
    pub try_actions: ActionFlags,

    // Game control flag
    pub quit: bool,

    // level data
    pub level: Level,

    // camera coords
    pub camera: Camera,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_level: 1,
            score: 0,
            lives: 3,
            scroll_x: 0,
            tick: 0,
            view_x: 0,
            view_y: 0,
            check_door: false,
            check_pickup_x: 0,
            check_pickup_y: 0,
            collision_points: [false; 9],
            dave: DaveState::default(),
            dave_bullet: None,
            enemy_bullet: None,
            monster: MonsterState::default(),
            try_actions: ActionFlags::default(),
            quit: false,
            level: Level::default(),
            camera: Camera::default()
        }
    }
}
