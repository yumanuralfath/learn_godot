mod components;
mod resources;
mod systems;

use bevy::prelude::{App, Plugin, Update};
use resources::{MouseMoveTarget, PlayerMovementSettings};
use systems::player_movement;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMovementSettings>()
            .init_resource::<MouseMoveTarget>()
            .add_systems(Update, player_movement);
    }
}
