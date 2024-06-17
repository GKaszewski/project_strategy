use bevy::prelude::*;
use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin};
use bevy_mod_picking::prelude::*;
use camera::CameraPlugin;
use debug_gui::DebugGuiPlugin;
use map::{components::Tile, resources::SelectedTile, MapPlugin};
use player::PlayerPlugin;

pub mod camera;
pub mod debug_gui;
pub mod map;
pub mod player;

fn main() {
    App::new()
        .init_resource::<SelectedTile>()
        .add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        // .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FilterQueryInspectorPlugin::<Without<Tile>>::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugGuiPlugin)
        .add_plugins(PlayerPlugin)
        //.insert_resource(DebugPickingMode::Normal)
        .run();
}
