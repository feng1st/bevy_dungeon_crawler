use std::fmt::Write;

use bevy::prelude::*;
use game_core::prelude::Name;
use game_core::prelude::*;

use crate::prelude::*;

#[allow(clippy::type_complexity)]
pub fn update_in_game_ui_inventory(
    mut inventory_box_query: Query<&mut Style, With<InventoryBox>>,
    mut inventory_text_query: Query<&mut Text, With<InventoryText>>,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<(&Name, &Carried), (With<Item>, With<Carried>)>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    let Ok(mut inventory_box) = inventory_box_query.get_single_mut() else {
        return;
    };

    let Ok(mut inventory_text) = inventory_text_query.get_single_mut() else {
        return;
    };

    let mut text = String::new();
    for (no, (name, _)) in item_query
        .iter()
        .filter(|(_, carried)| carried.0 == player_entity)
        .enumerate()
    {
        let _ = write!(text, "\n{} - {}", no + 1, name.0);
    }

    if text.is_empty() {
        inventory_box.display = Display::None;
    } else {
        inventory_text.sections[1].value = text;
        inventory_box.display = Display::Flex;
    }
}
