use bevy::prelude::{
    ButtonInput, KeyCode, MessageReader, Query, Res, ResMut, Time, Transform, Vec2, With,
};
use godot::builtin::Vector2 as GodotVec2;
use godot::classes::CanvasItem;
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};
use godot_bevy::plugins::input::{GodotMouseButton, GodotMouseButtonInput};

use super::components::Player;
use super::resources::{MouseMoveTarget, PlayerMovementSettings};

pub(super) fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_clicks: MessageReader<GodotMouseButtonInput>,
    settings: Res<PlayerMovementSettings>,
    mut mouse_target: ResMut<MouseMoveTarget>,
    mut godot: GodotAccess,
    player_nodes: Query<&GodotNodeHandle, With<Player>>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let click_position = mouse_clicks
        .read()
        .filter(|click| click.button == GodotMouseButton::Left && click.pressed)
        .map(|click| click.position)
        .last();

    if let (Some(click_position), Some(handle)) = (click_position, player_nodes.iter().next()) {
        if let Some(canvas_item) = godot.try_get::<CanvasItem>(*handle) {
            let mouse_to_world = canvas_item.get_canvas_transform().affine_inverse();
            let world_position =
                mouse_to_world * GodotVec2::new(click_position.x, click_position.y);
            mouse_target.0 = Some(Vec2::new(world_position.x, world_position.y));
        }
    }

    let direction = keyboard_direction(&keys);

    for mut transform in &mut players {
        if direction != Vec2::ZERO {
            transform.translation += (direction * settings.speed * time.delta_secs()).extend(0.0);
            mouse_target.0 = None;
            continue;
        }

        let Some(target) = mouse_target.0 else {
            continue;
        };

        let current_position = transform.translation.truncate();
        let offset = target - current_position;
        let max_step = settings.speed * time.delta_secs();

        if offset.length_squared() <= max_step * max_step {
            transform.translation.x = target.x;
            transform.translation.y = target.y;
        } else {
            let movement = offset.normalize() * max_step;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

fn keyboard_direction(keys: &ButtonInput<KeyCode>) -> Vec2 {
    let mut direction = Vec2::ZERO;

    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        direction.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        direction.x += 1.0;
    }
    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        direction.y -= 1.0;
    }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        direction.y += 1.0;
    }

    direction.normalize_or_zero()
}
