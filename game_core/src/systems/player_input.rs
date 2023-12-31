use bevy::prelude::*;

use crate::prelude::*;

const KEYSTROKE_INTERVAL_MIN: u128 = 150;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn player_input(
    mut move_figure_event_writer: EventWriter<MoveAction>,
    mut take_rest_event_writer: EventWriter<RestAction>,
    mut pickup_event_writer: EventWriter<PickupAction>,
    mut use_item_event_writer: EventWriter<UseItemAction>,
    mut next_game_turn: ResMut<NextState<GameTurn>>,
    player_query: Query<(Entity, &MapPos), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut timer: Local<u128>,
    time: Res<Time>,
) {
    *timer += time.delta().as_millis();

    let Ok((player_entity, player_pos)) = player_query.get_single() else {
        return;
    };

    let mut event_write_fn: Box<dyn FnMut()>;
    if keyboard_input.pressed(KeyCode::Up) {
        event_write_fn = Box::new(|| {
            move_figure_event_writer.send(MoveAction {
                actor: player_entity,
                target_pos: player_pos.0 + IVec2::new(0, 1),
            });
        });
    } else if keyboard_input.pressed(KeyCode::Down) {
        event_write_fn = Box::new(|| {
            move_figure_event_writer.send(MoveAction {
                actor: player_entity,
                target_pos: player_pos.0 + IVec2::new(0, -1),
            });
        });
    } else if keyboard_input.pressed(KeyCode::Left) {
        event_write_fn = Box::new(|| {
            move_figure_event_writer.send(MoveAction {
                actor: player_entity,
                target_pos: player_pos.0 + IVec2::new(-1, 0),
            });
        });
    } else if keyboard_input.pressed(KeyCode::Right) {
        event_write_fn = Box::new(|| {
            move_figure_event_writer.send(MoveAction {
                actor: player_entity,
                target_pos: player_pos.0 + IVec2::new(1, 0),
            });
        });
    } else if keyboard_input.pressed(KeyCode::Space) {
        event_write_fn = Box::new(|| {
            take_rest_event_writer.send(RestAction {
                actor: player_entity,
            });
        });
    } else if keyboard_input.pressed(KeyCode::G) {
        event_write_fn = Box::new(|| {
            pickup_event_writer.send(PickupAction {
                actor: player_entity,
                target_pos: player_pos.0,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key1) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 0,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key2) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 1,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key3) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 2,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key4) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 3,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key5) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 4,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key6) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 5,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key7) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 6,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key8) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 7,
            });
        });
    } else if keyboard_input.pressed(KeyCode::Key9) {
        event_write_fn = Box::new(|| {
            use_item_event_writer.send(UseItemAction {
                actor: player_entity,
                target_index: 8,
            });
        });
    } else {
        return;
    }

    if *timer < KEYSTROKE_INTERVAL_MIN {
        return;
    }
    *timer = 0;

    event_write_fn();

    next_game_turn.set(GameTurn::Monster);
}
