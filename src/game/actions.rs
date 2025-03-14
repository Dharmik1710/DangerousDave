#[derive(Debug, Clone)]
pub struct ActionFlags {
    /// Flags set when the player presses a key for an action.
    pub try_jump: bool,
    pub try_shoot: bool,
    pub move_left: bool,
    pub move_right: bool,
}

impl Default for ActionFlags {
    fn default() -> Self {
        Self {
            try_jump: false,
            try_shoot: false,
            move_left: false,
            move_right: false,
        }
    }
}
