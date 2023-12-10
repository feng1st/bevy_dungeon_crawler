use bevy::prelude::*;

use crate::prelude::*;

pub fn hide_tooltip(mut tooltip_style_query: Query<&mut Style, With<Tooltip>>) {
    for mut tooltip_style in &mut tooltip_style_query {
        tooltip_style.display = Display::None;
    }
}
