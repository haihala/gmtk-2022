use bevy::prelude::*;

use crate::assets::AssetHandles;

use super::utils::{div, div_style, FULL};

// Markers
#[derive(Debug, Component)]
pub struct ChatBox;
#[derive(Debug, Component)]
pub struct StaminaText;
#[derive(Debug, Component)]
pub struct MoneyText;
#[derive(Debug, Component)]
pub struct BulletText;
#[derive(Debug, Component)]
pub struct BatteryText;
#[derive(Debug, Component)]
pub struct ActiveDecision;

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
            bottom_bar(root, &assets);
        });
}

fn top_bar(root: &mut ChildBuilder, assets: &Res<AssetHandles>) {
    root.spawn_bundle(NodeBundle {
        color: assets.colors.dark_background,
        style: Style {
            size: Size::new(FULL, Val::Percent(3.0)),
            ..div_style()
        },
        ..div()
    })
    .with_children(|bar| {
        stat_text(bar, assets, "Stamina: 0", StaminaText);
        stat_text(bar, assets, "Money: 0", MoneyText);
        stat_text(bar, assets, "Ammo: 0", BulletText);
        stat_text(bar, assets, "Batteries: 0", BatteryText);
    });
}

fn stat_text(
    root: &mut ChildBuilder,
    assets: &Res<AssetHandles>,
    initial_text: &'static str,
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
        container
            .spawn_bundle(text_bundle(&assets, initial_text.into()))
            .insert(marker);
    });
}

fn bottom_bar(root: &mut ChildBuilder, assets: &Res<AssetHandles>) {
    root.spawn_bundle(NodeBundle {
        color: assets.colors.gray_background,
        style: Style {
            size: Size::new(FULL, Val::Percent(40.0)),
            ..div_style()
        },
        ..div()
    })
    .with_children(|bar| {
        spawn_chat_box(bar, assets);
    });
}

const BOTTOM_BOX_PORTRAIT_WIDTH: f32 = 15.0;

fn spawn_chat_box(root: &mut ChildBuilder, assets: &Res<AssetHandles>) {
    root.spawn_bundle(NodeBundle {
        color: assets.colors.dark_background,
        style: Style {
            margin: Rect {
                left: Val::Percent(BOTTOM_BOX_PORTRAIT_WIDTH),
                right: Val::Percent(BOTTOM_BOX_PORTRAIT_WIDTH),
                ..default()
            },
            size: Size {
                width: Val::Percent(100.0 - 2.0 * BOTTOM_BOX_PORTRAIT_WIDTH),
                height: FULL,
            },
            overflow: Overflow::Hidden,
            align_self: AlignSelf::FlexStart,
            flex_direction: FlexDirection::ColumnReverse,
            align_content: AlignContent::FlexStart,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_grow: 0.0,
            ..div_style()
        },
        ..div()
    })
    .insert(ChatBox);
}

pub fn spawn_line(root: &mut ChildBuilder, assets: &Res<AssetHandles>, text: String) -> Entity {
    root.spawn_bundle(spawn_message_container())
        .with_children(|container| {
            container.spawn_bundle(text_bundle(&assets, text));
        })
        .id()
}

pub fn spawn_decision(
    root: &mut ChildBuilder,
    assets: &Res<AssetHandles>,
    prompt: String,
    options: Vec<String>,
) -> Entity {
    root.spawn_bundle(spawn_message_container())
        .with_children(|container| {
            container.spawn_bundle(text_bundle(&assets, prompt));

            container
                .spawn_bundle(div())
                .insert(ActiveDecision)
                .with_children(|option_wrapper| {
                    let mut first = true;
                    for option in options {
                        if first {
                            option_wrapper.spawn_bundle(colored_text(
                                &assets,
                                option,
                                assets.colors.highlight_text,
                            ));
                            first = false;
                        } else {
                            option_wrapper.spawn_bundle(text_bundle(&assets, option));
                        }
                    }
                });
        })
        .id()
}

fn spawn_message_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: Rect::all(Val::Px(3.0)),
            flex_direction: FlexDirection::ColumnReverse,
            align_content: AlignContent::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..div_style()
        },
        ..div()
    }
}

fn text_bundle(assets: &Res<AssetHandles>, text: String) -> TextBundle {
    colored_text(assets, text, assets.colors.basic_text)
}
fn colored_text(assets: &Res<AssetHandles>, text: String, color: Color) -> TextBundle {
    TextBundle {
        style: Style { ..div_style() },
        text: Text::with_section(
            text,
            TextStyle {
                font: assets.font.clone(),
                font_size: 18.0,
                color,
            },
            default(),
        ),
        ..default()
    }
}
