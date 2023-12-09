use bevy::prelude::*;

use crate::prelude::*;

pub fn build_in_game_ui(mut commands: Commands) {
    commands
        .spawn((root_node(), RootNode))
        .with_children(|parent| {
            parent.spawn(title_bar_node()).with_children(|parent| {
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

fn title_bar_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.),
            height: Val::Px(32.),
            ..Default::default()
        },
        background_color: Color::BLUE.into(),
        ..Default::default()
    }
}

fn player_health_label() -> TextBundle {
    TextBundle::from_sections([
        TextSection::new(
            "Health: ",
            TextStyle {
                font_size: 24.,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: 24.,
            ..default()
        }),
        TextSection::new(
            " / ",
            TextStyle {
                font_size: 24.,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: 24.,
            ..default()
        }),
    ])
}
