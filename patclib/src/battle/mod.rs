use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    dice_value::DiceValue,
    flow::AppState,
    player::{BattleAction, Player, PlayerResources},
    ui::UIHelper,
};

pub const BATTLE_ARENA_WIDTH: u32 = 4;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Battle {
    unused_enemies: Vec<Enemy>,
    lanes: Vec<Option<Enemy>>,
}
impl Battle {
    /// Used to create a battle without any info on player state
    pub fn with(enemies: Vec<Enemy>) -> Self {
        Self {
            unused_enemies: enemies,
            lanes: vec![None; BATTLE_ARENA_WIDTH as usize],
        }
    }

    fn is_over(&self) -> bool {
        self.lanes.iter().all(|lane| lane.is_none()) && self.unused_enemies.len() == 0
    }

    fn clean_out_dead(&mut self) {
        for lane in self.lanes.iter_mut() {
            if let Some(enemy) = lane {
                if enemy.health == 0 {
                    *lane = None;
                }
            }
        }
    }

    fn place_enemies(&mut self) {
        while self.lanes.iter().any(|lane| lane.is_none()) && self.unused_enemies.len() > 0 {
            let enemy = self.unused_enemies.pop().unwrap();
            self.place_enemy(enemy);
        }
    }

    fn place_enemy(&mut self, enemy: Enemy) {
        for lane in self.lanes.iter_mut() {
            if lane.is_none() {
                *lane = Some(enemy);
                return;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Weapon {
    pub name: &'static str,
    pub damage: DiceValue,
    pub range: u32,
    pub cost: Option<PlayerResources>,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            name: Default::default(),
            damage: Default::default(),
            range: 1, // Melee
            cost: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Enemy {
    pub name: &'static str,
    pub health: u32,
    pub weapons: Vec<Weapon>,
    pub position: u32,
}

#[derive(Debug, Deref, DerefMut)]
pub struct OngoingBattle(pub Battle);

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Battle).with_system(init_battle))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(advance_battle));
    }
}

fn init_battle(
    mut ui_helper: ResMut<UIHelper>,
    player: Res<Player>,
    mut battle: ResMut<OngoingBattle>,
) {
    // TODO:
    // - Create play area visual
    battle.place_enemies();
    prompt_for_action(&mut ui_helper, player.get_battle_actions());
}

fn advance_battle(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut ui_helper: ResMut<UIHelper>,
    mut player: ResMut<Player>,
    mut battle: ResMut<OngoingBattle>,
) {
    let mut rng = thread_rng();

    if let Some(decision) = player.decision {
        player.decision = None;

        if let Some(selected_action) = player.selected_action {
            match selected_action {
                BattleAction::Move => {
                    player.position = player.get_movable_locations()[decision].1;
                    // Next round
                    prompt_for_action(&mut ui_helper, player.get_battle_actions());
                    player.clear_selections();
                }
                BattleAction::Attack => {
                    if let Some(selected_weapon) = player.selected_weapon {
                        // Decision is about selecting a target
                        let selected_target = &mut battle.unused_enemies[decision];

                        if player
                            .resources
                            .force_remove(selected_weapon.cost.unwrap_or_default())
                        {
                            // Could successfully afford to use that weapon
                            let damage = selected_weapon.damage.roll();

                            if damage >= selected_target.health {
                                selected_target.health -= damage;
                                ui_helper.show_line(format!(
                                    "Using your {} to deal {} damage to {}, it remains steadfast",
                                    selected_weapon.name, damage, selected_target.name
                                ));
                            } else {
                                ui_helper.show_line(format!(
                                    "With your {} you rob {} of it's life",
                                    selected_weapon.name, selected_target.name
                                ));
                                selected_target.health = 0;
                            }
                            battle.clean_out_dead();
                            battle.place_enemies();
                        } else {
                            // Attempting to shoot, but resources are out
                            ui_helper.show_line(
                                "Try as you might, your resources ran dry before the crescendo",
                            );
                        }

                        for enemy in battle.unused_enemies.iter_mut() {
                            if enemy.position > 1 {
                                // Move closer
                                enemy.position -= 1;
                            } else {
                                // Attack
                                enemy.position += 2;

                                let weapon = enemy.weapons.choose(&mut rng).unwrap();
                                let damage = weapon.damage.roll();
                                player.resources.stamina -= damage;
                                ui_helper.show_line(format!(
                                    "{} uses {} to deal {} damage",
                                    enemy.name, weapon.name, damage
                                ));
                            }
                        }

                        if player.is_dead() {
                            app_state.set(AppState::GameOver).unwrap();
                        } else if battle.is_over() {
                            commands.remove_resource::<OngoingBattle>();
                            // Battle is over, return to previous state
                            app_state.pop().unwrap();
                        } else {
                            // Next round
                            prompt_for_action(&mut ui_helper, player.get_battle_actions());
                            player.clear_selections();
                        }
                    } else {
                        let selected_weapon = player.get_weapons()[decision];
                        player.selected_weapon = Some(selected_weapon);
                        prompt_for_target(
                            &mut ui_helper,
                            battle
                                .unused_enemies
                                .clone()
                                .into_iter()
                                .filter(|enemy| {
                                    // Filter to enemies in range
                                    (enemy.position - UVec2::X * player.position)
                                        .as_vec2() // For some reason, IVec doesn't have a length, I guess you can't get a neat integer length
                                        .length()
                                        .round() as u32
                                        <= selected_weapon.range
                                })
                                .collect(),
                        );
                    }
                }
            }
        } else {
            let selected_action = player.get_battle_actions()[decision];
            player.selected_action = Some(selected_action);
            match selected_action {
                BattleAction::Move => {
                    prompt_for_location(&mut ui_helper, player.get_movable_locations())
                }
                BattleAction::Attack => prompt_for_weapon(&mut ui_helper, player.get_weapons()),
            }
        }
    }
}

fn prompt_for_action(ui_helper: &mut ResMut<UIHelper>, options: Vec<BattleAction>) {
    ui_helper.prompt(
        "How do thy wish to occupy thine time",
        options.iter().map(|option| format!("{}", option)).collect(),
    );
}
fn prompt_for_weapon(ui_helper: &mut ResMut<UIHelper>, weapons: Vec<Weapon>) {
    ui_helper.prompt(
        "Once more into the frey, choose thy means of destruction",
        weapons.iter().map(|weapon| weapon.name).collect(),
    );
}

fn prompt_for_target(ui_helper: &mut ResMut<UIHelper>, enemies: Vec<Enemy>) {
    ui_helper.prompt(
        "Who is the unfortunate soul to bear the brunt of your fury",
        enemies.iter().map(|enemy| enemy.name).collect(),
    );
}

fn prompt_for_location(ui_helper: &mut ResMut<UIHelper>, locations: Vec<(&'static str, u32)>) {
    ui_helper.prompt(
        "Whereabouts",
        locations.iter().map(|location| location.0).collect(),
    );
}
