use godot::classes::Sprite2D;
use godot::global::clampf;
use godot::prelude::*;

use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::Input;
use godot::global::lerp;
// use godot::classes::CharacterBody2D;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    acceleration_factor: f64,
    deceleration_factor: f64,
    // min_speed:
    // max_speed: Vector2,

    base: Base<CharacterBody2D>
}

// #[godot_api]
// impl Player {
//     #[func]
//     pub fn init_children(&mut self) {
//         let mut sprite = Sprite2D::new_alloc();
//         sprite.text
//         self.base_mut().add_child(sprite);
//     }
// }


#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self  {
        godot_print!("Initializing Player.rs"); // Prints to the Godot console
        
        Self {
            speed: 500.0,
            angular_speed: std::f64::consts::PI * 0.01,
            acceleration_factor: 0.01,
            deceleration_factor: 0.98,
            // max_speed: Vector2::new(-1500.0, 1500.0),

            base,
        } 
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let speed = self.speed as f32;

        let steering_force = input.get_vector(
            "ui_left".into(),
            "ui_right".into(),
            "ui_up".into(),
            "ui_down".into()
        ) * speed * self.acceleration_factor as f32;

        let decel_factor = self.deceleration_factor as f32;

        let new_velocity = (self.base().get_velocity() + steering_force) * decel_factor;

        self.base_mut().set_velocity(new_velocity);

        let position = self.base().get_position();
        let rotation = self.base().get_rotation();

        let desired_rotation: f32 = match position.cross(position + new_velocity).is_sign_negative() {
            true => {
                position.angle_to_point(position + new_velocity)
            },
            false => {
                (position + new_velocity).angle_to_point(position)
            }
        };

        let actual_rotation = clampf(value, min, max);



        // self.base_mut().set_rotation(rotation + (actual_rotation * delta as f32));

        self.base_mut().move_and_slide();



    }
}

// #[godot_api]
// impl CharacterBody2D2D for Player {
//     fn init(base: Base<C>)
// }
