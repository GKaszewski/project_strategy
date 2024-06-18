use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct HeroDeselectEvent {
    pub hero: Entity,
    pub button: Option<PointerButton>,
}

impl From<ListenerInput<Pointer<Down>>> for HeroDeselectEvent {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self {
            hero: value.target,
            button: Some(value.button),
        }
    }
}
