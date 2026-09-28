use bevy::prelude::*;
use godot_bevy::prelude::*;

mod game;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins)
        .insert_resource(DebuggerConfig {
            enabled: false,
            ..Default::default()
        })
        .add_plugins(game::GamePlugin);
}
