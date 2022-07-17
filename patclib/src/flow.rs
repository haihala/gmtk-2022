use bevy::{app::AppExit, prelude::*};

use crate::encounter::{game_over, game_start, OngoingEncounter};

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

/*
1. Game starts at MainMenu
2. Initial encounter starts, push encounter as state
3. State is popped once that is done, back in MainMenu
4. Travel is set as state
5. Travel randomizes encounters pushes encounter as state
6. Encounters push battles as state
7. Battle pops state
8. Encounter pops state
9. Repeat from 4
10. Player dies, GameOver is set as state
11. Final encounter set active, push encounter state
12. Once encounter state is popped, wait a bit and close the game

iirc on_enter doesn't register when a state is resumed
 */

fn prompt_to_start(mut commands: Commands, mut app_state: ResMut<State<AppState>>) {
    commands.insert_resource(OngoingEncounter(game_start()));
    app_state.push(AppState::Encounter).unwrap();
}

fn start_game(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Travel).unwrap();
}

fn start_end_game(mut commands: Commands, mut app_state: ResMut<State<AppState>>) {
    commands.insert_resource(OngoingEncounter(game_over()));
    app_state.push(AppState::Encounter).unwrap();
}

#[derive(Debug)]
struct TimeToQuit(f64);

fn end_game(
    mut commands: Commands,
    time: Res<Time>,
    time_to_quit: Option<Res<TimeToQuit>>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(ttq) = time_to_quit {
        if ttq.0 < time.seconds_since_startup() {
            exit.send(AppExit);
        }
    } else {
        commands.insert_resource(TimeToQuit(time.seconds_since_startup() + 5.0));
    }
}
