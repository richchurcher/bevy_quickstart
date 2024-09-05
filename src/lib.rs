mod asset_tracking;
pub mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screens;
mod theme;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use blenvy::BlenvyPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::linear_rgb(0., 0., 0.)))
            .add_plugins((DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resizable: false,
                        resolution: (1920., 1080.).into(),
                        title: "three".to_string(),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),));

        app.add_plugins((
            asset_tracking::plugin,
            BlenvyPlugin::default(),
            game::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    // commands.spawn((
    //     Name::new("Camera"),
    //     Camera2dBundle::default(),
    //     IsDefaultUiCamera,
    // ));
    commands.spawn((Name::new("3D Camera"), Camera3dBundle::default()));
}
