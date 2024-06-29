use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::map::resources::SelectedTile;

pub fn draw_tile_stats(mut contexts: EguiContexts, selected_tile: Res<SelectedTile>) {
    if selected_tile.tile.is_none() {
        return;
    }

    if let Some(tile) = &selected_tile.tile {
        egui::Window::new("Tile Info").show(contexts.ctx_mut(), |ui| {
            ui.label("Tile Info");
            ui.separator();
            ui.label(format!(
                "Selected Tile: {:?}, Movement Cost: {:?}",
                tile,
                tile.cost()
            ));
        });
    }
}
