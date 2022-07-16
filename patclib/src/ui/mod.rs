use bevy::prelude::*;

mod components;
mod helper;
mod utils;

pub use helper::UIHelper;

use crate::{assets::AssetHandles, player::Player};

use self::components::{spawn_gui, BulletText, MoneyText, StaminaText};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init).add_system(update);
    }
}

fn init(mut commands: Commands, assets: Res<AssetHandles>) {
    commands.insert_resource(UIHelper::new());
    commands.spawn_bundle(UiCameraBundle::default());
    spawn_gui(&mut commands, assets);
}

fn update(
    player: Res<Player>,
    mut queries: ParamSet<(
        Query<&mut Text, With<StaminaText>>,
        Query<&mut Text, With<MoneyText>>,
        Query<&mut Text, With<BulletText>>,
    )>,
) {
    for mut text in queries.p0().iter_mut() {
        text.sections[0].value = format!("{}", player.resources.stamina);
    }
    for mut text in queries.p1().iter_mut() {
        text.sections[0].value = format!("{}", player.resources.money);
    }
    for mut text in queries.p2().iter_mut() {
        text.sections[0].value = format!("{}", player.resources.bullets);
    }
}
