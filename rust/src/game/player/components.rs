use bevy::prelude::Component;
use godot_bevy::prelude::GodotNode;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Sprite2D, class_name = PlayerNode)]
#[gdbevy(require(speed: PlayerSpeed, as = f32, default = 300.0))]
pub(super) struct Player;

#[derive(Component, Default)]
pub(super) struct PlayerSpeed(pub(super) f32);
