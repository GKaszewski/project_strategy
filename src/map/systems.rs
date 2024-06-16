use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use hexx::{algorithms::field_of_movement, *};

use crate::camera::components::GameCamera;

use super::resources::HexGrid;
use super::resources::MapSettings;
use super::resources::SelectedTile;
use super::utils::generate_terrain_hex_grid;

pub fn setup_grid(
    mut commands: Commands,
    settings: Res<MapSettings>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if !settings.is_changed() {
        return;
    }

    let layout = HexLayout {
        hex_size: settings.hex_size,
        ..default()
    };
    let entities = generate_terrain_hex_grid(
        settings.map_radius,
        settings.hex_size,
        &mut commands,
        meshes,
        materials,
    );
    commands.insert_resource(HexGrid {
        entities,
        reachable_entities: HashSet::default(),
        layout,
    });
}

pub fn handle_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut tile_transforms: Query<(Entity, &mut Transform)>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
    settings: Res<MapSettings>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();

    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        let hex_pos = grid.layout.world_pos_to_hex(pos);

        if hex_pos == *current {
            return;
        }

        *current = hex_pos;

        if let Some((tile, _)) = grid.entities.get(hex_pos) {
            selected_tile.0 = Some(tile.clone());
        } else {
            selected_tile.0 = None;
        }

        let field_of_movement = field_of_movement(hex_pos, settings.budget, |h| {
            grid.entities.get(h).and_then(|(tile, _)| tile.cost())
        });

        let reachable_entities: HashSet<_> = field_of_movement
            .into_iter()
            .filter_map(|h| grid.entities.get(h).map(|&(_, ent)| ent))
            .collect();

        for (entity, mut transform) in tile_transforms.iter_mut() {
            if reachable_entities.contains(&entity) {
                *transform = transform.with_scale(Vec3::splat(0.9));
            } else {
                *transform = transform.with_scale(Vec3::splat(1.0));
            }
        }

        grid.reachable_entities = reachable_entities;
    }
}

pub fn regenerate_grid(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<HexGrid>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<MapSettings>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        grid.entities.iter_mut().for_each(|hex| {
            for (_, entity) in hex.iter() {
                commands.entity(*entity).despawn_recursive();
            }
        });

        grid.entities = generate_terrain_hex_grid(
            settings.map_radius,
            settings.hex_size,
            &mut commands,
            meshes,
            materials,
        );
        grid.reachable_entities.clear();
    }
}
