use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use patclib::PatcPlugins;

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        })
        .add_plugins(PatcPlugins)
        .run();
}
