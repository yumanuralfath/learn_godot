mod components;
mod resources;
mod systems;

use bevy::{
    app::FixedUpdate,
    prelude::{App, Plugin},
};
use resources::MouseMoveTarget;
use systems::movement::player_movement;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseMoveTarget>()
            .add_systems(FixedUpdate, player_movement);
    }
}
