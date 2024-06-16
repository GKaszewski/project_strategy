use bevy::prelude::*;
use camera::CameraPlugin;
use map::MapPlugin;

pub mod camera;
pub mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
