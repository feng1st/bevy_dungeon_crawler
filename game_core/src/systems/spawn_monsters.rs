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

    for spawn in &monster_spawns.0 {
        match rng.gen_range(0..20) {
            0..=1 => spawn_item(&mut commands, rng.gen_range(1..3), *spawn),
            _ => spawn_monster(&mut commands, &mut map_figure_grid, *spawn, &mut rng),
        }
    }
}

fn spawn_monster(
    commands: &mut Commands,
    map_figure_grid: &mut MapFigureGrid,
    pos: IVec2,
    rng: &mut ThreadRng,
) {
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
            MapPos(pos),
            FieldOfView::new(10),
        ))
        .id();
    map_figure_grid.set(pos, entity);
}
