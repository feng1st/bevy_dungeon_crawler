use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_tooltip(mut commands: Commands) {
    commands
        .spawn((tooltip(), Tooltip))
        .with_children(|parent| {
            parent.spawn((tooltip_text(), TooltipText));
        });
}

fn tooltip() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            padding: UiRect {
                left: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(8.),
                bottom: Val::Px(8.),
            },
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.5)),
        ..Default::default()
    }
}

fn tooltip_text() -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            font_size: 16.,
            ..default()
        },
    )
}
