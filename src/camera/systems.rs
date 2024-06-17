use std::f32::consts::PI;

use bevy::{input::mouse::MouseWheel, prelude::*};

use super::components::GameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 60.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        },
        GameCamera,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

pub fn handle_camera_input(
    mut cameras: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for mut transform in cameras.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.z += 1.0;
        }

        if keys.pressed(KeyCode::KeyS) {
            transform.translation.z -= 1.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x += 1.0;
        }

        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x -= 1.0;
        }

        for ev in evr_scroll.read() {
            transform.translation.y -= ev.y * 10.0;
        }
    }
}
