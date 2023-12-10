use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_player(mut commands: Commands, mut query: Query<(&MapRoomList, &mut MapFigureGrid)>) {
    if let Ok((map_room_list, mut map_figure_grid)) = query.get_single_mut() {
        let player_pos = IVec2::from(map_room_list.0[0].center());
        let entity = commands
            .spawn((
                Player,
                Health {
                    current: 20,
                    max: 20,
                },
                Rest {
                    current: 0,
                    max: 10,
                },
                MapPos(player_pos),
            ))
            .id();
        map_figure_grid.set(player_pos, entity);
    }
}
