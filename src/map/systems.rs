use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    utils::HashSet,
    window::PrimaryWindow,
};
use hexx::{algorithms::field_of_movement, storage::HexagonalMap, *};
use rand::prelude::*;

use super::{components::Biome, resources::HexGrid, BUDGET, HEX_SIZE, MAP_RADIUS};

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };

    let mesh = meshes.add(hexagonal_plane(&layout));

    let plains_mat = materials.add(Color::WHITE);
    let forest_mat = materials.add(Color::GREEN);
    let mountain_mat = materials.add(Color::GRAY);
    let shallow_water_mat = materials.add(Color::BLUE);
    let deep_water_mat = materials.add(Color::NAVY);
    let desert_mat = materials.add(Color::ORANGE);
    let snow_mat = materials.add(Color::BEIGE);

    let mut rng = rand::thread_rng();

    let entities = HexagonalMap::new(Hex::ZERO, MAP_RADIUS, |coord| {
        let biome = match rng.gen_range(0..=6) {
            0 => Biome::Plains,
            1 => Biome::Forest,
            2 => Biome::Mountain,
            3 => Biome::ShallowWater,
            4 => Biome::DeepWater,
            5 => Biome::Desert,
            6 => Biome::Snow,
            _ => unreachable!("Invalid biome"),
        };
        let pos = layout.hex_to_world_pos(coord);
        let material = match biome {
            Biome::Plains => plains_mat.clone(),
            Biome::Forest => forest_mat.clone(),
            Biome::Mountain => mountain_mat.clone(),
            Biome::ShallowWater => shallow_water_mat.clone(),
            Biome::DeepWater => deep_water_mat.clone(),
            Biome::Desert => desert_mat.clone(),
            Biome::Snow => snow_mat.clone(),
        };

        let entity = commands
            .spawn(ColorMesh2dBundle {
                mesh: mesh.clone().into(),
                material,
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            })
            .id();

        (biome, entity)
    });

    commands.insert_resource(HexGrid {
        entities,
        reachable_entities: HashSet::default(),
        layout,
    });
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .center_aligned()
        .build();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
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
