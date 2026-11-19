use std::collections::HashMap;

use bevy::prelude::{IVec3, Resource};

use super::block::Block;
use super::chunk::{CHUNK_SIZE, Chunk};

/// Holds all loaded chunks in the arena, keyed by chunk coordinate `(cx, cy, cz)`.
/// Kept as pure block data so collision and a future multiplayer server can query it directly.
#[derive(Resource, Default)]
pub struct VoxelWorld {
    chunks: HashMap<IVec3, Chunk>,
}

impl VoxelWorld {
    /// Builds a square flat arena of chunks from `-radius..=radius` on X and Z.
    /// For example, `radius = 1` creates a 3x3 chunk grid (48x48 blocks).
    pub fn new_flat_arena(radius_chunks: i32, ground_height: usize) -> Self {
        let mut chunks = HashMap::new();

        for cz in -radius_chunks..=radius_chunks {
            for cx in -radius_chunks..=radius_chunks {
                let coord = IVec3::new(cx, 0, cz);
                chunks.insert(coord, Chunk::new_flat(ground_height));
            }
        }

        Self { chunks }
    }

    /// Reads the block at any integer world coordinate `(x, y, z)`.
    /// Returns `Block::Air` if that chunk is not loaded.
    pub fn get_block(&self, world_x: i32, world_y: i32, world_z: i32) -> Block {
        let size = CHUNK_SIZE as i32;
        let chunk_coord = IVec3::new(
            world_x.div_euclid(size),
            world_y.div_euclid(size),
            world_z.div_euclid(size),
        );

        let Some(chunk) = self.chunks.get(&chunk_coord) else {
            return Block::Air;
        };

        let local_x = world_x.rem_euclid(size);
        let local_y = world_y.rem_euclid(size);
        let local_z = world_z.rem_euclid(size);
        chunk.get(local_x, local_y, local_z)
    }

    /// Iterates over every `(chunk_coord, &Chunk)` currently in the world.
    pub fn iter_chunks(&self) -> impl Iterator<Item = (IVec3, &Chunk)> {
        self.chunks.iter().map(|(&coord, chunk)| (coord, chunk))
    }
}
