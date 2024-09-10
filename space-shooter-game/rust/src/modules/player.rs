use godot::global::lerp_angle;
// use godot::classes::Sprite2D;
// use godot::global::clampf;
use godot::prelude::*;

use godot::classes::CharacterBody2D;
// use godot::classes::CollisionShape2D;
use godot::classes::ICharacterBody2D;
use godot::classes::Input;
// use godot::global::view;
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
            angular_speed: std::f64::consts::TAU,
            acceleration_factor: 0.05,
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

        let velocity = self.base().get_velocity();
        let new_velocity = (velocity + steering_force) * decel_factor;

        // Independent rotation, testing with mouse pos lookat
        if let Some(viewport) = self.base().get_viewport() {
            let rotation = self.base().get_global_rotation();
            let mouse_pos = viewport.get_mouse_position();

            let desired_rot = self.base().get_position().angle_to_point(mouse_pos);
            let actual_rot = lerp_angle(
            rotation.into(), 
            desired_rot.into(),
            self.angular_speed * delta
            );
            self.base_mut().set_global_rotation(actual_rot as f32)
        }

        self.base_mut().set_velocity(new_velocity);
        self.base_mut().move_and_slide();        
    }

    // fn ready(&mut self) {
    //     let offset = self.base().get_node_as(CharacterShape2D)
    // }
}

// #[godot_api]
// impl CharacterBody2D2D for Player {
//     fn init(base: Base<C>)
// }
