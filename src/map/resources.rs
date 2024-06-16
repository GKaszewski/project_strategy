use bevy::{prelude::*, utils::HashSet};
use bevy_inspector_egui::prelude::*;
use hexx::{storage::HexagonalMap, *};

use super::components::Tile;

#[derive(Debug, Resource)]
pub struct HexGrid {
    pub entities: HexagonalMap<(Tile, Entity)>,
    pub reachable_entities: HashSet<Entity>,
    pub layout: HexLayout,
}

impl Default for HexGrid {
    fn default() -> Self {
        Self {
            entities: HexagonalMap::new(Hex::ZERO, 0, |_| (Tile::default(), Entity::PLACEHOLDER)),
            reachable_entities: HashSet::default(),
            layout: HexLayout::default(),
        }
    }
}

#[derive(Debug, Resource, Reflect, InspectorOptions)]
pub struct MapSettings {
    pub hex_size: Vec2,
    #[inspector(min = 1, max = 100)]
    pub map_radius: u32,
    #[inspector(min = 1, max = 10)]
    pub budget: u32,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            hex_size: Vec2::splat(16.0),
            map_radius: 80,
            budget: 7,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SelectedTile(pub Option<Tile>);

impl Default for SelectedTile {
    fn default() -> Self {
        Self(None)
    }
}
