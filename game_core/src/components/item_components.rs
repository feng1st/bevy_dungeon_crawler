use bevy::prelude::*;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Resource, Default)]
pub struct AmuletOfYalaPos(pub IVec2);
