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
    Gain(&'static str, PlayerResources),
    Lose(&'static str, PlayerResources),
    Trade(&'static str, &'static str, PlayerResources, PlayerResources),
    Loop(Vec<EncounterPhase>),
    Break,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Encounter {
    stack: Vec<Vec<EncounterPhase>>,
    stack_pointers: Vec<usize>,
}
impl Encounter {
    fn from_phases(phases: Vec<EncounterPhase>) -> Self {
        Self {
            stack: vec![phases],
            stack_pointers: vec![0],
            ..default()
        }
    }

    fn move_forward(&mut self) {
        *self.stack_pointers.last_mut().unwrap() += 1;
        if self.get_active_phase().is_none() {
            if self.in_a_loop() {
                // Back to loop start
                *self.stack_pointers.last_mut().unwrap() = 0;
            }
        }
    }

    fn get_active_phase(&self) -> Option<EncounterPhase> {
        if let (Some(stack_frame), Some(pointer)) = (self.stack.last(), self.stack_pointers.last())
        {
            stack_frame.get(*pointer).map(|item| item.to_owned())
        } else {
            None
        }
    }

    fn waiting_for_input(&self) -> bool {
        matches!(
            self.get_active_phase(),
            Some(EncounterPhase::Decision(_) | EncounterPhase::Battle(_))
        )
    }

    fn break_loop(&mut self) {
        self.stack.pop();
        self.stack_pointers.pop();

        // To move over the decision that caused the loop in the first place
        self.move_forward();
    }

    fn start_loop(&mut self, stack_frame: Vec<EncounterPhase>) {
        self.stack.push(stack_frame);
        self.stack_pointers.push(0);
    }

    fn in_a_loop(&self) -> bool {
        self.stack.len() > 1
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
    event_loop(
        &mut encounter,
        &mut commands,
        &mut app_state,
        &mut ui_helper,
        &mut player,
    );
}

fn advance_encounter(
    mut commands: Commands,
    mut encounter: ResMut<OngoingEncounter>,
    mut app_state: ResMut<State<AppState>>,
    mut ui_helper: ResMut<UIHelper>,
    mut player: ResMut<Player>,
) {
    if let Some(decision) = player.drain_decision() {
        // Only act if player has done something
        if let Some(EncounterPhase::Decision(next_phase)) = encounter.get_active_phase() {
            process_encounter_phase(
                &mut encounter,
                *next_phase.options.get(decision).unwrap().1.clone(),
                &mut commands,
                &mut app_state,
                &mut ui_helper,
                &mut player,
            );
        } else {
            panic!("Got input while not waiting for input");
        };
        event_loop(
            &mut encounter,
            &mut commands,
            &mut app_state,
            &mut ui_helper,
            &mut player,
        );
    }
}

fn event_loop(
    encounter: &mut ResMut<OngoingEncounter>,
    commands: &mut Commands,
    app_state: &mut ResMut<State<AppState>>,
    ui_helper: &mut ResMut<UIHelper>,
    player: &mut ResMut<Player>,
) {
    loop {
        if let Some(phase) = encounter.get_active_phase() {
            dbg!(&phase);
            process_encounter_phase(encounter, phase, commands, app_state, ui_helper, player);
            if encounter.waiting_for_input() {
                // This will show the question being prompted for
                let phase = encounter.get_active_phase().unwrap();
                process_encounter_phase(encounter, phase, commands, app_state, ui_helper, player);
                break;
            }
        } else {
            // Ran out of phases, encounter is over
            commands.remove_resource::<OngoingEncounter>();
            app_state.pop().unwrap();
            break;
        }
    }
}

fn process_encounter_phase(
    encounter: &mut ResMut<OngoingEncounter>,
    phase: EncounterPhase,
    commands: &mut Commands,
    app_state: &mut ResMut<State<AppState>>,
    ui_helper: &mut ResMut<UIHelper>,
    player: &mut ResMut<Player>,
) {
    match phase {
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
        }
        EncounterPhase::Battle(battle) => {
            app_state.push(AppState::Battle).unwrap();
            commands.insert_resource(OngoingBattle(battle));
            ui_helper.show_line("Can't escape from crossing fate!");
            encounter.move_forward();
        }
        EncounterPhase::Line(line) => {
            ui_helper.show_line(line);
            encounter.move_forward();
        }
        EncounterPhase::Gain(line, resources) => {
            ui_helper.show_line(line);
            player.resources.add(resources);
            encounter.move_forward();
        }
        EncounterPhase::Lose(line, resources) => {
            ui_helper.show_line(line);
            player.resources.remove(resources);
            encounter.move_forward();
        }
        EncounterPhase::Trade(line_success, line_failure, resources_cost, resources_reward) => {
            if player.resources.remove(resources_cost) {
                player.resources.add(resources_reward);
                ui_helper.show_line(line_success);
            } else {
                ui_helper.show_line(line_failure);
            }
            encounter.move_forward();
        }
        EncounterPhase::Break => {
            encounter.break_loop();
        }
        EncounterPhase::Loop(phases) => {
            encounter.start_loop(phases);
        }
    }
}
