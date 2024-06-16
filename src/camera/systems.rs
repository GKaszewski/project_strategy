use bevy::prelude::*;

use super::components::GameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}

pub fn handle_camera_input(
    mut cameras: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in cameras.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += 10.0;
        }

        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= 10.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= 10.0;
        }

        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += 10.0;
        }
    }
}
