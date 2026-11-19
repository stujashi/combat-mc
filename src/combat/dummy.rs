use std::f32::consts::FRAC_PI_2;
use std::fs;

use bevy::prelude::*;
use serde::Deserialize;

use super::health::{Health, Hurtbox};

/// Training dummy balance and hitbox values loaded from `assets/config/dummy.ron`.
#[derive(Resource, Deserialize, Debug, Clone)]
pub struct DummyConfig {
    pub max_health: f32,
    pub respawn_secs: f32,
    pub hit_flash_secs: f32,
    pub half_width: f32,
    pub height: f32,
}

impl DummyConfig {
    pub fn load(path: &str) -> Self {
        let contents = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("Failed to read dummy config at '{path}': {err}"));

        ron::from_str(&contents)
            .unwrap_or_else(|err| panic!("Failed to parse dummy config at '{path}': {err}"))
    }
}

/// Marker component for the training dummy root entity.
#[derive(Component)]
pub struct TrainingDummy;

/// Marker component for the dummy's visual body (tilts back on hit or when defeated).
#[derive(Component)]
pub struct DummyBody;

/// Marker component for the green fill bar above the dummy's head.
#[derive(Component)]
pub struct DummyHealthBarFill;

/// Tracks visual hit-flash timing and holds the material handle to tint on impact.
#[derive(Component)]
pub struct HitFeedback {
    pub flash_remaining: f32,
    pub body_material: Handle<StandardMaterial>,
}

/// Tracks the countdown before a defeated dummy pops back up at full health.
#[derive(Component, Default)]
pub struct DummyRespawn {
    pub timer: f32,
}

const WOOD_COLOR: Color = Color::srgb(0.68, 0.50, 0.30);
const HIT_FLASH_COLOR: Color = Color::srgb(0.95, 0.22, 0.18);
const DEFEATED_COLOR: Color = Color::srgb(0.36, 0.36, 0.40);

/// Spawns a blocky training dummy standing in front of the player's starting position.
pub fn spawn_training_dummy(
    mut commands: Commands,
    config: Res<DummyConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_material = materials.add(StandardMaterial {
        base_color: WOOD_COLOR,
        perceptual_roughness: 0.85,
        ..default()
    });
    let base_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.30, 0.18),
        perceptual_roughness: 0.9,
        ..default()
    });
    let bar_bg_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.14),
        unlit: true,
        ..default()
    });
    let bar_fill_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.82, 0.32),
        unlit: true,
        ..default()
    });

    commands.spawn((
        TrainingDummy,
        Health::new(config.max_health),
        Hurtbox {
            center_offset: Vec3::new(0.0, config.height * 0.5, 0.0),
            half_extents: Vec3::new(config.half_width, config.height * 0.5, config.half_width),
        },
        HitFeedback {
            flash_remaining: 0.0,
            body_material: body_material.clone(),
        },
        DummyRespawn::default(),
        Transform::from_xyz(8.0, 4.0, 3.0),
        Visibility::default(),
        children![
            // Base stand stays planted on the ground.
            (
                Mesh3d(meshes.add(Cuboid::new(0.75, 0.10, 0.75))),
                MeshMaterial3d(base_material),
                Transform::from_xyz(0.0, 0.05, 0.0),
            ),
            // Visual dummy body rocks back on hits and tips over when defeated.
            (
                DummyBody,
                Transform::default(),
                Visibility::default(),
                children![
                    // Center post
                    (
                        Mesh3d(meshes.add(Cuboid::new(0.18, 0.95, 0.18))),
                        MeshMaterial3d(body_material.clone()),
                        Transform::from_xyz(0.0, 0.52, 0.0),
                    ),
                    // Torso
                    (
                        Mesh3d(meshes.add(Cuboid::new(0.62, 0.68, 0.32))),
                        MeshMaterial3d(body_material.clone()),
                        Transform::from_xyz(0.0, 1.08, 0.0),
                    ),
                    // Outstretched arms crossbar
                    (
                        Mesh3d(meshes.add(Cuboid::new(1.22, 0.18, 0.18))),
                        MeshMaterial3d(body_material.clone()),
                        Transform::from_xyz(0.0, 1.24, 0.0),
                    ),
                    // Head
                    (
                        Mesh3d(meshes.add(Cuboid::new(0.42, 0.42, 0.42))),
                        MeshMaterial3d(body_material),
                        Transform::from_xyz(0.0, 1.66, 0.0),
                    ),
                ],
            ),
            // Floating 3D health bar above the dummy's head.
            (
                Transform::from_xyz(0.0, 2.18, 0.0),
                Visibility::default(),
                children![
                    (
                        Mesh3d(meshes.add(Cuboid::new(1.06, 0.13, 0.04))),
                        MeshMaterial3d(bar_bg_material),
                    ),
                    (
                        DummyHealthBarFill,
                        Mesh3d(meshes.add(Cuboid::new(1.0, 0.09, 0.05))),
                        MeshMaterial3d(bar_fill_material),
                        Transform::from_xyz(0.0, 0.0, 0.005),
                    ),
                ],
            ),
        ],
    ));
}

/// Handles respawning the dummy after its health drops to zero (`FixedUpdate`).
pub fn step_dummy_respawn(
    time: Res<Time<Fixed>>,
    config: Res<DummyConfig>,
    mut dummy: Single<(&mut Health, &mut DummyRespawn), With<TrainingDummy>>,
) {
    let (ref mut health, ref mut respawn) = *dummy;
    if health.is_alive() {
        respawn.timer = 0.0;
        return;
    }

    respawn.timer += time.delta_secs();
    if respawn.timer >= config.respawn_secs {
        health.reset();
        respawn.timer = 0.0;
        info!("Training Dummy respawned ({:.0}/{:.0} HP)", health.current, health.max);
    }
}

/// Updates the dummy's hit-flash color, recoil tilt, and floating health bar (`Update`).
pub fn update_dummy_visuals(
    time: Res<Time>,
    config: Res<DummyConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut dummy: Single<(&Health, &mut HitFeedback), With<TrainingDummy>>,
    mut body_transform: Single<&mut Transform, (With<DummyBody>, Without<DummyHealthBarFill>)>,
    health_bar: Single<
        (&mut Transform, &mut Visibility),
        (With<DummyHealthBarFill>, Without<DummyBody>),
    >,
) {
    let (health, ref mut feedback) = *dummy;
    let dt = time.delta_secs();

    if feedback.flash_remaining > 0.0 {
        feedback.flash_remaining = (feedback.flash_remaining - dt).max(0.0);
    }

    // Update wood material tint.
    if let Some(mut material) = materials.get_mut(&feedback.body_material) {
        material.base_color = if !health.is_alive() {
            DEFEATED_COLOR
        } else if feedback.flash_remaining > 0.0 {
            HIT_FLASH_COLOR
        } else {
            WOOD_COLOR
        };
    }

    // Tilt back sharply during hit flash, or lie flat when defeated.
    let tilt_angle = if !health.is_alive() {
        -FRAC_PI_2
    } else if feedback.flash_remaining > 0.0 && config.hit_flash_secs > 0.0 {
        let strength = feedback.flash_remaining / config.hit_flash_secs;
        -0.30 * strength
    } else {
        0.0
    };
    body_transform.rotation = Quat::from_rotation_x(tilt_angle);

    // Scale and shift the floating health bar so it drains smoothly from right to left.
    let (mut bar_transform, mut bar_visibility) = health_bar.into_inner();
    let ratio = health.fraction();
    if ratio <= 0.0 {
        *bar_visibility = Visibility::Hidden;
    } else {
        *bar_visibility = Visibility::Visible;
        bar_transform.scale.x = ratio;
        bar_transform.translation.x = -0.5 * (1.0 - ratio);
    }
}
