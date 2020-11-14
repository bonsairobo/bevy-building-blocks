mod chunk_cache_flusher;
mod chunk_compressor;
mod edit_buffer;
mod editor;
mod plugin;

pub use chunk_compressor::ChunkCacheConfig;
pub use edit_buffer::{double_buffering_system, DirtyChunks, EditBuffer};
pub use editor::VoxelEditor;
pub use plugin::MapIoPlugin;

use crate::ThreadLocalResource;

use building_blocks::storage::LocalChunkCache3;

pub type ThreadLocalVoxelCache<V> = ThreadLocalResource<LocalChunkCache3<V>>;