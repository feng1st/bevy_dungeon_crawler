use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala(mut commands: Commands, query: Query<&AmuletStart>) {
    let Ok(amulet_start) = query.get_single() else {
        return;
    };
    commands.spawn((Item, AmuletOfYala, MapPos(amulet_start.0)));
}
