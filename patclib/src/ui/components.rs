use bevy::prelude::*;

use crate::assets::AssetHandles;

use super::utils::{div, div_style, FULL};

// Markers
#[derive(Debug, Component)]
pub struct TextBox;
#[derive(Debug, Component)]
pub struct StaminaText;
#[derive(Debug, Component)]
pub struct MoneyText;
#[derive(Debug, Component)]
pub struct BulletText;

pub(super) fn spawn_gui(commands: &mut Commands, assets: Res<AssetHandles>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::SpaceBetween,
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                ..div_style()
            },
            ..div()
        })
        .with_children(|root| {
            top_bar(root, &assets);
            text_box(root, &assets);
        });
}

fn top_bar(root: &mut ChildBuilder, assets: &Res<AssetHandles>) {
    root.spawn_bundle(NodeBundle {
        color: assets.colors.dark_background,
        style: Style {
            size: Size::new(FULL, Val::Percent(3.0)),
            padding: Rect {
                top: Val::Auto,
                bottom: Val::Auto,
                ..default()
            },
            ..div_style()
        },
        ..div()
    })
    .with_children(|bar| {
        stat_text(bar, assets, "Stamina: ", StaminaText);
        stat_text(bar, assets, "Money: ", MoneyText);
        stat_text(bar, assets, "Ammo: ", BulletText);
    });
}

fn stat_text(
    root: &mut ChildBuilder,
    assets: &Res<AssetHandles>,
    text: &'static str,
    marker: impl Component,
) {
    root.spawn_bundle(NodeBundle {
        style: Style {
            align_self: AlignSelf::Center,
            margin: Rect {
                right: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        ..div()
    })
    .with_children(|container| {
        container.spawn_bundle(TextBundle {
            text: Text::with_section(
                text,
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 18.0,
                    color: assets.colors.white_font,
                },
                default(),
            ),
            ..default()
        });
        container
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "0",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 18.0,
                        color: assets.colors.white_font,
                    },
                    default(),
                ),
                ..default()
            })
            .insert(marker);
    });
}

fn text_box(root: &mut ChildBuilder, assets: &Res<AssetHandles>) {
    root.spawn_bundle(NodeBundle {
        color: assets.colors.gray_background,
        style: Style {
            size: Size::new(FULL, Val::Percent(40.0)),
            ..div_style()
        },
        ..div()
    })
    .with_children(|bar| {
        message_container(bar, assets);
    });
}

const BOTTOM_BOX_PORTRAIT_WIDTH: f32 = 15.0;

fn message_container(root: &mut ChildBuilder, assets: &Res<AssetHandles>) {
    root.spawn_bundle(NodeBundle {
        color: assets.colors.dark_background,
        style: Style {
            margin: Rect {
                left: Val::Percent(BOTTOM_BOX_PORTRAIT_WIDTH),
                right: Val::Percent(BOTTOM_BOX_PORTRAIT_WIDTH),
                ..default()
            },
            ..div_style()
        },
        ..div()
    })
    .insert(TextBox);
}
