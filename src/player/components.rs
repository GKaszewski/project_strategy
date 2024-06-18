use bevy::prelude::*;
use hexx::Hex;

#[derive(Component)]
pub struct Hero;

#[derive(Component, Reflect)]
pub struct Position(pub Hex);

#[derive(Component, Reflect)]
pub struct Experience(pub u32);

#[derive(Component, Reflect)]
pub struct Level(pub u32);

#[derive(Component, Reflect)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Component, Reflect)]
pub struct AttackPoints(pub i32);

#[derive(Component, Reflect)]
pub struct DefensePoints(pub i32);

#[derive(Component, Reflect, Debug)]
pub struct MovementPoints(pub u32);

#[derive(Component, Reflect)]
pub struct Range(pub u32);

#[derive(Component, Reflect)]
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

#[derive(Component, Reflect)]
pub struct Unit;

#[derive(Component, Reflect)]
pub struct HeroUnits(pub Vec<Option<Entity>>);

#[derive(Component, Reflect)]
pub struct HeroMaxUnits(pub u32);

#[derive(Component)]
pub struct SelectedHero(pub Entity);

#[derive(Component)]
pub struct HasCalculatedFieldOfMovement;

#[derive(Component, Reflect)]
pub struct MoveTarget(pub Hex);
