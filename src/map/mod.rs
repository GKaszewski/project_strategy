use bevy::prelude::*;
use components::Tile;
use events::{TileDeselectEvent, TileSelectEvent};
use resources::{HexPreview, MapSettings, SelectedTile};
use systems::{handle_selected_tile_material, handle_tile_selection, regenerate_grid, setup_grid};

pub mod components;
mod events;
pub mod resources;
mod systems;
mod utils;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapSettings>()
            .init_resource::<HexPreview>()
            .register_type::<Tile>()
            .add_event::<TileSelectEvent>()
            .add_event::<TileDeselectEvent>()
            .add_systems(Startup, setup_grid)
            .add_systems(
                Update,
                (
                    regenerate_grid,
                    handle_tile_selection,
                    handle_selected_tile_material,
                ),
            );
    }
}
