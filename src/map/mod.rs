use bevy::prelude::*;
use components::Tile;
use resources::{MapSettings, SelectedTile};
use systems::{handle_input, regenerate_grid, setup_grid};

pub mod components;
pub mod resources;
mod systems;
mod utils;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapSettings>()
            .init_resource::<SelectedTile>()
            .register_type::<Tile>()
            .add_systems(Startup, setup_grid)
            .add_systems(Update, (regenerate_grid));
    }
}
