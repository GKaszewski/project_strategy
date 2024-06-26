use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {}
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> bool;
}
