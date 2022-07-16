use bevy::prelude::*;

use crate::{
    encounter::{get_random_encounter, Encounter, OngoingEncounter},
    flow::AppState,
    ui::UIHelper,
};

#[derive(Debug)]
pub struct NextEncounter {
    encounter: Encounter,
    start_at: f64,
}
pub struct TravelPlugin;

impl Plugin for TravelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Travel).with_system(start_encounter));
    }
}

const WAIT_BEFORE_ENCOUNTER_STARTS: f64 = 5.0;

fn start_encounter(
    mut commands: Commands,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
    mut ui_helper: ResMut<UIHelper>,
    next_encounter: Option<Res<NextEncounter>>,
) {
    if let Some(next) = next_encounter {
        if time.seconds_since_startup() > next.start_at {
            dbg!(&next);
            commands.remove_resource::<NextEncounter>();
            commands.insert_resource(OngoingEncounter(next.encounter.clone()));
            app_state.push(AppState::Encounter).unwrap()
        }
    } else {
        ui_helper.show_line("You start looking for trouble");

        commands.insert_resource(NextEncounter {
            encounter: get_random_encounter(),
            start_at: time.seconds_since_startup() + WAIT_BEFORE_ENCOUNTER_STARTS,
        });
    }
}
