use bevy::prelude::*;

use crate::assets::AssetHandles;

use super::components::{spawn_line, ChatBox};

#[derive(Debug)]
enum ChatEvent {
    Line(&'static str),
}

const MAX_SPAWNED_EVENTS: i32 = 9;

#[derive(Debug, Default)]
pub struct UIHelper {
    to_spawn: Vec<ChatEvent>,
    spawned: Vec<Entity>,
}

impl UIHelper {
    // Interface for public use
    pub fn show_line(&mut self, line: &'static str) {
        self.to_spawn.push(ChatEvent::Line(line));
    }

    pub fn prompt(&mut self, prompt: &'static str, options: Vec<&'static str>) {
        self.to_spawn.push(ChatEvent::Line(prompt));
        dbg!(options);
    }

    // Interface for internal use
    pub(super) fn new() -> Self {
        Self::default()
    }
}

pub(super) fn update_helper(
    mut commands: Commands,
    assets: Res<AssetHandles>,
    query: Query<Entity, With<ChatBox>>,
    mut helper: ResMut<UIHelper>,
) {
    commands.entity(query.single()).with_children(|container| {
        let queue = helper.to_spawn.drain(..).collect::<Vec<_>>().into_iter();
        helper
            .spawned
            .extend(queue.map(|spawnable| match spawnable {
                ChatEvent::Line(line) => spawn_line(container, &assets, line),
            }));
    });

    let to_despawn = helper.spawned.len() as i32 - MAX_SPAWNED_EVENTS;
    if to_despawn > 0 {
        // Too many events
        for entity in helper.spawned.drain(..to_despawn as usize) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
