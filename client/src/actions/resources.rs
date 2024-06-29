use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);
