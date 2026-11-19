pub mod config;
pub mod movement;

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use config::PlayerConfig;
use movement::{PlayerBody, PlayerInput, gather_player_input, step_player_movement};

const PLAYER_CONFIG_PATH: &str = "assets/config/player.ron";

/// Feature plugin responsible for the local player and first-person controller.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let config = PlayerConfig::load(PLAYER_CONFIG_PATH);

        app.insert_resource(config)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (toggle_cursor_lock, mouse_look, gather_player_input))
            .add_systems(FixedUpdate, step_player_movement);
    }
}

/// Marker component for the player's body entity (feet position + horizontal facing).
#[derive(Component)]
pub struct Player;

/// Stores horizontal (`yaw`) and vertical (`pitch`) look angles in radians.
#[derive(Component, Default)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
}

/// Marker component for the first-person camera attached to the player's head.
#[derive(Component)]
pub struct PlayerCamera;

fn spawn_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn((
        Player,
        PlayerLook::default(),
        PlayerInput::default(),
        PlayerBody::default(),
        Transform::from_xyz(8.0, 4.0, 8.0),
        children![(
            PlayerCamera,
            Camera3d::default(),
            Transform::from_xyz(0.0, config.eye_height, 0.0),
        )],
    ));
}

pub fn toggle_cursor_lock(
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}

fn mouse_look(
    cursor: Single<&CursorOptions, With<PrimaryWindow>>,
    motion: Res<AccumulatedMouseMotion>,
    config: Res<PlayerConfig>,
    mut player: Single<(&mut PlayerLook, &mut Transform), (With<Player>, Without<PlayerCamera>)>,
    mut camera_transform: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    if cursor.grab_mode == CursorGrabMode::None || motion.delta == Vec2::ZERO {
        return;
    }

    let (ref mut look, ref mut player_transform) = *player;
    let max_pitch = config.max_pitch_degrees.to_radians();

    look.yaw -= motion.delta.x * config.mouse_sensitivity;
    look.pitch = (look.pitch - motion.delta.y * config.mouse_sensitivity)
        .clamp(-max_pitch, max_pitch);

    // Body rotates left/right (yaw); head/camera tilts up/down (pitch).
    player_transform.rotation = Quat::from_rotation_y(look.yaw);
    camera_transform.rotation = Quat::from_rotation_x(look.pitch);
}
