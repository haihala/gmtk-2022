use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Battle;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, _: &mut App) {}
}
