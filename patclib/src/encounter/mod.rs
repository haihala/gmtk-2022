use bevy::prelude::*;

use crate::battle::Battle;

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
pub struct Encounter(Vec<EncounterPhase>);

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, _: &mut App) {}
}
