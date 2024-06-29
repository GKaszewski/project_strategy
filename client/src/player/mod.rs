use bevy::prelude::*;
use components::{
    AttackPoints, DefensePoints, Experience, Health, HeroMaxUnits, HeroUnits, Level, MoveTarget,
    MovementPoints, Position, Range, UnitType,
};
use events::{HeroDeselectEvent, PathCalculatedEvent};
use systems::{
    calculate_path_system, clear_move_path, display_field_of_movement, draw_move_path,
    handle_hero_deselect, handle_hero_movement, setup_player,
};

pub mod components;
pub mod events;
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
            .register_type::<MoveTarget>()
            .add_event::<HeroDeselectEvent>()
            .add_event::<PathCalculatedEvent>()
            .add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (
                    display_field_of_movement,
                    handle_hero_deselect,
                    calculate_path_system,
                    handle_hero_movement,
                    draw_move_path,
                    clear_move_path,
                ),
            );
    }
}
