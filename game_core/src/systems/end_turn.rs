use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn end_turn(
    mut next_game_turn: ResMut<NextState<GameTurn>>,
    curr_game_turn: Res<State<GameTurn>>,
) {
    match curr_game_turn.get() {
        GameTurn::Player => (),
        GameTurn::Monster => next_game_turn.set(GameTurn::Player),
    }
}
