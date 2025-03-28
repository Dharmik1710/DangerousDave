use super::animation::{Animation, AnimationState};
use std::collections::HashMap;

pub struct AnimationRegistry;

impl AnimationRegistry {
    pub fn initialize_dave_animations() -> HashMap<AnimationState, Animation> {
        [
            (AnimationState::Idle, Animation::new(vec![56], 10)),
            (AnimationState::IdleRight, Animation::new(vec![54], 5)),
            (AnimationState::IdleLeft, Animation::new(vec![58], 5)),
            (
                AnimationState::RunningRight,
                Animation::new(vec![53, 54, 55], 3),
            ),
            (
                AnimationState::RunningLeft,
                Animation::new(vec![57, 58, 58], 3),
            ),
            (AnimationState::JumpingRight, Animation::new(vec![67], 10)),
            (AnimationState::JumpingLeft, Animation::new(vec![68], 10)),
            (
                AnimationState::JetpackingRight,
                Animation::new(vec![77, 78, 79], 6),
            ),
            (
                AnimationState::JetpackingLeft,
                Animation::new(vec![80, 81, 82], 6),
            ),
            (
                AnimationState::Climbing,
                Animation::new(vec![71, 72, 73], 8),
            ),
            (
                AnimationState::Dying,
                Animation::new(vec![129, 130, 131, 132], 3),
            ),
        ]
        .into_iter()
        .collect() // Converts Vec<(Key, Value)> into HashMap<Key, Value>
    }
}
