use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Default)]
pub(super) struct MouseMoveTarget(pub(super) Option<Vec2>);
