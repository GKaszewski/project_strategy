use bevy::prelude::*;

use super::components::Tile;

#[derive(Event)]
pub struct TileSelectEvent {
    pub entity: Entity,
    pub tile: Tile,
}

#[derive(Event)]
pub struct TileDeselectEvent {
    pub entity: Entity,
    pub tile: Tile,
}
