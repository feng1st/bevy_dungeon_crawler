use bevy::prelude::*;

use crate::prelude::*;

pub fn player_input_end_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        next_game_state.set(GameState::InGame);
    }
}
