pub mod block;
pub mod chunk;
mod mesh;
pub mod voxel_world;

use bevy::prelude::*;

use chunk::CHUNK_SIZE;
use mesh::build_chunk_mesh;
use voxel_world::VoxelWorld;

/// Feature plugin responsible for the voxel world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // 3x3 grid of 16x16x16 chunks (48x48 blocks) with a 4-block-thick flat ground.
        app.insert_resource(VoxelWorld::new_flat_arena(1, 4))
            .add_systems(Startup, spawn_world_chunks);
    }
}

fn spawn_world_chunks(
    mut commands: Commands,
    world: Res<VoxelWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.9,
        ..default()
    });

    let chunk_size = CHUNK_SIZE as f32;

    for (chunk_coord, chunk) in world.iter_chunks() {
        let chunk_mesh = build_chunk_mesh(&world, chunk_coord, chunk);
        let world_pos = chunk_coord.as_vec3() * chunk_size;

        commands.spawn((
            Mesh3d(meshes.add(chunk_mesh)),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(world_pos),
        ));
    }

    // Sunlight angled down onto the arena.
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(36.0, 48.0, 24.0).looking_at(Vec3::new(8.0, 2.0, 8.0), Vec3::Y),
    ));
}
