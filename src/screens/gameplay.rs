use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{game::level::spawn_level as spawn_level_command, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(spawn_level_command);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
