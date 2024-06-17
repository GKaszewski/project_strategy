use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;
use hexx::{algorithms::field_of_movement, Hex};

use crate::{
    camera::components::GameCamera,
    map::{components::Tile, resources::HexGrid},
};

use super::{
    components::{
        Experience, HasCalculatedFieldOfMovement, Health, Hero, HeroMaxUnits, HeroUnits, Level,
        MovementPoints, SelectedHero,
    },
    events::HeroDeselectEvent,
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
            if grid.entities.get(h).map(|&ent| ent) == Some(entity) {
                return tile.cost();
            }
        }

        None
    });

    let reachable_entities: HashSet<_> = field_of_movement
        .into_iter()
        .filter_map(|h| grid.entities.get(h).map(|&ent| ent))
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

pub fn handle_hero_movement(
    mut hero_query: Query<(&Hero, &mut Transform), With<SelectedHero>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut tile_transforms: Query<(Entity, &mut Transform), (With<Tile>, Without<SelectedHero>)>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
) {
    return;
}

pub fn handle_hero_deselect(
    mut ev_hero_deselect: EventReader<HeroDeselectEvent>,
    mut commands: Commands,
    mut grid: ResMut<HexGrid>,
    mut tile_transforms: Query<(Entity, &mut Transform), With<Tile>>,
) {
    for event in ev_hero_deselect.read() {
        if event.button == PointerButton::Secondary {
            commands
                .entity(event.hero)
                .remove::<SelectedHero>()
                .remove::<HasCalculatedFieldOfMovement>();
            grid.reachable_entities.clear();

            for (_, mut transform) in tile_transforms.iter_mut() {
                *transform = transform.with_scale(Vec3::splat(1.0));
            }
        }
    }
}
