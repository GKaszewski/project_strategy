use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub fn setup_ui(mut commands: Commands) {
    let root = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::End,
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section("End Turn", TextStyle {
                ..default()
            }));
        });
    }).id();

    commands.entity(root).insert(Pickable::IGNORE);
}