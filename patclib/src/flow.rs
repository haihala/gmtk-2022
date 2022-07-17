use bevy::{app::AppExit, prelude::*};

use crate::{player::Player, ui::UIHelper};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    GameOver,
    Travel,
    Encounter,
    Battle,
}

pub struct FlowPlugin;
impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::MainMenu)
            .add_startup_system_to_stage(StartupStage::PostStartup, prompt_to_start)
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(start_game))
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(start_end_game))
            .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(end_game));
    }
}

fn prompt_to_start(mut ui_helper: ResMut<UIHelper>) {
    ui_helper.show_line("Welcome to the frontier. You are a cowboy in charge of a pupper");
    ui_helper.show_line("Use the arrow keys to navigate and space or enter to select");
    ui_helper.show_line("Adventure awaits");
    ui_helper.prompt(
        "Start your adventure?",
        vec![
            "Simple yes",
            "Sarcastic yes",
            "No but actually yes",
            "I got confused",
        ],
    )
}

fn start_game(
    mut app_state: ResMut<State<AppState>>,
    mut player: ResMut<Player>,
    mut ui_helper: ResMut<UIHelper>,
) {
    if let Some(choice) = player.drain_decision() {
        match choice {
            0 => {
                ui_helper.show_line("That's the sprit!");
            }
            1 => {
                ui_helper.show_line("Aren't you a rascal! Too bad you have no agency");
            }
            2 => {
                ui_helper.show_line(
                    "Denying the quest, how heroic. Too bad there is a game to be played.",
                );
            }
            3 => {
                ui_helper
                    .show_line("Yeah sometimes it be like that. Hopefully you'll figure it out");
            }
            _ => panic!("How did you do that?"),
        }

        app_state.set(AppState::Travel).unwrap();
    }
}

fn start_end_game(mut ui_helper: ResMut<UIHelper>) {
    ui_helper.show_line("Well, sometimes it that's how the dice fall. Can't win them all.");
    ui_helper.show_line("Some people can't win them any");
    ui_helper.show_line("Hopefully you found some enjoyment out of this. But I have to go now.");
    ui_helper.prompt(
        "Wait what?",
        vec![
            "Who are you?",
            "Cheers",
            "*Nod and tip your hat",
            "I'm still confused, even moreso than before",
        ],
    )
}

#[derive(Debug)]
struct TimeToQuit(f64);

fn end_game(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mut ui_helper: ResMut<UIHelper>,
    time: Res<Time>,
    time_to_quit: Option<Res<TimeToQuit>>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(ttq) = time_to_quit {
        if ttq.0 < time.seconds_since_startup() {
            exit.send(AppExit);
        }
    } else if let Some(choice) = player.drain_decision() {
        match choice {
            0 => {
                ui_helper.show_line("Does it matter?");
            }
            1 => {
                ui_helper.show_line("It was fun while it lasted");
            }
            2 => {
                ui_helper.show_line("Pardner, *tips back");
            }
            3 => {
                ui_helper.show_line("I sincerely hope you figure it out");
            }
            _ => panic!("How did you do that?"),
        }
        commands.insert_resource(TimeToQuit(time.seconds_since_startup() + 5.0));
    }
}
