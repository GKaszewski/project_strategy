use bevy::prelude::*;
use systems::setup_player;

pub mod components;
pub mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}
