use crate::controls::KeyboardControls;
use gdnative::prelude::*;
use gdnative::api::*;

const MOVEMENT_SPEED: f32 = 1.0;
const MOVEMENT_JUMP_SCALE: f32 = 2.0;
const MOVEMENT_KEY_FORWARD: i64 = GlobalConstants::KEY_Z;
const MOVEMENT_KEY_BACKWARD: i64 = GlobalConstants::KEY_S;
const MOVEMENT_KEY_LEFT: i64 = GlobalConstants::KEY_Q;
const MOVEMENT_KEY_RIGHT: i64 = GlobalConstants::KEY_D;
const MOVEMENT_KEY_JUMP: i64 = GlobalConstants::KEY_SPACE;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    controls: KeyboardControls,
    up_velocity: f32,
    camera_rotation: Vector2,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            controls: KeyboardControls::new(),
            up_velocity: 0.0,
            camera_rotation: Vector2::zero(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody) {
        Input::godot_singleton().set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
        godot_dbg!("ready 7");
    }

    #[export]
    fn _input(&mut self, _owner: &KinematicBody, event: Ref<InputEvent>) {
        if let Some(event) = event.clone().cast::<InputEventMouseMotion>() {
            let event = unsafe { event.assume_safe() };

            let motion: Vector2 = event.relative();
            self.camera_rotation.x += motion.y * 0.0025f32;
            self.camera_rotation.y -= motion.x * 0.0025f32;

            if self.camera_rotation.x < -0.4 {
                self.camera_rotation.x = -0.4;
            }
            if self.camera_rotation.x > 0.4 {
                self.camera_rotation.x = 0.4;
            }
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f32) {
        let mut player_movement = Vector3::zero();
        let basis: Basis = owner.transform().basis;
        let mut x = basis.x();
        x *= MOVEMENT_SPEED;
        let mut z = basis.z();
        z *= MOVEMENT_SPEED;

        let player_input = Input::godot_singleton();

        // When player press forward key
        if player_input.is_key_pressed(MOVEMENT_KEY_FORWARD) {
            self.controls.forward = true;
            player_movement -= z;
        }

        // When player press backward key
        if player_input.is_key_pressed(MOVEMENT_KEY_BACKWARD) {
            self.controls.backward = true;
            player_movement += z;
        }

        // When forward and backward keys are pressed
        if self.controls.backward == self.controls.forward {
            self.controls.backward = false;
            self.controls.forward = false;
        }

        // When player press left key
        if player_input.is_key_pressed(MOVEMENT_KEY_LEFT) {
            self.controls.left = true;
            player_movement -= x;
        }

        // When player press right key
        if player_input.is_key_pressed(MOVEMENT_KEY_RIGHT) {
            self.controls.right = true;
            player_movement += x;
        }

        // When left and right keys are pressed
        if self.controls.left == self.controls.right {
            self.controls.right = false;
            self.controls.left = false;
        }

        // When player press right key
        if player_input.is_key_pressed(MOVEMENT_KEY_JUMP) && self.controls.jump == false {
            self.controls.jump = true;
            self.up_velocity = MOVEMENT_JUMP_SCALE;
        }

        // Move player in game
        owner.move_and_collide(player_movement, false, false, false);

        // Rotate player depending player mouse mouvement on Y axis
        owner.set_rotation(Vector3::new(0.0f32, self.camera_rotation.y, 0.0f32));

        unsafe {
            // Get Spacial node named "CameraRotation"
            if let Some(camera_rotation_node) = owner.get_node("CameraRotation") {
                // Cast node to Spacial
                if let Some(camera_rotation_spatial) = camera_rotation_node.assume_safe().cast::<Spatial>() {
                    // Rotate camera depending player mouse mouvement on X axis
                    camera_rotation_spatial.set_rotation(Vector3::new(-1.0f32 * self.camera_rotation.x, 0.0f32, 0.0f32));
                } else {
                    godot_error!("Can't cast node to Spacial");
                }
            } else {
                godot_error!("Spacial node 'CameraRotation' not found.");
            }
        }

        if self.up_velocity > MOVEMENT_JUMP_SCALE {
            self.up_velocity = MOVEMENT_JUMP_SCALE;
        }
        if self.up_velocity < -MOVEMENT_JUMP_SCALE {
            self.up_velocity = -MOVEMENT_JUMP_SCALE;
        }
        
        let gravity = Vector3::new(0.0f32, self.up_velocity, 0.0f32);
        if let Some(_collision) = owner.move_and_collide(gravity, false, false, false) {
            // When player is on floor
            self.up_velocity = 0.0;
            self.controls.jump = false;
        } else {
            self.up_velocity -= 0.1;
        }
    }
}
