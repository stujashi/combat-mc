/// Every type of voxel block that can exist in the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    Air,
    Grass,
    Dirt,
    Stone,
}

impl Block {
    /// Returns true if this block is solid (rendered and blocks movement).
    pub fn is_solid(self) -> bool {
        self != Block::Air
    }
}
