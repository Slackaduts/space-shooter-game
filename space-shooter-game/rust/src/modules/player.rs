// use godot::classes::RigidBody2D;
use godot::global::lerp_angle;
use godot::prelude::*;

use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::Input;

// use crate::modules::bullet::Bullet;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    acceleration_factor: f64,
    deceleration_factor: f64,

    base: Base<CharacterBody2D>
}

// #[godot_api]
// impl Player {
//     fn shoot(&mut self) {
//         let bullet = Bullet::new_alloc();
//         let mut binding = bullet.clone();
//         let mut bullet_body = binding.bind_mut();

//         // Set bullet position and rotation
//         bullet_body.base_mut().set_global_position(self.base().get_global_position());
//         bullet_body.base_mut().set_global_rotation(self.base().get_global_rotation());

//         // Calculate bullet velocity based on player's rotation
//         // let bullet_speed = 1000.0; // Adjust as needed
//         let velocity = Vector2::new(
//             bullet_body.bullet_speed as f32 * self.base().get_global_rotation().cos(),
//             bullet_body.bullet_speed as f32 * self.base().get_global_rotation().sin()
//         );

//         // Set up bullet properties
//         bullet_body.base_mut().set_linear_velocity(velocity);
//         // Call the shoot method on the Bullet instance
//         bullet_body.shoot(velocity);

//         // Add bullet to the scene
//         if let Some(mut parent) = self.base().get_parent() {
//             // let bullet_rbody = bullet.clone().upcast::<RigidBody2D>();
//             parent.add_child(bullet.upcast::<Node>());
//         }
        
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

        let velocity = self.base().get_velocity();
        let new_velocity = (velocity + steering_force) * self.deceleration_factor as f32;

        // Independent rotation, testing with mouse pos lookat
        // TODO: Point at the crosshair node, when we make it
        // TODO: Impl a crosshair node
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
}