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
    pub translation: Vec2,
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
            // scene: level_assets.scene.clone(),
            scene: level_assets.donut.clone(),
            transform: Transform::from_translation(config.translation.extend(1.)),
            ..default()
        },
        StateScoped(Screen::Gameplay),
    ));
}
