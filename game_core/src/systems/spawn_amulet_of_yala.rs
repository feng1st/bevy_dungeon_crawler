use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala(mut commands: Commands, query: Query<&AmuletStart>) {
    let Ok(amulet_start) = query.get_single() else {
        return;
    };

    spawn_item(&mut commands, 0, amulet_start.0);
}
