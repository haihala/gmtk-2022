use bevy::prelude::*;

use crate::{battle::Battle, flow::AppState, travel::OngoingEncounter};

mod encounters;
pub use encounters::get_random_encounter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EncounterOptionOutcome {
    Battle(Battle),
    // Blocked by not having a player struct
    // Purchase {cost: i32, reward: todo!()} ,
    // Reward(),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EncounterDecision {
    prompt: String,
    options: Vec<(String, EncounterOptionOutcome)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EncounterPhase {
    Battle(Battle),
    Line(String),
    Decision(EncounterDecision),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Encounter {
    phases: Vec<EncounterPhase>,
    active_phase: usize,
}

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Encounter).with_system(init_encounter))
            .add_system_set(
                SystemSet::on_update(AppState::Encounter).with_system(advance_encounter),
            );
    }
}

fn init_encounter(encounter: Res<OngoingEncounter>) {
    process_encounter_phase(encounter.phases[0].clone());
}

fn advance_encounter(
    mut commands: Commands,
    mut encounter: ResMut<OngoingEncounter>,
    mut app_state: ResMut<State<AppState>>,
) {
    // See if user made a choice
    let advance = true;

    if advance {
        if let Some(next_phase) = encounter.phases.get(encounter.active_phase + 1) {
            // There is a phase after current one
            process_encounter_phase(next_phase.clone());
            encounter.active_phase += 1;
        } else {
            // That was the last phase
            app_state.set(AppState::Travel).unwrap();
            commands.remove_resource::<OngoingEncounter>();
        }
    }
}

fn process_encounter_phase(phase: EncounterPhase) {
    todo!()
}
