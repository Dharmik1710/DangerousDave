#[derive(Debug, Clone)]
pub struct ActionFlags {
    /// Flags set when the player pushes a key for a specific action.
    pub try_jump: bool,
    pub try_shoot: bool,
    // Add other action flags as needed (e.g., try_climb, try_use, etc.)
}

impl Default for ActionFlags {
    fn default() -> Self {
        Self {
            try_jump: false,
            try_shoot: false,
        }
    }
}
