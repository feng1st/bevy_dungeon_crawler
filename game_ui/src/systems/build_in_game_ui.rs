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
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Px(32.),
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
