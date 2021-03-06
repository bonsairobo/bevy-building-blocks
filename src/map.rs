use crate::{ThreadLocalResourceHandle, Voxel};

use building_blocks::prelude::*;

/// The global source of truth for voxels in the current map.
///
/// The map can contain any voxel type that implements the `Voxel` trait. Each voxel is expected to
/// store its type as some number. This number should correspond to an index into the `VoxelPalette`
/// which contains extra data about that type of voxel.
///
/// One convenience of this scheme is that you don't have to store this data for each voxel, but you
/// can use the `voxel_info_transform` method to construct a `TransformMap` which allows you to read
/// from the map as if each voxel stored its own `TypeInfo`.
///
/// # Constructing a Voxel Map
/// ```
/// use bevy_building_blocks::{bb::prelude::*, Voxel, VoxelMap, VoxelPalette, empty_compressible_chunk_map};
///
/// #[derive(Copy, Clone, Default)]
/// struct MyVoxel {
///     voxel_type: u8,
/// }
///
/// struct MyVoxelTypeInfo {
///     is_empty: bool,
/// }
///
/// impl Voxel for MyVoxel {
///     type TypeInfo = MyVoxelTypeInfo;
///
///     fn get_type_index(&self) -> usize {
///         self.voxel_type as usize
///     }
/// }
///
/// const CHUNK_SHAPE: Point3i = PointN([16; 3]);
///
/// let map = VoxelMap {
///     voxels: empty_compressible_chunk_map::<MyVoxel>(CHUNK_SHAPE),
///     palette: VoxelPalette {
///         infos: vec![
///             MyVoxelTypeInfo { is_empty: true },
///             MyVoxelTypeInfo { is_empty: false },
///         ],
///     },
/// };
/// ```
pub struct VoxelMap<V>
where
    V: Voxel,
{
    pub voxels: CompressibleChunkMap3<V>,
    pub palette: VoxelPalette<V::TypeInfo>,
}

impl<V> VoxelMap<V>
where
    V: Voxel,
{
    /// Returns a closure that transforms voxels into their type's corresponding info. This is
    /// intended to be used with a `TransformMap`.
    #[inline]
    pub fn voxel_info_transform<'a>(&'a self) -> impl Fn(V) -> &'a V::TypeInfo {
        move |v: V| self.palette.get_voxel_type_info(v)
    }

    pub fn reader<'a>(
        &'a self,
        cache: &'a ThreadLocalResourceHandle<LocalChunkCache3<V>>,
    ) -> ChunkMap3<V, (), CompressibleChunkStorageReader3<V>> {
        self.voxels
            .reader(cache.get_or_create_with(|| LocalChunkCache3::new()))
    }
}

#[derive(Clone, Default)]
pub struct VoxelPalette<I> {
    pub infos: Vec<I>,
}

impl<I> VoxelPalette<I> {
    pub fn get_voxel_type_info<V>(&self, voxel: V) -> &I
    where
        V: Voxel,
    {
        &self.infos[voxel.get_type_index()]
    }
}

pub fn chunk_map_builder<V>(chunk_shape: Point3i) -> ChunkMapBuilder3<V>
where
    V: Voxel,
{
    ChunkMapBuilder3 {
        chunk_shape,
        ambient_value: V::default(),
        default_chunk_metadata: (),
    }
}

pub fn empty_compressible_chunk_map<V>(chunk_shape: Point3i) -> CompressibleChunkMap3<V>
where
    V: Voxel,
{
    chunk_map_builder(chunk_shape)
        .build_with_write_storage(CompressibleChunkStorage3::new(Lz4 { level: 10 }))
}

pub fn empty_chunk_hash_map<V>(chunk_shape: Point3i) -> ChunkHashMap3<V>
where
    V: Voxel,
{
    chunk_map_builder(chunk_shape).build_with_hash_map_storage()
}

pub fn default_array<V>(extent: Extent3i) -> Array3<V>
where
    V: Voxel,
{
    Array3::fill(extent, V::default())
}
