use bevy::prelude::*;

#[derive(Event)]
pub struct MoveAction {
    pub actor: Entity,
    pub target_pos: IVec2,
}

#[derive(Event)]
pub struct AttackAction {
    pub actor: Entity,
    pub target: Entity,
    pub damage: i32,
}

#[derive(Event)]
pub struct RestAction {
    pub actor: Entity,
}

#[derive(Event)]
pub struct PickupAction {
    pub actor: Entity,
    pub target_pos: IVec2,
}
