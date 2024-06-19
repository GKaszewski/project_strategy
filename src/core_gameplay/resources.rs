use bevy::prelude::*;

use super::states::GameplayState;

#[derive(Resource)]
pub struct TurnManager {
    pub current_turn: u32,
    pub max_turns: u32,
    pub current_state: GameplayState,
}

pub enum MaxTurns {
    Quick,
    Normal,
    Long,
    LongLongLong,
    Marathon,
    Infinite,
}

impl MaxTurns {
    pub fn get_max_turns(&self) -> u32 {
        match self {
            MaxTurns::Quick => 50,
            MaxTurns::Normal => 100,
            MaxTurns::Long => 150,
            MaxTurns::LongLongLong => 175,
            MaxTurns::Marathon => 200,
            MaxTurns::Infinite => u32::MAX,
        }
    }
}

impl Default for TurnManager {
    fn default() -> Self {
        TurnManager {
            current_turn: 1,
            max_turns: MaxTurns::Normal.get_max_turns(),
            current_state: GameplayState::Player1Turn,
        }
    }
}
