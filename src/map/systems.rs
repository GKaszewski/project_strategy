use bevy::window::PrimaryWindow;
use bevy::{prelude::*, utils::HashSet};
use hexx::*;

use crate::camera::components::GameCamera;
use crate::map::components::Cross;
use crate::player::components::{MoveTarget, SelectedHero};
use crate::player::events::HeroDeselectEvent;

use super::components::Tile;
use super::events::{TileDeselectEvent, TileSelectEvent};
use super::resources::HexGrid;
use super::resources::MapSettings;
use super::utils::generate_terrain_hex_grid;

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
        grid.entities.iter_mut().for_each(|(_, entity)| {
            commands.entity(*entity).despawn_recursive();
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
    mut ev_hero_deselect: EventWriter<HeroDeselectEvent>,
    tiles: Query<(Entity, &Tile)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    grid: ResMut<HexGrid>,
    mut current: Local<Hex>,
    selected_hero_query: Query<Entity, With<SelectedHero>>,
    move_target_query: Query<Option<&MoveTarget>, With<SelectedHero>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if selected_hero_query.iter().next().is_none() {
        return;
    }

    let selected_hero_entity = selected_hero_query.iter().next().unwrap();

    let window = windows.iter().next().unwrap();
    let (camera, cam_transform) = cameras.iter().next().unwrap();

    let local_hex = *current;

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if move_target_query.iter().next().is_some() {
            commands.entity(selected_hero_entity).remove::<MoveTarget>();

            ev_hero_deselect.send(HeroDeselectEvent {
                hero: selected_hero_entity,
                button: None,
            });

            for (ent, tile) in tiles.iter() {
                if grid.entities.get(&local_hex).map(|&e| e) == Some(ent) {
                    ev_tile_desel.send(TileDeselectEvent {
                        tile: tile.clone(),
                        entity: ent,
                        hex: *current,
                    });
                }
            }
        }
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

        *current = hex_pos;

        if let Some(tile_entity) = grid.entities.get(&hex_pos) {
            for (ent, tile) in tiles.iter() {
                if tile.cost() == None {
                    continue;
                }

                if grid.entities.get(&hex_pos).map(|&e| e) == Some(ent) {
                    ev_tile_desel.send(TileDeselectEvent {
                        tile: tile.clone(),
                        entity: ent,
                        hex: *current,
                    });
                }

                commands
                    .entity(selected_hero_entity)
                    .insert(MoveTarget(*current));

                if commands.entity(*tile_entity).id() == commands.entity(ent).id() {
                    ev_tile_select.send(TileSelectEvent {
                        tile: tile.clone(),
                        entity: ent,
                        hex: *current,
                    });
                }
            }
        }
    }
}

pub fn handle_selected_tile_material(
    mut ev_tile_select: EventReader<TileSelectEvent>,
    mut ev_tile_deselect: EventReader<TileDeselectEvent>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    hex_grid: Res<HexGrid>,
    mut cross_query: Query<(Entity, &Cross)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in ev_tile_select.read() {
        let hex = event.hex;
        let cross_sprite = assets_server.load("sprites/cross.png");
        let world_position = hex_grid.layout.hex_to_world_pos(hex);

        let quad = meshes.add(Rectangle::new(8.0, 8.0));
        let quad_material = materials.add(StandardMaterial {
            base_color_texture: Some(cross_sprite.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: quad.clone().into(),
                material: quad_material.clone(),
                transform: Transform::from_xyz(world_position.x, 2.0, world_position.y)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            Cross,
            Name::new("Cross".to_string()),
        ));
    }

    ev_tile_deselect.read().for_each(|_| {
        for (entity, _) in cross_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
    });
}
