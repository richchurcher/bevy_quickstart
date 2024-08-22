use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, Actionlike,
    InputManagerBundle,
};

use crate::{asset_tracking::TextureAssets, screens::Screen, AppSet};

use super::{animation::PlayerAnimation, movement::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.add_plugins(InputManagerPlugin::<Move>::default());

    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug)]
pub struct SpawnPlayer {
    pub max_speed: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 3, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let input_map = InputMap::new([
        (Move::Up, KeyCode::KeyW),
        (Move::Right, KeyCode::KeyD),
        (Move::Down, KeyCode::KeyS),
        (Move::Left, KeyCode::KeyA),
    ]);
    commands.spawn((
        InputManagerBundle::with_map(input_map),
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: texture_assets.player.clone(),
            transform: Transform::from_scale(Vec2::splat(1.0).extend(1.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController {
            max_speed: config.max_speed,
            ..default()
        },
        player_animation,
        StateScoped(Screen::Gameplay),
    ));
}

fn record_player_directional_input(
    mut player: Query<(&ActionState<Move>, &mut MovementController), With<Player>>,
) {
    if let Ok((action_state, mut movement_controller)) = player.get_single_mut() {
        let mut intent = Vec2::ZERO;
        if action_state.pressed(&Move::Up) {
            intent.y += 1.;
        }
        if action_state.pressed(&Move::Down) {
            intent.y -= 1.;
        }
        if action_state.pressed(&Move::Right) {
            intent.x += 1.;
        }
        if action_state.pressed(&Move::Left) {
            intent.x -= 1.;
        }

        // Normalize so that diagonal movement has the same speed as
        // horizontal and vertical movement.
        // This should be omitted if the input comes from an analog stick instead.
        let intent = intent.normalize_or_zero();

        movement_controller.intent = intent;
    }
}
