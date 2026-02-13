use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::type_complexity)]
pub fn reveal_map(
    mut player_query: Query<
        (&FieldOfView, &mut LastFieldOfView),
        (With<Player>, Changed<FieldOfView>),
    >,
    mut map_query: Query<&mut MapVisibilityGrid>,
    mut tile_visibility_event_writer: EventWriter<TileVisibilityEvent>,
) {
    let Ok(mut map_visibility_grid) = map_query.get_single_mut() else {
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

    let mut events = vec![];

    for &pos in old_set {
        map_visibility_grid.set_at(pos, TileVisibility::Revealed);
        events.push(TileVisibilityEvent {
            pos,
            visibility: TileVisibility::Revealed,
        });
    }

    for &pos in new_set {
        map_visibility_grid.set_at(pos, TileVisibility::InSight);
        events.push(TileVisibilityEvent {
            pos,
            visibility: TileVisibility::InSight,
        });
    }

    tile_visibility_event_writer.send_batch(events);

    last_field_of_view.0.clone_from(&field_of_view.visible_tiles);
}
