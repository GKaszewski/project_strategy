use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use hexx::{algorithms::field_of_movement, *};

use super::utils::generate_terrain_hex_grid;
use super::{resources::HexGrid, BUDGET, HEX_SIZE, MAP_RADIUS};

pub fn setup_grid(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    let entities = generate_terrain_hex_grid(MAP_RADIUS, &mut commands, meshes, materials);
    commands.insert_resource(HexGrid {
        entities,
        reachable_entities: HashSet::default(),
        layout,
    });
}

pub fn handle_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut tile_transforms: Query<(Entity, &mut Transform)>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
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

        let field_of_movement = field_of_movement(hex_pos, BUDGET, |h| {
            grid.entities.get(h).and_then(|(biome, _)| biome.cost())
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
) {
    if keys.just_pressed(KeyCode::KeyR) {
        grid.entities.iter_mut().for_each(|hex| {
            for (_, entity) in hex.iter() {
                commands.entity(*entity).despawn_recursive();
            }
        });

        grid.entities = generate_terrain_hex_grid(MAP_RADIUS, &mut commands, meshes, materials);
        grid.reachable_entities.clear();
    }
}
