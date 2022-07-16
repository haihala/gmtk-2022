use bevy::prelude::*;

use crate::{
    battle::{Battle, OngoingBattle},
    flow::AppState,
    player::{Player, PlayerResources},
    ui::UIHelper,
};

mod encounters;
pub use encounters::get_random_encounter;

#[derive(Debug, Deref, DerefMut)]
pub struct OngoingEncounter(pub Encounter);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EncounterDecision {
    prompt: &'static str,
    options: Vec<(&'static str, Box<EncounterPhase>)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EncounterPhase {
    Battle(Battle),
    Line(&'static str),
    Decision(EncounterDecision),
    Gain((&'static str, PlayerResources)),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Encounter {
    phases: Vec<EncounterPhase>,
    index: usize,
    active_phase: Option<EncounterPhase>,
    waiting_decision: Option<EncounterDecision>,
}
impl Encounter {
    fn from_phases(phases: Vec<EncounterPhase>) -> Self {
        Self {
            phases,
            index: 0,
            active_phase: None,
            waiting_decision: None,
        }
    }

    fn bump_phase(&mut self) {
        self.active_phase = self.phases.get(self.index + 1).map(|item| item.to_owned());
        self.index += 1;
    }

    pub fn choose(&mut self, index: usize) {
        if let Some(decision) = self.waiting_decision.clone() {
            self.active_phase = Some(*decision.options.get(index).unwrap().1.clone());
        } else {
            dbg!(self);
            panic!("Choice is not happening, but choice was called");
        }
    }
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

fn init_encounter(
    mut commands: Commands,
    mut encounter: ResMut<OngoingEncounter>,
    mut app_state: ResMut<State<AppState>>,
    mut ui_helper: ResMut<UIHelper>,
    mut player: ResMut<Player>,
) {
    if let Some(waiting) = process_encounter_phase(
        encounter.phases[0].clone(),
        &mut commands,
        &mut app_state,
        &mut ui_helper,
        &mut player,
    ) {
        // It was a prompt, wait for answer
        encounter.waiting_decision = Some(waiting);
        encounter.active_phase = None;
    } else {
        // It was not a prompt, proceed
        encounter.waiting_decision = None;
        encounter.bump_phase();
    }
}

fn advance_encounter(
    mut commands: Commands,
    mut encounter: ResMut<OngoingEncounter>,
    mut app_state: ResMut<State<AppState>>,
    mut ui_helper: ResMut<UIHelper>,
    mut player: ResMut<Player>,
) {
    if let Some(active) = encounter.active_phase.clone() {
        if let Some(waiting) = process_encounter_phase(
            active,
            &mut commands,
            &mut app_state,
            &mut ui_helper,
            &mut player,
        ) {
            // It was a prompt, wait for answer
            encounter.waiting_decision = Some(waiting);
            encounter.active_phase = None;
        } else {
            // It was not a prompt, proceed
            encounter.waiting_decision = None;
            encounter.bump_phase();
        }
    } else if encounter.waiting_decision.is_none() {
        // No active phase, nor is the system waiting for a decision
        // Return to travel
        app_state.pop().unwrap();
        commands.remove_resource::<OngoingEncounter>();
    }
}

fn process_encounter_phase(
    phase: EncounterPhase,
    commands: &mut Commands,
    app_state: &mut ResMut<State<AppState>>,
    ui_helper: &mut ResMut<UIHelper>,
    player: &mut ResMut<Player>,
) -> Option<EncounterDecision> {
    match phase {
        EncounterPhase::Battle(battle) => {
            app_state.push(AppState::Battle).unwrap();
            commands.insert_resource(OngoingBattle(battle));
            ui_helper.show_line("Can't escape from crossing fate!");
            None
        }
        EncounterPhase::Line(line) => {
            ui_helper.show_line(line);
            None
        }
        EncounterPhase::Decision(decision) => {
            ui_helper.prompt(
                decision.prompt,
                decision
                    .options
                    .iter()
                    .map(|(line, _)| line)
                    .cloned()
                    .collect(),
            );
            Some(decision)
        }
        EncounterPhase::Gain((line, resources)) => {
            ui_helper.show_line(line);
            player.resources.add(resources);
            None
        }
    }
}
