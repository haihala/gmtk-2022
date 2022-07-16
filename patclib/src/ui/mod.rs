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
        app.add_startup_system(init)
            .add_system(update_top)
            .add_system(helper::update_helper);
    }
}

fn init(mut commands: Commands, assets: Res<AssetHandles>) {
    commands.insert_resource(UIHelper::new());
    commands.spawn_bundle(UiCameraBundle::default());
    spawn_gui(&mut commands, assets);
}

fn update_top(
    player: Res<Player>,
    mut queries: ParamSet<(
        Query<&mut Text, With<StaminaText>>,
        Query<&mut Text, With<MoneyText>>,
        Query<&mut Text, With<BulletText>>,
    )>,
) {
    queries.p0().single_mut().sections[0].value = format!("Stamina: {}", player.resources.stamina);
    queries.p1().single_mut().sections[0].value = format!("Money: {}", player.resources.money);
    queries.p2().single_mut().sections[0].value = format!("Bullets: {}", player.resources.bullets);
}
