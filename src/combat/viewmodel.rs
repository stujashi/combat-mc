use bevy::prelude::*;

use crate::player::{Player, PlayerCamera};

use super::state::{CombatState, Combatant};

/// Marker component for the first-person blocky sword attached to the camera.
#[derive(Component)]
pub struct SwordViewmodel;

/// Spawns a blocky voxel sword as a child of the first-person camera.
pub fn spawn_sword_viewmodel(
    mut commands: Commands,
    camera: Single<Entity, With<PlayerCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.38, 0.24, 0.13),
        perceptual_roughness: 0.9,
        ..default()
    });
    let guard = materials.add(StandardMaterial {
        base_color: Color::srgb(0.32, 0.34, 0.38),
        metallic: 0.6,
        perceptual_roughness: 0.45,
        ..default()
    });
    let blade = materials.add(StandardMaterial {
        base_color: Color::srgb(0.82, 0.85, 0.90),
        metallic: 0.75,
        perceptual_roughness: 0.3,
        ..default()
    });

    let (idle_pos, idle_rot) = idle_pose();

    commands.spawn((
        SwordViewmodel,
        ChildOf(*camera),
        Transform::from_translation(idle_pos).with_rotation(idle_rot),
        Visibility::default(),
        children![
            // Pommel
            (
                Mesh3d(meshes.add(Cuboid::new(0.065, 0.05, 0.065))),
                MeshMaterial3d(guard.clone()),
                Transform::from_xyz(0.0, -0.02, 0.0),
            ),
            // Handle / grip
            (
                Mesh3d(meshes.add(Cuboid::new(0.045, 0.18, 0.045))),
                MeshMaterial3d(wood),
                Transform::from_xyz(0.0, 0.08, 0.0),
            ),
            // Crossguard
            (
                Mesh3d(meshes.add(Cuboid::new(0.22, 0.045, 0.06))),
                MeshMaterial3d(guard),
                Transform::from_xyz(0.0, 0.18, 0.0),
            ),
            // Blade
            (
                Mesh3d(meshes.add(Cuboid::new(0.075, 0.56, 0.028))),
                MeshMaterial3d(blade.clone()),
                Transform::from_xyz(0.0, 0.48, 0.0),
            ),
            // Blade tip
            (
                Mesh3d(meshes.add(Cuboid::new(0.045, 0.08, 0.024))),
                MeshMaterial3d(blade),
                Transform::from_xyz(0.0, 0.80, 0.0),
            ),
        ],
    ));
}

/// Animates the first-person sword based on the player's current combat state and progress.
pub fn animate_sword_viewmodel(
    combatant: Single<&Combatant, With<Player>>,
    mut sword: Single<&mut Transform, With<SwordViewmodel>>,
) {
    let t = combatant.progress();
    let (idle_pos, idle_rot) = idle_pose();
    let (windup_pos, windup_rot) = windup_pose();
    let (slash_pos, slash_rot) = slash_end_pose();

    let (pos, rot) = match combatant.state {
        CombatState::Windup => (idle_pos.lerp(windup_pos, t), idle_rot.slerp(windup_rot, t)),
        CombatState::Active => (
            windup_pos.lerp(slash_pos, t),
            windup_rot.slerp(slash_rot, t),
        ),
        CombatState::Recovery => (slash_pos.lerp(idle_pos, t), slash_rot.slerp(idle_rot, t)),
        _ => (idle_pos, idle_rot),
    };

    sword.translation = pos;
    sword.rotation = rot;
}

fn idle_pose() -> (Vec3, Quat) {
    (
        Vec3::new(0.42, -0.34, -0.62),
        Quat::from_euler(EulerRot::YXZ, -0.25, -0.15, 0.12),
    )
}

fn windup_pose() -> (Vec3, Quat) {
    (
        Vec3::new(0.56, -0.08, -0.48),
        Quat::from_euler(EulerRot::YXZ, -0.55, 0.55, -0.45),
    )
}

fn slash_end_pose() -> (Vec3, Quat) {
    (
        Vec3::new(-0.32, -0.44, -0.78),
        Quat::from_euler(EulerRot::YXZ, 0.85, -0.85, 1.15),
    )
}
