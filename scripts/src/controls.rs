use gdnative::api::GlobalConstants;

pub struct KeyboardControls {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub key_forward: i64,
    pub key_backward: i64,
    pub key_left: i64,
    pub key_right: i64,
    pub key_jump: i64
}

impl KeyboardControls {
    pub fn new() -> Self {
        KeyboardControls {
            forward: false,
            backward: false,
            left: false,
            right: false,
            jump: false,
            key_forward: GlobalConstants::KEY_Z,
            key_backward: GlobalConstants::KEY_S,
            key_left: GlobalConstants::KEY_Q,
            key_right: GlobalConstants::KEY_D,
            key_jump: GlobalConstants::KEY_SPACE,
        }
    }

}