use bevy::prelude::*;

mod battle;
mod encounter;
mod flow;
mod player_state;
mod travel;
mod ui;

pub struct PatcPlugins;
impl PluginGroup for PatcPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(flow::FlowPlugin)
            .add(battle::BattlePlugin)
            .add(encounter::EncounterPlugin)
            .add(travel::TravelPlugin)
            .add(ui::UIPlugin)
            .add(player_state::PlayerStatePlugin);
    }
}
