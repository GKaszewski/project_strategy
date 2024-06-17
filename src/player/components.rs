use bevy::prelude::*;
use hexx::Hex;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Position(pub Hex);

#[derive(Component)]
pub struct Experience(pub u32);

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct AttackPoints(pub i32);

#[derive(Component)]
pub struct DefensePoints(pub i32);

#[derive(Component)]
pub struct MovementPoints(pub u32);

#[derive(Component)]
pub struct Range(pub u32);

#[derive(Component)]
pub enum UnitType {
    Melee,
    Ranged,
    Support,
    Naval,
    Air,
    Siege,
    Cavalry,
    Artillery,
    Armor,
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct HeroUnits(pub Vec<Option<Entity>>);

#[derive(Component)]
pub struct HeroMaxUnits(pub u32);

#[derive(Component)]
pub struct SelectedHero(pub Entity);
