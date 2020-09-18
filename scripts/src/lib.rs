extern crate gdnative;

use gdnative::prelude::*;

mod controls;
mod player;

pub fn init(handle: InitHandle) {
    // Load player strust
    handle.add_class::<player::Player>(); 
}

godot_init!(init);