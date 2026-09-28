use bevy::prelude::Component;
use godot_bevy::prelude::GodotNode;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Sprite2D, class_name = PlayerNode)]
pub(super) struct Player;
