use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Loading Root"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("loading...").insert(Style {
                align_items: AlignItems::End,
                justify_content: JustifyContent::End,
                margin: UiRect::all(Val::VMin(2.)),
                ..default()
            });
        });
}
