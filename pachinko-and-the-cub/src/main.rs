use bevy::prelude::*;
use pachinko_and_the_cub::PatcPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PatcPlugins)
        .run();
}
