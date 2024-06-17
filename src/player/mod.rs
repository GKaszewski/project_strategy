use bevy::prelude::*;
use components::{
    AttackPoints, DefensePoints, Experience, Health, HeroMaxUnits, HeroUnits, Level,
    MovementPoints, Position, Range, SelectedHero, UnitType,
};
use systems::setup_player;

pub mod components;
pub mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>()
            .register_type::<Experience>()
            .register_type::<Level>()
            .register_type::<Health>()
            .register_type::<AttackPoints>()
            .register_type::<DefensePoints>()
            .register_type::<MovementPoints>()
            .register_type::<Range>()
            .register_type::<UnitType>()
            .register_type::<HeroUnits>()
            .register_type::<HeroMaxUnits>()
            .register_type::<SelectedHero>()
            .add_systems(Startup, setup_player);
    }
}
