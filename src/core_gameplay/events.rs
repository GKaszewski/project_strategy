use bevy::prelude::*;

#[derive(Event)]
pub struct TurnStartEvent {
    pub player_id: u32, // player that started their turn
}

#[derive(Event)]
pub struct TurnEndEvent {
    pub player_id: u32, // player that ended their turn
}
