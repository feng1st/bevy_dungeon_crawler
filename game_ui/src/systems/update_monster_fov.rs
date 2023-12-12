use bevy::prelude::*;
use game_core::prelude::*;

#[allow(clippy::type_complexity)]
pub fn update_monster_fov(
    player_query: Query<&FieldOfView, With<Player>>,
    mut monster_query: Query<(&MapPos, &mut Visibility), With<Monster>>,
) {
    let Ok(field_of_view) = player_query.get_single() else {
        return;
    };

    for (map_pos, mut visibility) in &mut monster_query {
        if field_of_view.visible_tiles.contains(&map_pos.0) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
