use godot::prelude::*;

use godot::classes::Sprite2D;
use godot::classes::ISprite2D;
use godot::classes::Input;
// use godot::classes::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>
}


#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self  {
        godot_print!("Initializing Player.rs"); // Prints to the Godot console
        
        Self {
            speed: 200.0,
            angular_speed: std::f64::consts::PI,
            base,
        } 
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        let input_vec = input.get_vector(
            "ui_left".into(), 
            "ui_right".into(),
            "ui_up".into(),
            "ui_down".into()
        );

        // Move the player in accordance to the input, self.speed, and delta.
        let velocity = input_vec * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);



        // if input.is_action_pressed("ui_left".into()) {
        //     let radians = (self.angular_speed * delta) as f32;
            
    
        //     let rotation = self.base().get_rotation();
        //     let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        //     self.base_mut().translate(velocity * delta as f32);
        // }



        // godot_print!("inputs: {}", vector);
        // godot_print!("coords: {}", self.base_mut().get_real_velocity());
    }
}

// #[godot_api]
// impl Sprite2D for Player {
//     fn init(base: Base<C>)
// }
