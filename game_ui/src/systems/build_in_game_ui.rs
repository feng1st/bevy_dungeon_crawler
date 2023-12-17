use bevy::prelude::*;

use crate::prelude::*;

pub fn build_in_game_ui(mut commands: Commands) {
    commands
        .spawn((root_node(), RootNode))
        .with_children(|parent| {
            parent
                .spawn(player_health_bar_box_node())
                .with_children(|parent| {
                    parent.spawn((player_health_bar_node(), PlayerHealthBar));
                    parent.spawn((player_health_label(), PlayerHealthLabel));
                });
            parent
                .spawn((inventory_box_node(), InventoryBox))
                .with_children(|parent| {
                    parent.spawn((inventory_text(), InventoryText));
                });
        });
}

fn root_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn player_health_bar_box_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            right: Val::Px(0.),
            top: Val::Px(0.),
            height: Val::Px(32.),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: Color::MAROON.into(),
        ..Default::default()
    }
}

fn player_health_bar_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        background_color: Color::RED.into(),
        ..Default::default()
    }
}

fn player_health_label() -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            font_size: 24.,
            ..default()
        },
    )
}

fn inventory_box_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            top: Val::Px(32.),
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            align_content: AlignContent::Start,
            justify_content: JustifyContent::Start,
            padding: UiRect {
                left: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(8.),
                bottom: Val::Px(8.),
            },
            ..Default::default()
        },
        background_color: Color::rgba(0., 0., 0., 0.5).into(),
        ..Default::default()
    }
}

fn inventory_text() -> TextBundle {
    TextBundle::from_sections([
        TextSection::new(
            "Items carrried",
            TextStyle {
                font_size: 16.,
                color: Color::YELLOW,
                ..default()
            },
        ),
        TextSection::new(
            "",
            TextStyle {
                font_size: 16.,
                color: Color::WHITE,
                ..default()
            },
        ),
    ])
}
