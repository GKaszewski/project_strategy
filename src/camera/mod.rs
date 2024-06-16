use bevy::prelude::*;
use systems::{handle_camera_input, setup_camera};

mod components;
mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, handle_camera_input);
    }
}
