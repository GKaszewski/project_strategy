use bevy::{prelude::*, window::PrimaryWindow};
use hexx::Hex;

use crate::{camera::components::GameCamera, map::resources::HexGrid};

use super::components::{Experience, Health, Hero, HeroMaxUnits, HeroUnits, Level, SelectedHero};

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
        HeroUnits(vec![None; 10]),
        HeroMaxUnits(10),
    ));
}

pub fn handle_hero_movement(
    mut hero_query: Query<(&Hero, &mut Transform), With<SelectedHero>>,
    mouse_btn: ButtonInput<MouseButton>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut tile_transforms: Query<(Entity, &mut Transform)>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
) {
    todo!()
}
