use crate::controls::KeyboardControls;
use gdnative::prelude::*;
use gdnative::api::*;

const MOVEMENT_SPEED: f32 = 5.0;
const MOVEMENT_KEY_FORWARD: i64 = GlobalConstants::KEY_Z;
const MOVEMENT_KEY_BACKWARD: i64 = GlobalConstants::KEY_S;
const MOVEMENT_KEY_LEFT: i64 = GlobalConstants::KEY_Q;
const MOVEMENT_KEY_RIGHT: i64 = GlobalConstants::KEY_D;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    controls: KeyboardControls,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            controls: KeyboardControls::new(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody) {
        Input::godot_singleton().set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
        godot_dbg!("ready");
    }

    #[export]
    fn _input(&mut self, _owner: &KinematicBody, _event: Ref<InputEvent>) {
        // if let Some(event) = event.clone().cast::<InputEventKey>() {
        //     let event = unsafe { event.assume_safe() };
        //     let key_code = event.scancode();
        // }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, _delta: f32) {
        let mut player_movement = Vector3::zero();
        let player_input = Input::godot_singleton();

        // When player press forward key
        if player_input.is_key_pressed(MOVEMENT_KEY_FORWARD) {
            self.controls.forward = true;
            player_movement.z -= MOVEMENT_SPEED;
        }

        // When player press backward key
        if player_input.is_key_pressed(MOVEMENT_KEY_BACKWARD) {
            self.controls.backward = true;
            player_movement.z += MOVEMENT_SPEED;
        }

        // When forward and backward keys are pressed
        if self.controls.backward == self.controls.forward {
            self.controls.backward = false;
            self.controls.forward = false;
        }

        // When player press left key
        if player_input.is_key_pressed(MOVEMENT_KEY_LEFT) {
            self.controls.left = true;
            player_movement.x -= MOVEMENT_SPEED;
        }

        // When player press right key
        if player_input.is_key_pressed(MOVEMENT_KEY_RIGHT) {
            self.controls.right = true;
            player_movement.x += MOVEMENT_SPEED;
        }

        // When left and right keys are pressed
        if self.controls.left == self.controls.right {
            self.controls.right = false;
            self.controls.left = false;
        }

        // Move player in game
        owner.move_and_collide(player_movement, false, false, false);
    }
}
