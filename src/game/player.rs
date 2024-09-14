use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use crate::{asset_tracking::LevelAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Donut>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Donut;

#[derive(Debug)]
pub struct SpawnPlayer {
    pub translation: Vec3,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        Name::new("Player"),
        Donut,
        SceneBundle {
            scene: level_assets.donut.clone(),
            transform: Transform::from_translation(config.translation),
            ..default()
        },
        StateScoped(Screen::Gameplay),
    ));
    commands.spawn((
        Name::new("3D Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(config.translation, Vec3::Y),
            ..default()
        },
        StateScoped(Screen::Gameplay),
    ));
    commands.spawn((
        Name::new("Point light"),
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        },
        StateScoped(Screen::Title),
    ));
}
