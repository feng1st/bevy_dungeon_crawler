use bevy::prelude::*;
use bevy_fov::fov;

use crate::prelude::*;

pub fn calc_fov(
    mut figure_query: Query<(&MapPos, &mut FieldOfView), Changed<MapPos>>,
    map_query: Query<&MapTileGrid>,
) {
    let Ok(map_tile_grid) = map_query.get_single() else {
        return;
    };

    for (map_pos, mut field_of_view) in &mut figure_query {
        let vt = fov::compute(map_pos.0, field_of_view.radius, map_tile_grid);
        field_of_view.visible_tiles = vt;
    }
}
