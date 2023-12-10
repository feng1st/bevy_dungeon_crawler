use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_monsters(
    mut commands: Commands,
    mut query: Query<(&MapRoomList, &mut MapFigureGrid)>,
) {
    let mut rng = rand::thread_rng();

    if let Ok((map_room_list, mut map_figure_grid)) = query.get_single_mut() {
        for room in map_room_list.0.iter().skip(1) {
            let map_pos = IVec2::from(room.center());

            let monster_id = match rng.gen_range(0..20) {
                0..=1 => 0,
                2..=5 => 1,
                6..=11 => 2,
                _ => 3,
            };

            let (name, _, health) = MONSTER_TABLE[monster_id];

            let entity = commands
                .spawn((
                    Monster(monster_id),
                    Name(name.to_string()),
                    Health {
                        current: health,
                        max: health,
                    },
                    SimpleAI,
                    MapPos(map_pos),
                ))
                .id();
            map_figure_grid.set(map_pos, entity);
        }
    }
}
