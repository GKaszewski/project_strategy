use bevy::{
    prelude::*,
    utils::HashSet,
};
use hexx::{storage::HexagonalMap, *};

use super::components::Biome;

#[derive(Debug, Resource)]
pub struct HexGrid {
    pub entities: HexagonalMap<(Biome, Entity)>,
    pub reachable_entities: HashSet<Entity>,
    pub layout: HexLayout,
}
