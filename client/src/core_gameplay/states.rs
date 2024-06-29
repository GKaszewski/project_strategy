use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
    Player1Turn,
    Player2Turn,
    Player3Turn,
    Player4Turn,
    Player5Turn,
    Player6Turn,
    Player7Turn,
    Player8Turn,
    TurnTransition,
    GameOver,
}

impl Default for GameplayState {
    fn default() -> Self {
        GameplayState::Player1Turn
    }
}
