use crate::{Voxel, VoxelMap};

use bevy::ecs::prelude::*;
use building_blocks::core::Point3i;

/// The resource that tracks which chunks recently became empty and should be removed. This enables
/// multiple methods of detecting empty chunks. Chunks will be removed at the end of the frame in
/// which they are marked as empty, but removal happens before the edit buffer is merged into the
/// `VoxelMap`, so writes from the same frame will not be removed.
#[derive(Default)]
pub struct EmptyChunks {
    chunks_to_remove: Vec<Point3i>,
}

impl EmptyChunks {
    /// Mark the chunk at `chunk_key` as "empty" and thus ready to be removed by the
    /// `empty_chunk_remover_system`.
    pub fn mark_for_removal(&mut self, chunk_key: Point3i) {
        self.chunks_to_remove.push(chunk_key);
    }
}

pub fn empty_chunk_remover_system<V>(
    mut empty_chunks: ResMut<EmptyChunks>,
    mut voxel_map: ResMut<VoxelMap<V>>,
) where
    V: Voxel,
{
    for chunk_key in empty_chunks.chunks_to_remove.drain(..) {
        voxel_map.voxels.storage_mut().remove(chunk_key);
    }
}
