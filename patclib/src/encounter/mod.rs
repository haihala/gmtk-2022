use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Encounter;

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, _: &mut App) {}
}
