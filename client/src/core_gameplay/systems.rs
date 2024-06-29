use bevy::prelude::*;

use crate::player::components::{HasMoved, Hero, Player1Marker};

use super::{
    events::{TurnEndEvent, TurnStartEvent},
    resources::TurnManager,
    states::GameplayState,
};

pub fn turn_end_system(
    mut ev_turn_ends: EventReader<TurnEndEvent>,
    mut turn_manager: ResMut<TurnManager>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    ev_turn_ends
        .read()
        .for_each(|_| match turn_manager.current_state {
            GameplayState::Player1Turn => next_state.set(GameplayState::Player2Turn),
            GameplayState::Player2Turn => next_state.set(GameplayState::Player3Turn),
            GameplayState::Player3Turn => next_state.set(GameplayState::Player4Turn),
            GameplayState::Player4Turn => next_state.set(GameplayState::Player5Turn),
            GameplayState::Player5Turn => next_state.set(GameplayState::Player6Turn),
            GameplayState::Player6Turn => next_state.set(GameplayState::Player7Turn),
            GameplayState::Player7Turn => next_state.set(GameplayState::Player8Turn),
            GameplayState::Player8Turn => {
                turn_manager.current_turn += 1;
                next_state.set(GameplayState::Player1Turn);

                if turn_manager.current_turn > turn_manager.max_turns {
                    next_state.set(GameplayState::GameOver);
                }
            }
            _ => (),
        });
}

pub fn handle_player1_turn_system(
    mut commands: Commands,
    state: Res<State<GameplayState>>,
    hero_query: Query<(Entity, Option<&HasMoved>), (With<Hero>, With<Player1Marker>)>,
    mut ev_turn_start: EventReader<TurnStartEvent>,
) {
    if state.get() != &GameplayState::Player1Turn {
        return;
    }

    for _ in ev_turn_start.read() {
        for (entity, has_moved) in hero_query.iter() {
            if let Some(_) = has_moved {
                commands.entity(entity).remove::<HasMoved>();
            }
        }
    }
}
