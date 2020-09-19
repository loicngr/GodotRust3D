pub struct KeyboardControls {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

impl KeyboardControls {
    pub fn new() -> Self {
        KeyboardControls {
            forward: false,
            backward: false,
            left: false,
            right: false,
            jump: false,
        }
    }

}