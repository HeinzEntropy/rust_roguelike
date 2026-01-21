mod collision;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;
mod turn_state;

use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collision::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}
