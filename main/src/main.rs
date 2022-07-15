use bevy::prelude::*;
use patclib::PatcPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PatcPlugins)
        .run();
}
