use bevy::prelude::*;

mod helper;

pub use helper::UIHelper;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(UIHelper::new());
    // TODO: Create UI
}
