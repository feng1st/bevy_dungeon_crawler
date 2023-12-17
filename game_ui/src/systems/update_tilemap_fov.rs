use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game_core::prelude::*;

#[allow(clippy::type_complexity)]
pub fn update_tilemap_fov(
    mut tile_visibility_event_reader: EventReader<TileVisibilityEvent>,
    mut tilemap_query: Query<&mut TileStorage, Without<Delete>>,
    mut tile_query: Query<(&mut TileVisible, &mut TileColor)>,
) {
    let Ok(tile_storage) = tilemap_query.get_single_mut() else {
        return;
    };

    for event in tile_visibility_event_reader.read() {
        let tile_pos = TilePos::new(event.pos.x as u32, event.pos.y as u32);
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            match event.visibility {
                TileVisibility::InSight => {
                    if let Ok((mut tile_visibility, mut tile_color)) =
                        tile_query.get_mut(tile_entity)
                    {
                        tile_visibility.0 = true;
                        *tile_color = Color::WHITE.into();
                    }
                }
                TileVisibility::Revealed => {
                    if let Ok((mut tile_visibility, mut tile_color)) =
                        tile_query.get_mut(tile_entity)
                    {
                        tile_visibility.0 = true;
                        *tile_color = Color::GRAY.into();
                    }
                }
                TileVisibility::Hidden => {
                    if let Ok((mut tile_visibility, _)) = tile_query.get_mut(tile_entity) {
                        tile_visibility.0 = false;
                    }
                }
            }
        }
    }
}
