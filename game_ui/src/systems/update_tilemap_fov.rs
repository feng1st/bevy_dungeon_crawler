use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game_core::prelude::*;

#[allow(clippy::type_complexity)]
pub fn update_tilemap_fov(
    mut player_query: Query<
        (&FieldOfView, &mut LastFieldOfView),
        (With<Player>, Changed<FieldOfView>),
    >,
    mut tilemap_query: Query<&mut TileStorage, Without<Delete>>,
    mut tile_query: Query<(&mut TileVisible, &mut TileColor)>,
) {
    let Ok(tile_storage) = tilemap_query.get_single_mut() else {
        return;
    };

    let Ok((field_of_view, mut last_field_of_view)) = player_query.get_single_mut() else {
        return;
    };

    let new_set = field_of_view
        .visible_tiles
        .difference(&last_field_of_view.0);
    let old_set = last_field_of_view
        .0
        .difference(&field_of_view.visible_tiles);

    for pos in old_set {
        let tile_pos = TilePos::new(pos.x as u32, pos.y as u32);
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            let (_, mut tile_color) = tile_query.get_mut(tile_entity).unwrap();
            *tile_color = Color::GRAY.into();
        }
    }

    for pos in new_set {
        let tile_pos = TilePos::new(pos.x as u32, pos.y as u32);
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            let (mut tile_visibility, mut tile_color) = tile_query.get_mut(tile_entity).unwrap();
            tile_visibility.0 = true;
            *tile_color = Color::WHITE.into();
        }
    }

    last_field_of_view.0 = field_of_view.visible_tiles.clone();
}