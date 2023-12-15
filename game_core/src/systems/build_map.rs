use bevy::prelude::*;
use rand::Rng;

use crate::prelude::*;

const MAP_BOUND: IVec2 = IVec2::new(80, 50);

pub fn build_map(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let map_builder: &dyn MapBuilder = match rng.gen_range(0..10) {
        0..=2 => &DungeonMapBuilder,
        3..=5 => &AutomataMapBuilder,
        6..=8 => &DrunkardMapBuilder,
        _ => &EmptyMapBuilder,
    };

    let mut map_bundle = map_builder.build(MAP_BOUND);

    try_apply_prefab(&mut map_bundle, 20);

    commands.spawn((Map, map_bundle));
}
