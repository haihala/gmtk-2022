use bevy::prelude::*;

use crate::{assets::AssetHandles, encounter::OngoingEncounter};

use super::components::{spawn_decision, spawn_line, ChatBox};

#[derive(Debug)]
enum ChatEvent {
    Line(&'static str),
    Prompt {
        prompt: &'static str,
        options: Vec<&'static str>,
    },
}

const MAX_SPAWNED_EVENTS: i32 = 6;

#[derive(Debug, Default)]
pub struct UIHelper {
    to_spawn: Vec<ChatEvent>,
    spawned: Vec<Entity>,
    selected_option: Option<usize>,
    available_options: Option<usize>,
}

impl UIHelper {
    // Interface for public use
    pub fn show_line(&mut self, line: &'static str) {
        self.to_spawn.push(ChatEvent::Line(line));
    }

    pub fn prompt(&mut self, prompt: &'static str, options: Vec<&'static str>) {
        self.available_options = Some(options.len());
        self.selected_option = Some(0);
        self.to_spawn.push(ChatEvent::Prompt { prompt, options });
    }

    fn clear_decision(&mut self) {
        self.available_options = None;
        self.selected_option = None;
    }

    // Interface for internal use
    pub(super) fn new() -> Self {
        let mut item = Self::default();
        // Aight so hear me out
        // At the moment, the system looks weird if there is a low number of items shown
        // Flood it with a lot of empty items to make adding the second item look fine.
        for _ in 0..10 {
            item.show_line("");
        }
        item
    }
}

pub(super) fn update_helper(
    mut commands: Commands,
    assets: Res<AssetHandles>,
    query: Query<Entity, With<ChatBox>>,
    mut helper: ResMut<UIHelper>,
    kb_inputs: Res<Input<KeyCode>>,
    encounter: Option<ResMut<OngoingEncounter>>,
) {
    commands.entity(query.single()).with_children(|container| {
        let queue = helper.to_spawn.drain(..).collect::<Vec<_>>().into_iter();
        helper
            .spawned
            .extend(queue.map(|spawnable| match spawnable {
                ChatEvent::Line(line) => spawn_line(container, &assets, line),
                ChatEvent::Prompt { prompt, options } => {
                    spawn_decision(container, &assets, prompt, options)
                }
            }));
    });

    let to_despawn = helper.spawned.len() as i32 - MAX_SPAWNED_EVENTS;
    if to_despawn > 0 {
        // Too many events
        for entity in helper.spawned.drain(..to_despawn as usize) {
            commands.entity(entity).despawn_recursive();
        }
    }

    if let (Some(selected), Some(option_count)) = (helper.selected_option, helper.available_options)
    {
        // A decision is happening
        if kb_inputs.just_pressed(KeyCode::Left) {
            // Select the option to the left
            helper.selected_option = Some(0.max(selected - 1));
        }
        if kb_inputs.just_pressed(KeyCode::Right) {
            // Select the option to the right
            helper.selected_option = Some((option_count - 1).min(selected + 1));
        }
        if kb_inputs.any_just_pressed(vec![KeyCode::Space, KeyCode::Return]) {
            // Accept the choice
            encounter.unwrap().choose(selected);
            helper.clear_decision();
        }
    }
}
