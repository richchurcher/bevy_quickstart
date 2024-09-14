use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraSettings>()
        .add_systems(Update, orbit.in_set(AppSet::Update));
}

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            // These values are completely arbitrary, chosen because they seem to produce
            // "sensible" results for this example. Adjust as required.
            orbit_distance: 5.0,
            pitch_speed: 0.5,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 1.0,
        }
    }
}

fn orbit(
    mut camera: Query<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };

    let mut delta_pitch = 0.0;
    let mut delta_roll = 0.0;
    let mut delta_yaw = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        delta_pitch += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta_pitch -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        delta_roll -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        delta_roll += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        delta_yaw -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta_yaw += 1.0;
    }

    // Incorporating the delta time between calls prevents this from being framerate-bound.
    delta_pitch *= camera_settings.pitch_speed * time.delta_seconds();
    delta_roll *= camera_settings.roll_speed * time.delta_seconds();
    delta_yaw *= camera_settings.yaw_speed * time.delta_seconds();

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let roll = roll + delta_roll;
    let yaw = yaw + delta_yaw;
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    transform.translation = Vec3::ZERO - transform.forward() * camera_settings.orbit_distance;

    dbg!(transform.translation);
}
