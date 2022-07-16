use bevy::prelude::*;

mod assets;
mod battle;
mod encounter;
mod flow;
mod player;
mod travel;
mod ui;

pub struct PatcPlugins;
impl PluginGroup for PatcPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(assets::AssetPlugin)
            .add(flow::FlowPlugin)
            .add(battle::BattlePlugin)
            .add(encounter::EncounterPlugin)
            .add(travel::TravelPlugin)
            .add(ui::UIPlugin)
            .add(player::PlayerPlugin);
    }
}
