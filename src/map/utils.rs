use super::{
    components::{Biome, HexPreviewMarker, Tile, TileResource},
    resources::HexPreview,
};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use hexx::{storage::HexagonalMap, *};
use noise::{NoiseFn, Simplex};

use rand::prelude::*;

pub fn generate_terrain_hex_grid(
    map_radius: u32,
    hex_size: Vec2,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> HexagonalMap<Entity> {
    let layout = HexLayout {
        hex_size,
        ..default()
    };

    let mesh = meshes.add(hexagonal_plane(&layout));

    let plains_mat = materials.add(Color::GREEN);
    let forest_mat = materials.add(Color::DARK_GREEN);
    let mountain_mat = materials.add(Color::GRAY);
    let shallow_water_mat = materials.add(Color::BLUE);
    let deep_water_mat = materials.add(Color::NAVY);
    let desert_mat = materials.add(Color::ORANGE);
    let snow_mat = materials.add(Color::BEIGE);

    let mut rng = rand::thread_rng();
    let seed = rng.gen();
    let simplex = Simplex::new(seed);

    HexagonalMap::new(Hex::ZERO, map_radius, |coord| {
        let elevation = simplex.get([coord.x as f64, coord.y as f64]);
        // let moisture = simplex.get([coord.x as f64 + 100.0, coord.y as f64 + 100.0]);
        let pos = layout.hex_to_world_pos(coord);
        // let biome = Biome::from_elevation_and_moisture(elevation, moisture);
        let biome = Biome::simple_biome(elevation);
        let material = match biome {
            Biome::Plains => plains_mat.clone(),
            Biome::Forest => forest_mat.clone(),
            Biome::Mountain => mountain_mat.clone(),
            Biome::ShallowWater => shallow_water_mat.clone(),
            Biome::DeepWater => deep_water_mat.clone(),
            Biome::Desert => desert_mat.clone(),
            Biome::Snow => snow_mat.clone(),
        };

        let tile = Tile::new(
            biome,
            rng.gen_range(0..100),
            rng.gen_range(0..100),
            rng.gen_range(0..100),
            TileResource::get_from_number(rng.gen_range(0..=25)),
            TileResource::get_from_number(rng.gen_range(0..=25)),
        );

        let entity = commands
            .spawn((
                Name::new("HexTile".to_string()),
                PbrBundle {
                    mesh: mesh.clone().into(),
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x, 1.0 / 2.0, pos.y),
                    ..default()
                },
                tile.clone(),
            ))
            .id();

        entity
    })
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, 1.0)
        .without_bottom_face()
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

pub fn get_color_from_biome(biome: &Biome) -> Color {
    return match biome {
        Biome::Plains => Color::GREEN,
        Biome::Forest => Color::DARK_GREEN,
        Biome::Mountain => Color::GRAY,
        Biome::ShallowWater => Color::BLUE,
        Biome::DeepWater => Color::NAVY,
        Biome::Desert => Color::ORANGE,
        Biome::Snow => Color::BEIGE,
    };
}
