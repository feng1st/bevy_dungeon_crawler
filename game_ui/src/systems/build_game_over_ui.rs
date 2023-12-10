use bevy::prelude::*;

use crate::prelude::*;

pub fn build_game_over_ui(mut commands: Commands) {
    commands
        .spawn((root_node(), RootNode))
        .with_children(|parent| {
            parent.spawn(title_bar_node());
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
