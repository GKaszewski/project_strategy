use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use systems::draw_tile_stats;

pub mod components;
pub mod resources;
mod systems;

pub struct DebugGuiPlugin;

impl Plugin for DebugGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, draw_tile_stats);
    }
}
