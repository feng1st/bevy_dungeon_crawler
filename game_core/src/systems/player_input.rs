use bevy::prelude::*;

use crate::prelude::*;

pub fn player_input(
    mut move_figure_event_writer: EventWriter<MoveAction>,
    mut take_rest_event_writer: EventWriter<RestAction>,
    mut next_game_turn: ResMut<NextState<GameTurn>>,
    player_query: Query<(Entity, &MapPos), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut timer: Local<u128>,
    time: Res<Time>,
) {
    *timer += time.delta().as_millis();

    let Ok((entity, map_pos)) = player_query.get_single() else {
        return;
    };

    if let Some(delta) = if keyboard_input.pressed(KeyCode::Up) {
        Some(IVec2::new(0, 1))
    } else if keyboard_input.pressed(KeyCode::Down) {
        Some(IVec2::new(0, -1))
    } else if keyboard_input.pressed(KeyCode::Left) {
        Some(IVec2::new(-1, 0))
    } else if keyboard_input.pressed(KeyCode::Right) {
        Some(IVec2::new(1, 0))
    } else if keyboard_input.pressed(KeyCode::Space) {
        Some(IVec2::new(0, 0))
    } else {
        return;
    } {
        if *timer < 100 {
            return;
        }
        *timer = 0;

        if delta.x != 0 || delta.y != 0 {
            move_figure_event_writer.send(MoveAction {
                actor: entity,
                target_pos: map_pos.0 + delta,
            });
        } else {
            take_rest_event_writer.send(RestAction { actor: entity });
        }
        next_game_turn.set(GameTurn::Monster);
    }
}
