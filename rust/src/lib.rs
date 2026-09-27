use bevy::prelude::*;
use godot_bevy::prelude::*;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Sprite2D, class_name = PlayerNode)]
struct Player;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins)
        .insert_resource(DebuggerConfig {
            enabled: false,
            ..Default::default()
        })
        .add_systems(Update, player_movement);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        direction.y += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let movement = direction.normalize() * 300.0 * time.delta_secs();
    for mut transform in &mut players {
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}
