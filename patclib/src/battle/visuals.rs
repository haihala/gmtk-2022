use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{assets::AssetHandles, player::Player};

use super::{Enemy, OngoingBattle, BATTLE_ARENA_DEPTH, BATTLE_ARENA_WIDTH};

pub(super) fn init(
    commands: &mut Commands,
    assets: &Res<AssetHandles>,
    battle: &mut ResMut<OngoingBattle>,
    player: &Res<Player>,
) {
    draw_squares(commands, assets);
    draw_enemies(commands, assets, battle.lanes.clone());
    draw_player(commands, assets, player);
}

fn draw_squares(commands: &mut Commands, assets: &Res<AssetHandles>) {
    for y in 0..BATTLE_ARENA_DEPTH {
        for x in 0..BATTLE_ARENA_WIDTH {
            draw_tile(commands, assets, x, y);
        }
    }
}

const TILE_X_GAP: f32 = 10.0;
const TILE_Y_GAP: f32 = 10.0;
const TILE_WIDTH: f32 = 200.0;
const TILE_HEIGHT: f32 = 130.0;

const TILE_X_OFFSET: f32 = -((BATTLE_ARENA_WIDTH - 1) as f32 * TILE_WIDTH / 2.0);
const TILE_Y_OFFSET: f32 = 0.0;

const TILE_ANGLE_OFFSET: f32 = 2.0 * PI / (30.0);
const TILE_SCALE_CHANGE: f32 = 0.02 * TILE_WIDTH;

const ENEMY_SIZE_X: f32 = 100.0;
const ENEMY_SIZE_Y: f32 = 100.0;

fn tile_position(x: u32, y: u32) -> Vec3 {
    tile_position_default(x, y, 0.0, 0.0)
}

fn tile_position_default(x: u32, y: u32, x_offset: f32, y_offset: f32) -> Vec3 {
    Vec3::new(
        (x as f32) * TILE_WIDTH
            + TILE_X_OFFSET
            + (((BATTLE_ARENA_DEPTH - y) as f32) * (TILE_SCALE_CHANGE + x_offset)),
        (y as f32) * TILE_HEIGHT + TILE_Y_OFFSET + y_offset,
        0.0,
    )
}

fn draw_tile(commands: &mut Commands, assets: &Res<AssetHandles>, x: u32, y: u32) {
    let bundle: SpriteBundle = SpriteBundle {
        transform: Transform {
            translation: tile_position(x, y),
            rotation: Quat::from_rotation_y(((y as f32) + 1.0) * TILE_ANGLE_OFFSET),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(TILE_WIDTH - TILE_X_GAP, TILE_HEIGHT - TILE_Y_GAP)),
            color: assets.colors.battle_tile,
            ..default()
        },
        ..default()
    };
    commands.spawn_bundle(bundle);
}

fn draw_enemies(commands: &mut Commands, assets: &Res<AssetHandles>, lanes: Vec<Option<Enemy>>) {
    for lane in lanes.iter() {
        if let Some(enemy) = lane {
            let bundle: SpriteBundle = SpriteBundle {
                transform: Transform {
                    translation: tile_position_default(
                        enemy.position_x,
                        enemy.position_y,
                        0.0,
                        ENEMY_SIZE_Y / 2.0,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ENEMY_SIZE_X, ENEMY_SIZE_Y)),
                    ..default()
                },
                texture: enemy.handle_image.clone(),
                ..default()
            };
            commands.spawn_bundle(bundle);
        }
    }
}
fn draw_player(commands: &mut Commands, assets: &Res<AssetHandles>, player: &Res<Player>) {
    //todo!()
}
