use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use debug_gui::DebugGuiPlugin;
use map::{resources::SelectedTile, MapPlugin};
use player::PlayerPlugin;

pub mod camera;
pub mod debug_gui;
pub mod map;
pub mod player;

fn main() {
    App::new()
        .init_resource::<SelectedTile>()
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::default())
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugGuiPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
