use bevy::prelude::*;
use godot_bevy::prelude::*;

mod game;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotTransformSyncPlugin::default())
        .add_plugins(BevyInputBridgePlugin)
        .add_plugins(game::GamePlugin);
}
