use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {}
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> bool;
}
