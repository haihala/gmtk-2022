use bevy::prelude::*;

use crate::{battle::Battle, encounter::Encounter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    Travel,
    Encounter(Encounter),
    Battle(Battle),
}

pub struct FlowPlugin;
impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::MainMenu)
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(start_game));
    }
}

fn start_game(mut app_state: ResMut<State<AppState>>, inputs: Res<Input<KeyCode>>) {
    if inputs.any_just_pressed(vec![KeyCode::Return, KeyCode::Space]) {
        dbg!("Game started");
        app_state.set(AppState::Travel).unwrap();
    }
}
