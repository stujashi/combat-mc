use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use super::block::Block;
use super::chunk::{CHUNK_SIZE, Chunk};
use super::voxel_world::VoxelWorld;

struct Face {
    offset: [i32; 3],
    normal: [f32; 3],
    corners: [[f32; 3]; 4],
}

/// The 6 faces of a unit 1x1x1 voxel, with counter-clockwise vertex order
/// so every outward face renders cleanly with backface culling enabled.
const FACES: [Face; 6] = [
    // Top (+Y)
    Face {
        offset: [0, 1, 0],
        normal: [0.0, 1.0, 0.0],
        corners: [
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 0.0],
        ],
    },
    // Bottom (-Y)
    Face {
        offset: [0, -1, 0],
        normal: [0.0, -1.0, 0.0],
        corners: [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    },
    // Right (+X)
    Face {
        offset: [1, 0, 0],
        normal: [1.0, 0.0, 0.0],
        corners: [
            [1.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
        ],
    },
    // Left (-X)
    Face {
        offset: [-1, 0, 0],
        normal: [-1.0, 0.0, 0.0],
        corners: [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
        ],
    },
    // Front (+Z)
    Face {
        offset: [0, 0, 1],
        normal: [0.0, 0.0, 1.0],
        corners: [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
    },
    // Back (-Z)
    Face {
        offset: [0, 0, -1],
        normal: [0.0, 0.0, -1.0],
        corners: [
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
    },
];

/// Builds a single 3D mesh for a chunk, skipping hidden faces between touching solid blocks
/// (including across chunk boundaries).
pub fn build_chunk_mesh(world: &VoxelWorld, chunk_coord: IVec3, chunk: &Chunk) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let size = CHUNK_SIZE as i32;
    let origin = chunk_coord * size;

    for y in 0..size {
        for z in 0..size {
            for x in 0..size {
                let block = chunk.get(x, y, z);
                if !block.is_solid() {
                    continue;
                }

                let wx = origin.x + x;
                let wy = origin.y + y;
                let wz = origin.z + z;

                // Slight alternating shade so individual 1x1 blocks are easy to see.
                let shade = if (wx + wy + wz).rem_euclid(2) == 0 {
                    1.0
                } else {
                    0.94
                };

                for face in &FACES {
                    let neighbor = world.get_block(
                        wx + face.offset[0],
                        wy + face.offset[1],
                        wz + face.offset[2],
                    );

                    // Face culling: skip faces buried against another solid block.
                    if neighbor.is_solid() {
                        continue;
                    }

                    let base = positions.len() as u32;
                    let is_top = face.offset[1] == 1;
                    let color = block_color(block, is_top, shade);

                    for corner in face.corners {
                        positions.push([
                            x as f32 + corner[0],
                            y as f32 + corner[1],
                            z as f32 + corner[2],
                        ]);
                        normals.push(face.normal);
                        colors.push(color);
                    }

                    // Split the 4-corner quad into 2 triangles.
                    indices.extend_from_slice(&[
                        base,
                        base + 1,
                        base + 2,
                        base,
                        base + 2,
                        base + 3,
                    ]);
                }
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn block_color(block: Block, is_top: bool, shade: f32) -> [f32; 4] {
    let (r, g, b) = match block {
        Block::Air => (0.0, 0.0, 0.0),
        Block::Grass => {
            if is_top {
                (0.35, 0.68, 0.26)
            } else {
                (0.49, 0.34, 0.22)
            }
        }
        Block::Dirt => (0.49, 0.34, 0.22),
        Block::Stone => (0.50, 0.51, 0.53),
    };

    Color::srgb(r * shade, g * shade, b * shade)
        .to_linear()
        .to_f32_array()
}
