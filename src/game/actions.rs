#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ActionFlags {
    /// Flags set when the player presses a key for an action.
    pub try_jump: bool,
    pub try_shoot: bool,
    pub move_left: bool,
    pub move_right: bool,
}

