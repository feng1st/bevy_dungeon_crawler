use bevy::prelude::*;

use crate::prelude::*;

pub fn build_victory_ui(mut commands: Commands) {
    commands
        .spawn((root_node(), RootNode))
        .with_children(|parent| {
            parent.spawn(box_node()).with_children(|parent| {
                parent.spawn(text_node("You have won!", 60.0, Color::GREEN));
                parent.spawn(text_node(
                    "You put on the Amulet of Yala and feel its power course through your veins.",
                    32.0,
                    Color::WHITE,
                ));
                parent.spawn(text_node(
                    "Your town is saved, and you can return to your normal life.",
                    32.0,
                    Color::WHITE,
                ));
                parent.spawn(text_node("Press 1 to play again.", 32.0, Color::GREEN));
            });
        });
}

fn root_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn box_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            padding: UiRect {
                left: Val::Px(60.),
                right: Val::Px(60.),
                top: Val::Px(60.),
                bottom: Val::Px(60.),
            },
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(32.),
            ..Default::default()
        },
        background_color: Color::rgba(0., 0., 0., 0.5).into(),
        ..Default::default()
    }
}

fn text_node(value: &str, font_size: f32, color: Color) -> TextBundle {
    TextBundle::from_section(
        value,
        TextStyle {
            font_size,
            color,
            ..default()
        },
    )
}
