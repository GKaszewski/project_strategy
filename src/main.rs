use bevy::prelude::*;

use camera::CameraPlugin;
use debug_gui::DebugGuiPlugin;
use map::MapPlugin;

pub mod camera;
pub mod debug_gui;
pub mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugGuiPlugin)
        .run();
}
