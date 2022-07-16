use bevy::prelude::*;

use crate::assets::AssetHandles;

use super::utils::{div, div_style, FULL};

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
        bar.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..default()
            },
            text: Text::with_section(
                "Test text",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 18.0,
                    color: assets.colors.white_font,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
            ),
            ..default()
        });
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
    });
}
