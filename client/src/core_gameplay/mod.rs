use bevy::prelude::*;
use events::{TurnEndEvent, TurnStartEvent};
use resources::TurnManager;
use states::GameplayState;
use systems::turn_end_system;
use ui::setup_ui;

pub mod components;
pub mod events;
mod resources;
pub mod states;
mod systems;
mod utils;
mod ui;

pub struct CoreGameplayPlugin;

impl Plugin for CoreGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnManager>()
            .init_state::<GameplayState>()
            .add_event::<TurnStartEvent>()
            .add_event::<TurnEndEvent>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (turn_end_system,));
    }
}
