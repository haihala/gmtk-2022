use bevy::prelude::*;

mod components;
mod helper;
mod utils;

pub use helper::UIHelper;

use crate::assets::AssetHandles;

use self::components::spawn_gui;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
    }
}

fn init(mut commands: Commands, assets: Res<AssetHandles>) {
    commands.insert_resource(UIHelper::new());
    commands.spawn_bundle(UiCameraBundle::default());
    spawn_gui(&mut commands, assets);
}
