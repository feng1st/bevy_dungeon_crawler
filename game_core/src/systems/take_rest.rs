use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn take_rest(
    mut actor_query: Query<(&mut Rest, &mut Health)>,
    mut take_rest_event_reader: EventReader<RestAction>,
) {
    for action in take_rest_event_reader.read() {
        if let Ok((mut rest, mut health)) = actor_query.get_mut(action.actor) {
            rest.current += 1;
            if rest.current >= rest.max {
                rest.current = 0;
                health.current = i32::min(health.current + 1, health.max);
            }
        }
    }
}
