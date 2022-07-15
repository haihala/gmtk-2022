use bevy::prelude::*;

use crate::{
    encounter::{get_random_encounter, Encounter},
    flow::AppState,
};

#[derive(Debug)]
pub struct NextEncounter {
    encounter: Encounter,
    start_at: f64,
}
#[derive(Debug, Deref, DerefMut)]
pub struct OngoingEncounter(pub Encounter);

pub struct TravelPlugin;

impl Plugin for TravelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Travel).with_system(init_travel))
            .add_system_set(SystemSet::on_update(AppState::Travel).with_system(start_encounter));
    }
}

const WAIT_BEFORE_ENCOUNTER_STARTS: f64 = 5.0;

fn init_travel(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(NextEncounter {
        encounter: get_random_encounter(),
        start_at: time.seconds_since_startup() + WAIT_BEFORE_ENCOUNTER_STARTS,
    });
}

fn start_encounter(
    mut commands: Commands,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
    next_encounter: Res<NextEncounter>,
) {
    if time.seconds_since_startup() > next_encounter.start_at {
        dbg!(&next_encounter);
        commands.remove_resource::<NextEncounter>();
        commands.insert_resource(OngoingEncounter(next_encounter.encounter.clone()));
        app_state.set(AppState::Encounter).unwrap()
    }
}
