use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala(mut commands: Commands, query: Query<&AmuletStart>) {
    let Ok(amulet_start) = query.get_single() else {
        return;
    };
    commands.spawn((
        Item,
        AmuletOfYala,
        Name("Amulet of Yala".to_string()),
        MapPos(amulet_start.0),
    ));
}
