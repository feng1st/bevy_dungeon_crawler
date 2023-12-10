use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Item;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct AmuletOfYala;

#[derive(Resource, Default)]
pub struct AmuletOfYalaPos(pub IVec2);
