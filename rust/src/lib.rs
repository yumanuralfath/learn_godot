use godot::{
    classes::{ISprite2D, Input, Sprite2D},
    global::Key,
    prelude::*,
};

struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {}

#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
struct Player {
    #[init(val = 300.0)]
    speed: f32,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut direction = Vector2::ZERO;

        if input.is_physical_key_pressed(Key::B) || input.is_physical_key_pressed(Key::LEFT) {
            direction.x -= 1.0;
        }
        if input.is_physical_key_pressed(Key::D) || input.is_physical_key_pressed(Key::RIGHT) {
            direction.x += 1.0;
        }
        if input.is_physical_key_pressed(Key::W) || input.is_physical_key_pressed(Key::UP) {
            direction.y -= 1.0;
        }
        if input.is_physical_key_pressed(Key::S) || input.is_physical_key_pressed(Key::DOWN) {
            direction.y += 1.0;
        }

        if direction != Vector2::ZERO {
            let movement = direction.normalized() * self.speed * delta as f32;
            let next_position = self.base().get_position() + movement;
            self.base_mut().set_position(next_position);
        }
    }
}
