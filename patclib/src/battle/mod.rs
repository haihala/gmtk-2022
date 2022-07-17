use bevy::prelude::*;

use crate::{
    dice_value::DiceValue,
    flow::AppState,
    player::{BattleAction, Player, PlayerResources},
    ui::UIHelper,
};

pub const BATTLE_ARENA_WIDTH: u32 = 4;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Battle {
    enemies: Vec<Enemy>,
}
impl Battle {
    /// Used to create a battle without any info on player state
    pub fn with(enemies: Vec<Enemy>) -> Self {
        Self { enemies }
    }

    fn is_over(&self) -> bool {
        self.enemies.iter().all(|enemy| enemy.health == 0)
    }

    fn place_enemies(&mut self) {
        todo!();
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
    pub position: UVec2,
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
    // - Create play area
    // - Place enemies
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
                        let selected_target = &mut battle.enemies[decision];

                        if player
                            .resources
                            .force_remove(selected_weapon.cost.unwrap_or_default())
                        {
                            // Could successfully afford to use that weapon
                            selected_target.health -= selected_weapon.damage.roll();
                        } else {
                            // Attempting to shoot, but resources are out
                            ui_helper.show_line(
                                "Try as you might, your resources ran dry before the crescendo",
                            );
                        }

                        // TODO: Handle enemies

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
                                .enemies
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
                BattleAction::Move => prompt_for_location(&mut ui_helper),
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

fn prompt_for_location(ui_helper: &mut ResMut<UIHelper>) {
    todo!()
}
