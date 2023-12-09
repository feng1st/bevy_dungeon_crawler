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
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba_u8(0, 0, 0, 127)),
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
