use bevy::prelude::*;

use crate::{assets::AssetHandles, player::Player};

use super::components::{
    spawn_decision, spawn_highlighted_line, spawn_line, ActiveDecision, ChatBox,
};

#[derive(Debug)]
enum ChatEvent {
    Line(String),
    HighlightedLine(String),
    Prompt {
        prompt: String,
        options: Vec<String>,
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
    pub fn show_line(&mut self, line: impl Into<String>) {
        self.to_spawn.push(ChatEvent::Line(line.into()));
    }
    pub fn show_highlighted_line(&mut self, line: impl Into<String>) {
        self.to_spawn.push(ChatEvent::HighlightedLine(line.into()));
    }

    pub fn prompt(&mut self, prompt: impl Into<String>, options: Vec<impl Into<String>>) {
        self.available_options = Some(options.len());
        self.selected_option = Some(0);
        self.to_spawn.push(ChatEvent::Prompt {
            prompt: prompt.into(),
            options: options.into_iter().map(|option| option.into()).collect(),
        });
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
    chatbox_query: Query<Entity, With<ChatBox>>,
    decision_query: Query<(Entity, &Children), With<ActiveDecision>>,
    mut text_query: Query<&mut Text>,
    mut helper: ResMut<UIHelper>,
    kb_inputs: Res<Input<KeyCode>>,
    mut player: ResMut<Player>,
) {
    let queue: Vec<ChatEvent> = helper.to_spawn.drain(..).collect();

    commands
        .entity(chatbox_query.single())
        .with_children(|container| {
            helper
                .spawned
                .extend(queue.into_iter().map(|spawnable| match spawnable {
                    ChatEvent::Line(line) => spawn_line(container, &assets, line),
                    ChatEvent::HighlightedLine(line) => {
                        spawn_highlighted_line(container, &assets, line)
                    }
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

    if let (Some(selected), Some(option_count), Ok((active_decision, options))) = (
        helper.selected_option,
        helper.available_options,
        decision_query.get_single(),
    ) {
        let mut new_index = None;
        // A decision is happening
        if kb_inputs.just_pressed(KeyCode::Left) {
            // Select the option to the left
            new_index = Some(0.max(selected as i32 - 1) as usize);
        }
        if kb_inputs.just_pressed(KeyCode::Right) {
            // Select the option to the right
            new_index = Some((option_count - 1).min(selected + 1));
        }

        if let Some(index) = new_index {
            text_query.get_mut(options[selected]).unwrap().sections[0]
                .style
                .color = assets.colors.basic_text;
            text_query.get_mut(options[index]).unwrap().sections[0]
                .style
                .color = assets.colors.highlight_text;

            helper.selected_option = Some(index);
        }

        if kb_inputs.any_just_pressed(vec![KeyCode::Space, KeyCode::Return]) {
            // Accept the choice
            player.choose(selected);
            helper.clear_decision();

            // Previously active decision is no longer active
            commands.entity(active_decision).remove::<ActiveDecision>();
        }
    }
}
