use bevy::prelude::*;
use hexx::Hex;

use super::components::Tile;

#[derive(Event)]
pub struct TileSelectEvent {
    pub entity: Entity,
    pub tile: Tile,
    pub hex: Hex,
}

#[derive(Event)]
pub struct TileDeselectEvent {
    pub entity: Entity,
    pub tile: Tile,
    pub hex: Hex,
}
