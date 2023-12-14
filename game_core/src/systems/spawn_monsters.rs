use bevy::prelude::*;
use rand::prelude::*;

use crate::prelude::*;

pub fn spawn_monsters(
    mut commands: Commands,
    mut query: Query<(&MonsterSpawns, &mut MapFigureGrid)>,
) {
    let mut rng = rand::thread_rng();

    let Ok((monster_spawns, mut map_figure_grid)) = query.get_single_mut() else {
        return;
    };

    for monster_spawn in &monster_spawns.0 {
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
                MapPos(*monster_spawn),
                FieldOfView::new(10),
            ))
            .id();
        map_figure_grid.set(*monster_spawn, entity);
    }
}
