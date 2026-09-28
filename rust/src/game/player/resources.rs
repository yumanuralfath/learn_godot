use bevy::prelude::{Resource, Vec2};

#[derive(Resource)]
pub(super) struct PlayerMovementSettings {
    pub(super) speed: f32,
}

impl Default for PlayerMovementSettings {
    fn default() -> Self {
        Self { speed: 300.0 }
    }
}

#[derive(Resource, Default)]
pub(super) struct MouseMoveTarget(pub(super) Option<Vec2>);
