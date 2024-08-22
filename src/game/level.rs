use bevy::{ecs::world::Command, prelude::*};

use super::player::SpawnPlayer;

pub(super) fn plugin(_app: &mut App) {}

/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    SpawnPlayer { max_speed: 400.0 }.apply(world);
}
