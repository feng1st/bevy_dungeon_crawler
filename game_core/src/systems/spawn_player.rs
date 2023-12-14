use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, mut query: Query<(&PlayerStart, &mut MapFigureGrid)>) {
    let Ok((player_start, mut map_figure_grid)) = query.get_single_mut() else {
        return;
    };
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
            MapPos(player_start.0),
            FieldOfView::new(8),
            LastFieldOfView::default(),
        ))
        .id();
    map_figure_grid.set(player_start.0, entity);
}
