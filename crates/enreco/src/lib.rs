use godot::prelude::*;

// pub mod player;

struct ENrecoExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ENrecoExtension {
  fn on_level_init(_: InitLevel) {
    godot_print!("HUZZAH!");
  }
}
