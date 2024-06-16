use bevy::prelude::*;
use systems::{handle_input, regenerate_grid, setup_grid};

mod components;
mod resources;
mod systems;
mod utils;

pub struct MapPlugin;

pub const HEX_SIZE: Vec2 = Vec2::splat(16.0);
pub const MAP_RADIUS: u32 = 40;
pub const BUDGET: u32 = 7;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid)
            .add_systems(Update, (handle_input, regenerate_grid));
    }
}
