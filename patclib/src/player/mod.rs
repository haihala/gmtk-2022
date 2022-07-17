use core::fmt;

use bevy::prelude::*;

use crate::{
    battle::{Weapon, BATTLE_ARENA_WIDTH},
    dice_value::DiceValue,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BattleAction {
    Move,
    Attack,
    Wait,
}
impl fmt::Display for BattleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BattleAction::Move => write!(f, "move"),
            BattleAction::Attack => write!(f, "attack"),
            BattleAction::Wait => write!(f, "wait"),
        }
    }
}

// Implement functionality for different ammo types/currencies if there is time.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Player {
    pub resources: PlayerResources,
    pub decision: Option<usize>,
    pub weapons: Vec<Weapon>,
    pub selected_action: Option<BattleAction>,
    pub selected_weapon: Option<Weapon>,
    pub position: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            resources: PlayerResources {
                stamina: 100,
                money: "5d6".into(),
                batteries: "5d6".into(),
                bullets: 15,
            },
            weapons: vec![
                Weapon {
                    name: "Trusty sidearm",
                    damage: "1d6".into(),
                    range: 5,
                    cost: Some(PlayerResources {
                        bullets: 1,
                        ..default()
                    }),
                },
                Weapon {
                    name: "Still somewhat trusty taser",
                    damage: "1d6".into(),
                    range: 2,
                    cost: Some(PlayerResources {
                        batteries: "4".into(),
                        ..default()
                    }),
                },
                Weapon {
                    name: "Knuckle sandwich",
                    damage: "1".into(),
                    range: 1,
                    cost: None,
                },
            ],
            ..default()
        }
    }
    pub fn is_dead(&self) -> bool {
        self.resources.stamina == 0
    }

    pub fn choose(&mut self, index: usize) {
        self.decision = Some(index);
    }

    pub fn get_battle_actions(&self) -> Vec<BattleAction> {
        // A function so if we want to later on add statuses that prevent moving or something, it's easier.
        vec![BattleAction::Attack, BattleAction::Move, BattleAction::Wait]
    }

    pub fn get_weapons(&self) -> Vec<Weapon> {
        self.weapons
            .iter()
            .filter(|weapon| {
                weapon.cost.is_none() || self.resources.could_afford(&weapon.cost.unwrap())
            })
            .cloned()
            .collect()
    }

    pub fn get_movable_locations(&self) -> Vec<(&'static str, u32)> {
        let mut collector = vec![];

        if self.position > 0 {
            collector.push(("left", self.position - 1));
        }

        if self.position < BATTLE_ARENA_WIDTH - 1 {
            collector.push(("right", self.position + 1));
        }
        collector
    }

    pub fn clear_selections(&mut self) {
        self.selected_action = None;
        self.selected_weapon = None;
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayerResources {
    pub stamina: u32,
    pub money: DiceValue,
    pub bullets: u32,
    pub batteries: DiceValue,
}
impl PlayerResources {
    pub fn add(&mut self, other: PlayerResources) {
        self.stamina += other.stamina;
        self.money += other.money;
        self.batteries += other.batteries;
        self.bullets += other.bullets;
    }
    pub fn could_afford(&self, other: &PlayerResources) -> bool {
        // We could technically maybe afford something (theoretical maximum for uncertains)
        self.stamina >= other.stamina
            && self.bullets >= other.bullets
            && self.money.theoretical_limit() >= other.money.theoretical_limit()
            && self.batteries.theoretical_limit() >= other.batteries.theoretical_limit()
    }
    pub fn remove(&mut self, other: PlayerResources) -> bool {
        let can_afford = self.could_afford(&other);
        if can_afford {
            if let (Some(new_money), Some(new_batteries)) = (
                self.money.drained_to_match(other.money.roll()),
                self.batteries.drained_to_match(other.batteries.roll()),
            ) {
                self.money = new_money;
                self.batteries = new_batteries;
                self.stamina -= other.stamina;
                self.bullets -= other.bullets;
            }
        }

        can_afford
    }

    pub fn force_remove(&mut self, other: PlayerResources) -> bool {
        let can_afford = self.could_afford(&other);
        if can_afford {
            let new_money = self.money.drained_to_match(other.money.roll());
            let new_batteries = self.batteries.drained_to_match(other.batteries.roll());

            self.money = new_money.unwrap_or_default();
            self.batteries = new_batteries.unwrap_or_default();
            self.stamina -= other.stamina;
            self.bullets -= other.bullets;

            new_money.is_some() && new_batteries.is_some()
        } else {
            false
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(Player::new());
}
