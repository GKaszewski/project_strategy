use bevy::prelude::*;
use components::{
    AttackPoints, DefensePoints, Experience, Health, HeroMaxUnits, HeroUnits, Level,
    MovementPoints, Position, Range, UnitType,
};
use events::HeroDeselectEvent;
use systems::{
    display_field_of_movement, handle_hero_deselect, handle_hero_movement, setup_player,
};

pub mod components;
mod events;
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
            .add_event::<HeroDeselectEvent>()
            .add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (
                    display_field_of_movement,
                    handle_hero_movement,
                    handle_hero_deselect,
                ),
            );
    }
}
