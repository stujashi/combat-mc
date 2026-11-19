use super::block::Block;

/// Width, height, and depth of a single cubic chunk (16x16x16 blocks).
pub const CHUNK_SIZE: usize = 16;
const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// Pure voxel data for one 16x16x16 chunk (kept separate from rendering
/// so a headless multiplayer server can share the exact same world data).
pub struct Chunk {
    blocks: [Block; CHUNK_VOLUME],
}

impl Chunk {
    /// Creates a flat-ground chunk: stone at the bottom, one layer of dirt,
    /// and a top layer of grass up to `ground_height` blocks tall.
    pub fn new_flat(ground_height: usize) -> Self {
        let mut chunk = Self {
            blocks: [Block::Air; CHUNK_VOLUME],
        };

        let height = ground_height.min(CHUNK_SIZE);
        for y in 0..height {
            let block = if y + 1 == height {
                Block::Grass
            } else if y + 2 == height {
                Block::Dirt
            } else {
                Block::Stone
            };

            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    chunk.set(x, y, z, block);
                }
            }
        }

        chunk
    }

    /// Reads the block at `(x, y, z)`, returning `Block::Air` if out of bounds.
    pub fn get(&self, x: i32, y: i32, z: i32) -> Block {
        let size = CHUNK_SIZE as i32;
        if x < 0 || y < 0 || z < 0 || x >= size || y >= size || z >= size {
            return Block::Air;
        }
        self.blocks[Self::index(x as usize, y as usize, z as usize)]
    }

    /// Writes a block at `(x, y, z)` inside the chunk.
    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        let idx = Self::index(x, y, z);
        self.blocks[idx] = block;
    }

    fn index(x: usize, y: usize, z: usize) -> usize {
        x + z * CHUNK_SIZE + y * CHUNK_SIZE * CHUNK_SIZE
    }
}
