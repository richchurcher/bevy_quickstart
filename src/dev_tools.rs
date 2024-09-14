//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{states::log_transitions, ui_debug_overlay::UiDebugOptions},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiSettings, quick::WorldInspectorPlugin};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new());
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.insert_resource(EguiSettings {
        scale_factor: 1.5,
        ..default()
    });
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
