use crate::{Voxel, VoxelMap};

use bevy::{prelude::*, tasks::ComputeTaskPool};
use building_blocks::storage::{Compressible, FastLz4};

#[derive(Clone, Copy)]
pub struct ChunkCacheConfig {
    // These constants should be correlated with the size of a chunk.
    pub max_cached_chunks: usize,
    pub max_chunks_compressed_per_frame_per_thread: usize,
}

impl Default for ChunkCacheConfig {
    fn default() -> Self {
        Self {
            // Assuming 8192-byte chunks, we'll reserve a little under a gigabyte for the cache.
            max_cached_chunks: 100000,
            // Avoid high latency from compressing too many chunks in one frame. 8192-byte chunk
            // compression latency is around 0.01 ms.
            max_chunks_compressed_per_frame_per_thread: 50,
        }
    }
}

/// A system that evicts and compresses the least recently used voxel chunks when the cache gets too
/// big.
pub fn chunk_compressor_system<V>(
    cache_config: Res<ChunkCacheConfig>,
    pool: Res<ComputeTaskPool>,
    mut voxel_map: ResMut<VoxelMap<V>>,
) where
    V: Voxel,
{
    let num_cached = voxel_map.voxels.chunks.len_cached();
    if num_cached < cache_config.max_cached_chunks {
        return;
    }

    let overgrowth = num_cached - cache_config.max_cached_chunks;

    let num_to_compress =
        overgrowth.min(pool.thread_num() * cache_config.max_chunks_compressed_per_frame_per_thread);

    let mut chunks_to_compress = Vec::new();
    for _ in 0..num_to_compress {
        if let Some(key_and_chunk) = voxel_map.voxels.chunks.remove_lru() {
            chunks_to_compress.push(key_and_chunk);
        } else {
            break;
        }
    }

    let compressed_chunks = pool.scope(|s| {
        for (key, chunk) in chunks_to_compress.into_iter() {
            s.spawn(async move { (key, chunk.compress(FastLz4 { level: 10 })) });
        }
    });

    for (key, compressed_chunk) in compressed_chunks.into_iter() {
        voxel_map
            .voxels
            .chunks
            .insert_compressed(key, compressed_chunk);
    }
}
