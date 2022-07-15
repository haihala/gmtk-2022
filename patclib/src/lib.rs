use bevy::prelude::*;

mod flow;

pub struct PatcPlugins;
impl PluginGroup for PatcPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(flow::FlowPlugin);
    }
}
