use godot::prelude::*;

mod player;
mod input;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
