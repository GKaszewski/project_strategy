use bevy::window::PrimaryWindow;
use bevy::{prelude::*, utils::HashSet};
use hexx::*;

use crate::camera::components::GameCamera;

use super::components::Tile;
use super::events::{TileDeselectEvent, TileSelectEvent};
use super::resources::HexGrid;
use super::resources::MapSettings;
use super::resources::SelectedTile;
use super::utils::generate_terrain_hex_grid;
use super::utils::get_color_from_biome;

pub fn setup_grid(
    mut commands: Commands,
    settings: Res<MapSettings>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
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

pub fn regenerate_grid(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<HexGrid>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<MapSettings>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        grid.entities.iter_mut().for_each(|hex| {
            for entity in hex.iter() {
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

pub fn handle_tile_selection(
    mut commands: Commands,
    mut ev_tile_select: EventWriter<TileSelectEvent>,
    mut ev_tile_desel: EventWriter<TileDeselectEvent>,
    tiles: Query<(Entity, &Tile)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    grid: ResMut<HexGrid>,
    mut current: Local<Hex>,
    mut selected_tile: ResMut<SelectedTile>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let window = windows.iter().next().unwrap();
    let (camera, cam_transform) = cameras.iter().next().unwrap();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if selected_tile.tile.is_some() && selected_tile.entity.is_some() {
            ev_tile_desel.send(TileDeselectEvent {
                tile: selected_tile.tile.clone().unwrap(),
                entity: selected_tile.entity.unwrap(),
            });
        }
        selected_tile.tile = None;
        selected_tile.entity = None;
    }

    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }

    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    if let Some(ray) = camera.viewport_to_world(cam_transform, cursor_position) {
        let Some(distance) = ray.intersect_plane(
            Vec3::new(ray.origin.x, 0.5, ray.origin.z),
            Plane3d::new(Vec3::Y),
        ) else {
            return;
        };

        let point = ray.get_point(distance);
        let hex_pos = grid.layout.world_pos_to_hex(Vec2::new(point.x, point.z));

        if hex_pos == *current {
            return;
        }
        *current = hex_pos;

        if let Some(tile_entity) = grid.entities.get(*current) {
            for (ent, tile) in tiles.iter() {
                if commands.entity(*tile_entity).id() == commands.entity(ent).id() {
                    // Deselect the previous tile
                    if selected_tile.tile.is_some() && selected_tile.entity.is_some() {
                        ev_tile_desel.send(TileDeselectEvent {
                            tile: selected_tile.tile.clone().unwrap(),
                            entity: selected_tile.entity.unwrap(),
                        });
                    }

                    selected_tile.tile = Some(tile.clone());
                    selected_tile.entity = Some(ent);

                    ev_tile_select.send(TileSelectEvent {
                        tile: tile.clone(),
                        entity: ent,
                    });
                }
            }
        }
    }
}

pub fn handle_selected_tile_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tiles: Query<(&Tile, &Handle<StandardMaterial>)>,
    mut ev_tile_select: EventReader<TileSelectEvent>,
    mut ev_tile_deselect: EventReader<TileDeselectEvent>,
) {
    for event in ev_tile_select.read() {
        for (tile_component, material_handle) in tiles.iter_mut() {
            if tile_component == &event.tile {
                let material = materials.get_mut(material_handle).unwrap();
                material.base_color = Color::rgb(1.0, 1.0, 0.0);
            }
        }
    }

    for event in ev_tile_deselect.read() {
        for (tile_component, material_handle) in tiles.iter_mut() {
            if tile_component == &event.tile {
                let material = materials.get_mut(material_handle).unwrap();
                material.base_color = get_color_from_biome(&event.tile.biome);
            }
        }
    }
}
