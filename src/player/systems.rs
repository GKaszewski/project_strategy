use bevy::{prelude::*, utils::HashSet};
use bevy_mod_picking::prelude::*;
use hexx::{
    algorithms::{a_star, field_of_movement},
    Hex,
};

use crate::map::{
    components::Tile,
    events::{TileDeselectEvent, TileSelectEvent},
    resources::HexGrid,
};

use super::{
    components::{
        Experience, HasCalculatedFieldOfMovement, HasCalculatedPath, Health, Hero, HeroMaxUnits,
        HeroUnits, Level, MovePath, MovePathPreview, MoveTarget, MovementPoints, SelectedHero,
    },
    events::{HeroDeselectEvent, PathCalculatedEvent},
};

pub fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let hero_shape = meshes.add(Cuboid {
        half_size: Vec3::new(8.0, 8.0, 8.0),
    });
    let hero_mat = materials.add(Color::rgb(0.0, 0.0, 1.0));

    commands.spawn((
        Name::new("Hero".to_string()),
        PbrBundle {
            mesh: hero_shape,
            material: hero_mat,
            transform: Transform::from_translation(Vec3::new(0.0, 11.0, 1.0)),
            ..Default::default()
        },
        Hero,
        Experience(0),
        Level(1),
        Health {
            current: 100,
            max: 100,
        },
        MovementPoints(5),
        HeroUnits(vec![None; 10]),
        HeroMaxUnits(10),
        PickableBundle::default(),
        On::<Pointer<Click>>::target_commands_mut(|click, target_commands| {
            if click.button == PointerButton::Primary {
                target_commands.insert(SelectedHero { 0: click.target });
            }
        }),
        On::<Pointer<Down>>::send_event::<HeroDeselectEvent>(),
    ));
}

pub fn display_field_of_movement(
    mut commands: Commands,
    selected_hero: Query<
        (
            Entity,
            &MovementPoints,
            &Transform,
            Option<&HasCalculatedFieldOfMovement>,
        ),
        With<SelectedHero>,
    >,
    tiles: Query<(Entity, &Tile)>,
    mut tile_transforms: Query<(Entity, &mut Transform), (With<Tile>, Without<SelectedHero>)>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
) {
    if selected_hero.iter().count() != 1 {
        return;
    }

    let (hero_entity, movement_points, hero_transform, has_calculated_fom) = selected_hero.single();
    let hero_hex = grid.layout.world_pos_to_hex(Vec2::new(
        hero_transform.translation.x,
        hero_transform.translation.z,
    ));

    if has_calculated_fom.is_some() {
        return;
    }

    *current = hero_hex;

    let field_of_movement = field_of_movement(hero_hex, movement_points.0, |h| {
        for (entity, tile) in tiles.iter() {
            if grid.entities.get(&h).map(|&ent| ent) == Some(entity) {
                return tile.cost();
            }
        }

        None
    });

    let reachable_entities: HashSet<_> = field_of_movement
        .into_iter()
        .filter_map(|h| grid.entities.get(&h).map(|&ent| ent))
        .collect();

    for (entity, mut transform) in tile_transforms.iter_mut() {
        if reachable_entities.contains(&entity) {
            *transform = transform.with_scale(Vec3::splat(0.9));
        } else {
            *transform = transform.with_scale(Vec3::splat(1.0));
        }
    }

    grid.reachable_entities = reachable_entities;
    commands
        .entity(hero_entity)
        .insert(HasCalculatedFieldOfMovement);
}

fn calculate_path(
    start: Hex,
    goal: Hex,
    grid: &HexGrid,
    tiles: Vec<(Entity, &Tile)>,
) -> Option<Vec<Hex>> {
    a_star(start, goal, |_, h| {
        for (entity, tile) in tiles.iter() {
            if grid.entities.get(&h).map(|&ent| ent) == Some(*entity) {
                return tile.cost();
            }
        }

        None
    })
}

pub fn handle_hero_movement(
    mut hero_query: Query<(Entity, Option<&MovePath>, &mut Transform), With<SelectedHero>>,
    grid: Res<HexGrid>,
    mut commands: Commands,
) {
    for (hero_entity, move_path_option, mut transform) in hero_query.iter_mut() {
        // todo: move player along the path
    }
}

pub fn calculate_path_system(
    mut commands: Commands,
    grid: ResMut<HexGrid>,
    tiles: Query<(Entity, &Tile)>,
    move_target_query: Query<(Entity, &MoveTarget), With<SelectedHero>>,
    hero_transform_query: Query<(&Hero, &Transform), With<SelectedHero>>,
    mut ev_tile_select: EventReader<TileSelectEvent>,
    mut ev_path_calculated: EventWriter<PathCalculatedEvent>,
) {
    ev_tile_select.read().for_each(|_| {
        if move_target_query.iter().count() != 1 {
            return;
        }

        let move_target = move_target_query.single();
        for (_, transform) in hero_transform_query.iter() {
            let start = grid
                .layout
                .world_pos_to_hex(Vec2::new(transform.translation.x, transform.translation.z));
            let goal = move_target.1 .0;
            let goal_entity = grid.entities.get(&goal).map(|&ent| ent).unwrap();

            let path = calculate_path(start, goal, &grid, tiles.iter().collect())
                .unwrap_or_else(|| vec![]);

            // check if the goal is reachable
            if grid.reachable_entities.contains(&goal_entity) {
                let path_entities: HashSet<_> = path
                    .clone()
                    .into_iter()
                    .filter_map(|h| grid.entities.get(&h).map(|&ent| ent))
                    .collect();
                commands
                    .entity(move_target.0)
                    .insert(MovePath(path_entities.clone()));
            } else {
                // if it is not reachable, get the closest to the goal that is reachable
                path.iter().rev().for_each(|h| {
                    if let Some(entity) = grid.entities.get(h) {
                        if grid.reachable_entities.contains(entity) {
                            let path_entities: HashSet<_> = path
                                .clone()
                                .into_iter()
                                .filter_map(|h| grid.entities.get(&h).map(|&ent| ent))
                                .collect();

                            commands
                                .entity(move_target.0)
                                .insert(MovePath(path_entities.clone()));
                            return;
                        }
                    }
                });
            }

            commands.entity(move_target.0).insert(HasCalculatedPath);
            ev_path_calculated.send(PathCalculatedEvent);
        }
    });
}

pub fn handle_hero_deselect(
    mut ev_hero_deselect: EventReader<HeroDeselectEvent>,
    mut commands: Commands,
    mut grid: ResMut<HexGrid>,
    mut tile_transforms: Query<(Entity, &mut Transform), With<Tile>>,
) {
    for event in ev_hero_deselect.read() {
        if event.button.is_none() {
            commands
                .entity(event.hero)
                .remove::<SelectedHero>()
                .remove::<HasCalculatedFieldOfMovement>();
            grid.reachable_entities.clear();

            for (_, mut transform) in tile_transforms.iter_mut() {
                *transform = transform.with_scale(Vec3::splat(1.0));
            }
        }

        if let Some(button) = event.button {
            match button {
                PointerButton::Secondary => {
                    commands
                        .entity(event.hero)
                        .remove::<SelectedHero>()
                        .remove::<HasCalculatedFieldOfMovement>();
                    grid.reachable_entities.clear();

                    for (_, mut transform) in tile_transforms.iter_mut() {
                        *transform = transform.with_scale(Vec3::splat(1.0));
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn draw_move_path(
    mut commands: Commands,
    grid: ResMut<HexGrid>,
    move_path_query: Query<(Entity, &MovePath), With<SelectedHero>>,
    assets_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ev_path_calculated: EventReader<PathCalculatedEvent>,
) {
    ev_path_calculated.read().for_each(|_| {
        for (_, move_path) in move_path_query.iter() {
            for entity in move_path.0.iter() {
                let donut_sprite = assets_server.load("sprites/donut.png");

                let quad = meshes.add(Rectangle::new(8.0, 8.0));
                let quad_material = materials.add(StandardMaterial {
                    base_color_texture: Some(donut_sprite.clone()),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                });

                // check if the entity is in the grid
                if let Some(hex) =
                    grid.entities.iter().find_map(
                        |(h, ent)| {
                            if *ent == *entity {
                                Some(h)
                            } else {
                                None
                            }
                        },
                    )
                {
                    let pos = grid.layout.hex_to_world_pos(*hex);
                    commands.spawn(((
                        PbrBundle {
                            mesh: quad.clone().into(),
                            material: quad_material.clone(),
                            transform: Transform::from_xyz(pos.x, 2.0, pos.y)
                                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                            ..default()
                        },
                        MovePathPreview(*entity),
                        Name::new("MovePathPreview".to_string()),
                    ),));
                }
            }
        }
    });
}

pub fn clear_move_path(
    mut commands: Commands,
    mut ev_tile_deselect: EventReader<TileDeselectEvent>,
    move_path_preview_query: Query<Entity, With<MovePathPreview>>,
    move_path_query: Query<(Entity, &MovePath), With<SelectedHero>>,
) {
    ev_tile_deselect.read().for_each(|_| {
        for entity in move_path_preview_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for (entity, _) in move_path_query.iter() {
            commands
                .entity(entity)
                .remove::<MovePath>()
                .remove::<HasCalculatedPath>();
        }
    });
}
