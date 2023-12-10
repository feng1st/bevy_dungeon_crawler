use bevy::prelude::*;

use crate::prelude::*;

pub fn system_pre_cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).insert(Delete);
    }
}
