use bevy::prelude::*;

use super::components::{Experience, Health, Hero, HeroMaxUnits, HeroUnits, Level};

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
