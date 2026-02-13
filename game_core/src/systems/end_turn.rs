use bevy::prelude::*;

use crate::prelude::*;

pub fn end_turn(
    mut next_game_turn: ResMut<NextState<GameTurn>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    player_query: Query<(&Health, &MapPos), With<Player>>,
    amulet_query: Query<&AmuletStart>,
    map_query: Query<&MapTileGrid>,
    curr_game_turn: Res<State<GameTurn>>,
) {
    if let Ok((player_health, player_pos)) = player_query.get_single() {
        if player_health.current <= 0 {
            next_game_state.set(GameState::GameOver);
            return;
        }
        if let Ok(amulet_start) = amulet_query.get_single() {
            if player_pos.0 == amulet_start.0 {
                next_game_state.set(GameState::Victory);
                return;
            }
        }
        if let Ok(map_tile_grid) = map_query.get_single() {
            if map_tile_grid.get(player_pos.0) == TileType::Exit {
                next_game_state.set(GameState::NextLevel);
                return;
            }
        }
    }

    match curr_game_turn.get() {
        GameTurn::Player => (),
        GameTurn::Monster => next_game_turn.set(GameTurn::Player),
    }
}
