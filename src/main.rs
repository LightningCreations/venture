use bevy::prelude::*;

mod global;
mod scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(global::StatePlugin)
        .run();
}
