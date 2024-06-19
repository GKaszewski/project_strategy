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
        Experience, HasCalculatedFieldOfMovement, HasCalculatedPath, HasMoved, Health, Hero,
        HeroMaxUnits, HeroUnits, Level, MovePath, MovePathPreview, MoveTarget, MovementPoints,
        SelectedHero,
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
                if movement_points.0 < tile.cost()? {
                    return None;
                }

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
    movement_points: u32,
) -> Option<Vec<Hex>> {
    a_star(start, goal, |_, h| {
        for (entity, tile) in tiles.iter() {
            if grid.entities.get(&h).map(|&ent| ent) == Some(*entity) {
                if movement_points < tile.cost()? {
                    return None;
                }

                return tile.cost();
            }
        }

        None
    })
}

pub fn handle_hero_movement(
    mut hero_query: Query<
        (
            Entity,
            Option<&mut MovePath>,
            &mut Transform,
            Option<&HasMoved>,
        ),
        With<SelectedHero>,
    >,
    grid: Res<HexGrid>,
    mut commands: Commands,
) {
    for (hero_entity, move_path_option, mut transform, has_moved) in hero_query.iter_mut() {
        if has_moved.is_some() {
            continue;
        }

        // todo: move player along the path
        let move_path = match move_path_option {
            Some(move_path) => move_path,
            None => continue,
        };

        let mut path = move_path.0.iter().rev().map(|&entity| {
            grid.entities
                .iter()
                .find_map(|(hex, ent)| if *ent == entity { Some(hex) } else { None })
                .unwrap()
        }); // now we have the path reversed from goal to start

        // we have to get the first element of the path that is in reach of the hero

        let first_reachable_hex =
            path.find(|&hex| grid.reachable_entities.contains(&grid.entities[hex]));

        if let Some(first_reachable_hex) = first_reachable_hex {
            let move_pos = grid.layout.hex_to_world_pos(*first_reachable_hex);

            transform.translation = Vec3::new(move_pos.x, transform.translation.y, move_pos.y);

            commands
                .entity(hero_entity)
                .remove::<MovePath>()
                .remove::<HasCalculatedFieldOfMovement>()
                .remove::<HasCalculatedPath>()
                .insert(HasMoved);
        }
    }
}

pub fn calculate_path_system(
    mut commands: Commands,
    grid: ResMut<HexGrid>,
    tiles: Query<(Entity, &Tile)>,
    hero_query: Query<
        (Entity, &Hero, &Transform, &MovementPoints, &MoveTarget),
        With<SelectedHero>,
    >,
    mut ev_tile_select: EventReader<TileSelectEvent>,
    mut ev_path_calculated: EventWriter<PathCalculatedEvent>,
) {
    ev_tile_select.read().for_each(|_| {
        for (hero_entity, _, transform, hero_movement_points, move_target) in hero_query.iter() {
            let start = grid
                .layout
                .world_pos_to_hex(Vec2::new(transform.translation.x, transform.translation.z));
            let goal = move_target.0;

            let path = calculate_path(
                start,
                goal,
                &grid,
                tiles.iter().collect(),
                hero_movement_points.0,
            )
            .unwrap_or_else(|| vec![]);

            let path_entites: Vec<Entity> = path
                .iter()
                .filter_map(|h| grid.entities.get(h).map(|&ent| ent))
                .collect();

            commands.entity(hero_entity).insert(MovePath(path_entites));

            commands.entity(hero_entity).insert(HasCalculatedPath);
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
