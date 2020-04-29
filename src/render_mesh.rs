//! Module for mesh support.
use amethyst::assets::Asset;
use amethyst::assets::{
    AssetPrefab, AssetStorage, Format, Handle, Loader, PrefabData, ProgressCounter,
};
use amethyst::core::ecs::DenseVecStorage;
use amethyst::core::ecs::{Entity, Read, ReadExpect, WriteStorage};
use amethyst::error::Error;
use amethyst::renderer::{
    shape::{FromShape, ShapePrefab},
    types::Backend,
};
use serde::{Deserialize, Serialize};

use amethyst::renderer::rendy::{
    self as rendy,
    command::{EncoderCommon, Graphics, QueueId, RenderPassEncoder, Supports},
    factory::{BufferState, Factory},
    memory::{Data, Upload, Write},
    mesh::{AsVertex, Normal, Position, /*Tangent, */ TexCoord, VertexFormat},
    resource::{Buffer, BufferInfo, Escape},
    util::cast_cow,
};
use gfx_hal::adapter::PhysicalDevice;
use std::{borrow::Cow, mem::size_of};

// /// 'Obj' mesh format `Format` implementation.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// pub struct ObjFormat;

amethyst::assets::register_format_type!(MeshData);

// amethyst_assets::register_format!("OBJ", ObjFormat as MeshData);
// impl Format<MeshData> for ObjFormat {
//     fn name(&self) -> &'static str {
//         "OBJ"
//     }

//     fn import_simple(&self, bytes: Vec<u8>) -> Result<MeshData, Error> {
//         rendy::mesh::obj::load_from_obj(&bytes)
//             .map(|mut builders| {
//                 let mut iter = builders.drain(..);
//                 let builder = iter.next().unwrap();
//                 if iter.next().is_some() {
//                     log::warn!("OBJ file contains more than one object, only loading the first");
//                 }
//                 builder.0.into()
//             })
//             .map_err(|e| e.compat().into())
//     }
// }

// /// Internal mesh loading
// ///
// /// ### Type parameters:
// ///
// /// `V`: Vertex format to use for generated `Mesh`es, for example:
// ///     * `Vec<PosTex>`
// ///     * `Vec<PosNormTex>`
// ///     * `(Vec<Position>, Vec<Normal>)`
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(bound = "")]
// pub enum MeshPrefab<V> {
//     /// Load an asset Mesh from file
//     Asset(AssetPrefab<Mesh>),
//     /// Generate a Mesh from basic type
//     Shape(ShapePrefab<V>),
// }

// impl<'a, V> PrefabData<'a> for MeshPrefab<V>
// where
//     V: FromShape + Into<MeshBuilder<'static>>,
// {
//     type SystemData = (
//         ReadExpect<'a, Loader>,
//         WriteStorage<'a, Handle<Mesh>>,
//         Read<'a, AssetStorage<Mesh>>,
//     );
//     type Result = ();

//     fn add_to_entity(
//         &self,
//         entity: Entity,
//         system_data: &mut Self::SystemData,
//         entities: &[Entity],
//         children: &[Entity],
//     ) -> Result<(), Error> {
//         match self {
//             MeshPrefab::Asset(m) => {
//                 m.add_to_entity(entity, system_data, entities, children)?;
//             }
//             MeshPrefab::Shape(s) => {
//                 s.add_to_entity(entity, system_data, entities, children)?;
//             }
//         }
//         Ok(())
//     }

//     fn load_sub_assets(
//         &mut self,
//         progress: &mut ProgressCounter,
//         system_data: &mut Self::SystemData,
//     ) -> Result<bool, Error> {
//         Ok(match self {
//             MeshPrefab::Asset(m) => m.load_sub_assets(progress, system_data)?,
//             MeshPrefab::Shape(s) => s.load_sub_assets(progress, system_data)?,
//         })
//     }
// }

// region - Backend

// /// Extension of the rendy Backend trait.
// pub trait Backend: rendy::hal::Backend {
//     /// Unwrap a Backend to a rendy `Mesh`
//     fn unwrap_mesh(mesh: &Mesh) -> Option<&BackendMesh<Self>>;
//     /// Unwrap a Backend to a rendy `Texture`
//     fn wrap_mesh(mesh: BackendMesh<Self>) -> Mesh;

//     // /// Wrap a rendy `Texture` to its Backend generic.
//     // fn wrap_texture(texture: rendy::texture::Texture<Self>) -> Texture;
//     // fn unwrap_texture(texture: &Texture) -> Option<&rendy::texture::Texture<Self>>;
//     // /// Wrap a rendy `Mesh` to its Backend generic.
// }

pub trait ExtendedBackend: Backend {

    fn unwrap_custom_mesh(mesh: &Mesh) -> Option<&BackendMesh<Self>>;

    fn wrap_custom_mesh(mesh: BackendMesh<Self>) -> Mesh;

}

// macro_rules! impl_backends {
//     ($($variant:ident, $feature:literal, $backend:ty;)*) => {

//         // impl_single_default!($([$feature, $backend]),*);
//         // static_assertions::assert_cfg!(
//         //     any($(feature = $feature),*),
//         //     concat!("You must specify at least one graphical backend feature: ", stringify!($($feature),* "See the wiki article https://book.amethyst.rs/stable/appendices/c_feature_gates.html#graphics-features for more details."))
//         // );

//         // /// Backend wrapper.
//         // #[derive(Debug)]
//         // pub enum BackendVariant {
//         //     $(
//         //         #[cfg(feature = $feature)]
//         //         #[doc = "Backend Variant"]
//         //         $variant,
//         //     )*
//         // }

//         /// Mesh wrapper.
//         #[derive(Debug)]
//         pub enum Mesh {
//             $(
//                 #[cfg(feature = $feature)]
//                 #[doc = "Mesh Variant"]
//                 $variant(BackendMesh<$backend>),
//             )*
//         }

//         // /// Texture wrapper.
//         // #[derive(Debug)]
//         // pub enum Texture {
//         //     $(
//         //         #[cfg(feature = $feature)]
//         //         #[doc = "Texture Variant"]
//         //         $variant(rendy::texture::Texture<$backend>),
//         //     )*
//         // }

//         $(
//             #[cfg(feature = $feature)]
//             impl ExtendedBackend for $backend {
//                 #[inline]
//                 #[allow(irrefutable_let_patterns)]
//                 fn unwrap_custom_mesh(mesh: &Mesh) -> Option<&BackendMesh<Self>> {
//                     if let Mesh::$variant(inner) = mesh {
//                         Some(inner)
//                     } else {
//                         None
//                     }
//                 }
//                 #[inline]
//                 fn wrap_custom_mesh(mesh: BackendMesh<Self>) -> Mesh {
//                     Mesh::$variant(mesh)
//                 }
//                 // #[inline]
//                 // #[allow(irrefutable_let_patterns)]
//                 // fn unwrap_texture(texture: &Texture) -> Option<&rendy::texture::Texture<Self>> {
//                 //     if let Texture::$variant(inner) = texture {
//                 //         Some(inner)
//                 //     } else {
//                 //         None
//                 //     }
//                 // }
//                 // #[inline]
//                 // fn wrap_texture(texture: rendy::texture::Texture<Self>) -> Texture {
//                 //     Texture::$variant(texture)
//                 // }
//             }
//         )*
//     };
// }

// Create `DefaultExtendedBackend` type alias only when exactly one backend is selected.
// macro_rules! impl_single_default {
    // ( $([$feature:literal, $backend:ty]),* ) => {
    //     impl_single_default!(@ (), ($([$feature, $backend])*));
    // };
    // (@ ($($prev:literal)*), ([$cur:literal, $backend:ty]) ) => {
    //     #[cfg(all( feature = $cur, not(any($(feature = $prev),*)) ))]
    //     #[doc = "Default backend"]
    //     pub type DefaultExtendedBackend = $backend;
    // };
    // (@ ($($prev:literal)*), ([$cur:literal, $backend:ty] $([$nf:literal, $nb:ty])*) ) => {
    //     #[cfg(all( feature = $cur, not(any($(feature = $prev,)* $(feature = $nf),*)) ))]
    //     #[doc = "Default backend"]
    //     pub type DefaultExtendedBackend = $backend;

    //     impl_single_default!(@ ($($prev)* $cur), ($([$nf, $nb])*) );
    // };
// }

// impl_backends!(
//     // DirectX 12 is currently disabled because of incomplete gfx-hal support for it.
//     // It will be re-enabled when it actually works.
//     // Dx12, "dx12", rendy::dx12::Backend;
//     Metal, "metal", rendy::metal::Backend;
//     Vulkan, "vulkan", rendy::vulkan::Backend;
//     Empty, "empty", rendy::empty::Backend;
// );

pub type DefaultExtendedBackend = rendy::metal::Backend;

/// Mesh wrapper.
#[derive(Debug)]
pub enum Mesh {
    Metal(BackendMesh<rendy::metal::Backend>),
}

impl ExtendedBackend for rendy::metal::Backend {
    #[inline]
    #[allow(irrefutable_let_patterns)]
    fn unwrap_custom_mesh(mesh: &Mesh) -> Option<&BackendMesh<Self>> {
        if let Mesh::Metal(inner) = mesh {
            Some(inner)
        } else {
            None
        }
    }
    #[inline]
    fn wrap_custom_mesh(mesh: BackendMesh<Self>) -> Mesh {
        Mesh::Metal(mesh)
    }
}

// endregion

// region - Assets

impl Asset for Mesh {
    const NAME: &'static str = "Mesh";
    type Data = MeshData;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

// impl Asset for Texture {
//     const NAME: &'static str = "Mesh";
//     type Data = TextureData;
//     type HandleStorage = DenseVecStorage<Handle<Self>>;
// }

// endregion

// region - Data

/// Newtype for MeshBuilder prefab usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshData(#[serde(deserialize_with = "deserialize_data")] pub MeshBuilder<'static>);

// /// Newtype for TextureBuilder prefab usage.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TextureData(pub rendy::texture::TextureBuilder<'static>);

impl From<MeshBuilder<'static>> for MeshData {
    fn from(builder: MeshBuilder<'static>) -> Self {
        Self(builder)
    }
}

// impl From<rendy::texture::TextureBuilder<'static>> for TextureData {
//     fn from(builder: rendy::texture::TextureBuilder<'static>) -> Self {
//         Self(builder)
//     }
// }

fn deserialize_data<'de, D>(deserializer: D) -> Result<MeshBuilder<'static>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(MeshBuilder::deserialize(deserializer)?.into_owned())
}

// endregion

// region - Mesh builder

/// Vertex buffer with it's format
#[derive(Debug)]
pub struct VertexBufferLayout {
    offset: u64,
    format: VertexFormat,
}

/// Index buffer with it's type
#[derive(Debug)]
pub struct IndexBuffer<B: gfx_hal::Backend> {
    buffer: Escape<Buffer<B>>,
    index_type: gfx_hal::IndexType,
}

/// Abstracts over two types of indices and their absence.
#[derive(Debug)]
pub enum Indices<'a> {
    // /// No indices.
    // None,

    // /// `u16` per index.
    // U16(Cow<'a, [u16]>),
    /// `u32` per index.
    U32(Cow<'a, [u32]>),
}

// impl From<Vec<u16>> for Indices<'static> {
//     fn from(vec: Vec<u16>) -> Self {
//         Indices::U16(vec.into())
//     }
// }

// impl<'a> From<&'a [u16]> for Indices<'a> {
//     fn from(slice: &'a [u16]) -> Self {
//         Indices::U16(slice.into())
//     }
// }

// impl<'a> From<Cow<'a, [u16]>> for Indices<'a> {
//     fn from(cow: Cow<'a, [u16]>) -> Self {
//         Indices::U16(cow)
//     }
// }

impl From<Vec<u32>> for Indices<'static> {
    fn from(vec: Vec<u32>) -> Self {
        Indices::U32(vec.into())
    }
}

impl<'a> From<&'a [u32]> for Indices<'a> {
    fn from(slice: &'a [u32]) -> Self {
        Indices::U32(slice.into())
    }
}

impl<'a> From<Cow<'a, [u32]>> for Indices<'a> {
    fn from(cow: Cow<'a, [u32]>) -> Self {
        Indices::U32(cow)
    }
}

/// Generics-free mesh builder.
#[derive(Clone, Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeshBuilder<'a> {
    // #[cfg_attr(feature = "serde", serde(borrow))]
    #[serde(borrow)]
    vertices: smallvec::SmallVec<[RawVertices<'a>; 16]>,
    // #[cfg_attr(feature = "serde", serde(borrow))]
    #[serde(borrow)]
    indices: Option<RawIndices<'a>>,
    prim: gfx_hal::Primitive,
}

#[derive(Clone, Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(serde::Serialize, serde::Deserialize)]
struct RawVertices<'a> {
    // #[cfg_attr(feature = "serde", serde(with = "serde_bytes", borrow))]
    // #[serde(with = "serde_bytes", borrow)]
    #[serde(borrow)]
    vertices: Cow<'a, [u8]>,
    format: VertexFormat,
}

#[derive(Clone, Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(serde::Serialize, serde::Deserialize)]
struct RawIndices<'a> {
    // #[cfg_attr(feature = "serde", serde(with = "serde_bytes", borrow))]
    #[serde(borrow)]
    indices: Cow<'a, [u8]>,
    index_type: gfx_hal::IndexType,
}

fn index_stride(index_type: gfx_hal::IndexType) -> usize {
    match index_type {
        gfx_hal::IndexType::U16 => size_of::<u16>(),
        gfx_hal::IndexType::U32 => size_of::<u32>(),
    }
}

impl<'a> MeshBuilder<'a> {
    /// Create empty builder.
    pub fn new() -> Self {
        MeshBuilder {
            vertices: smallvec::SmallVec::new(),
            indices: None,
            prim: gfx_hal::Primitive::TriangleList,
        }
    }

    /// Convert builder into fully owned type. This forces internal vertex and index buffers
    /// to be cloned, which allows borrowed source buffers to be released.
    pub fn into_owned(self) -> MeshBuilder<'static> {
        MeshBuilder {
            vertices: self
                .vertices
                .into_iter()
                .map(|v| RawVertices {
                    vertices: Cow::Owned(v.vertices.into_owned()),
                    format: v.format,
                })
                .collect(),
            indices: self.indices.map(|i| RawIndices {
                indices: Cow::Owned(i.indices.into_owned()),
                index_type: i.index_type,
            }),
            prim: self.prim,
        }
    }

    /// Set indices buffer to the `MeshBuilder`
    pub fn with_indices<I>(mut self, indices: I) -> Self
    where
        I: Into<Indices<'a>>,
    {
        self.set_indices(indices);
        self
    }

    /// Set indices buffer to the `MeshBuilder`
    pub fn set_indices<I>(&mut self, indices: I) -> &mut Self
    where
        I: Into<Indices<'a>>,
    {
        self.indices = match indices.into() {
            // Indices::None => None,
            // Indices::U16(i) => Some(RawIndices {
            //     indices: cast_cow(i),
            //     index_type: gfx_hal::IndexType::U16,
            // }),
            Indices::U32(i) => Some(RawIndices {
                indices: cast_cow(i),
                index_type: gfx_hal::IndexType::U32,
            }),
        };
        self
    }

    /// Add another vertices to the `MeshBuilder`
    pub fn with_vertices<V, D>(mut self, vertices: D) -> Self
    where
        V: AsVertex + 'a,
        D: Into<Cow<'a, [V]>>,
    {
        self.add_vertices(vertices);
        self
    }

    /// Add another vertices to the `MeshBuilder`
    pub fn add_vertices<V, D>(&mut self, vertices: D) -> &mut Self
    where
        V: AsVertex + 'a,
        D: Into<Cow<'a, [V]>>,
    {
        self.vertices.push(RawVertices {
            vertices: cast_cow(vertices.into()),
            format: V::vertex(),
        });
        self
    }

    /// Sets the primitive type of the mesh.
    ///
    /// By default, meshes are constructed as triangle lists.
    pub fn with_prim_type(mut self, prim: gfx_hal::Primitive) -> Self {
        self.prim = prim;
        self
    }

    /// Sets the primitive type of the mesh.
    ///
    /// By default, meshes are constructed as triangle lists.
    pub fn set_prim_type(&mut self, prim: gfx_hal::Primitive) -> &mut Self {
        self.prim = prim;
        self
    }

    /// Builds and returns the new mesh.
    ///
    /// A mesh expects all vertex buffers to have the same number of elements.
    /// If those are not equal, the length of smallest vertex buffer is selected,
    /// effectively discaring extra data from larger buffers.
    ///
    /// Note that contents of index buffer is not validated.
    pub fn build<B>(
        &self, queue: QueueId, factory: &Factory<B>,
    ) -> Result<BackendMesh<B>, failure::Error>
    where
        B: gfx_hal::Backend,
    {
        let align = factory.physical().limits().non_coherent_atom_size;
        let mut len = self
            .vertices
            .iter()
            .map(|v| v.vertices.len() as u32 / v.format.stride)
            .min()
            .unwrap_or(0);

        let buffer_size = self
            .vertices
            .iter()
            .map(|v| (v.format.stride * len) as usize)
            .sum();

        let aligned_size = align_by(align, buffer_size) as u64;

        let mut staging = factory.create_buffer(
            BufferInfo {
                size: aligned_size,
                usage: gfx_hal::buffer::Usage::TRANSFER_SRC,
            },
            Upload,
        )?;

        let mut buffer = factory.create_buffer(
            BufferInfo {
                size: buffer_size as _,
                usage: gfx_hal::buffer::Usage::VERTEX | gfx_hal::buffer::Usage::TRANSFER_DST,
            },
            Data,
        )?;

        let mut mapped = staging.map(factory, 0..aligned_size)?;
        let mut writer = unsafe { mapped.write(factory, 0..aligned_size)? };
        let staging_slice = unsafe { writer.slice() };

        let mut offset = 0usize;
        let mut vertex_layouts: Vec<_> = self
            .vertices
            .iter()
            .map(|RawVertices { vertices, format }| {
                let size = (format.stride * len) as usize;
                staging_slice[offset..offset + size].copy_from_slice(&vertices[0..size]);
                let this_offset = offset as u64;
                offset += size;
                Ok(VertexBufferLayout {
                    offset: this_offset,
                    format: format.clone(),
                })
            })
            .collect::<Result<_, failure::Error>>()?;

        drop(staging_slice);
        drop(writer);
        drop(mapped);

        vertex_layouts.sort_unstable_by(|a, b| a.format.cmp(&b.format));

        let index_buffer = match self.indices {
            None => None,
            Some(RawIndices {
                ref indices,
                index_type,
            }) => {
                len = (indices.len() / index_stride(index_type)) as u32;
                let mut buffer = factory.create_buffer(
                    BufferInfo {
                        size: indices.len() as _,
                        usage: gfx_hal::buffer::Usage::INDEX | gfx_hal::buffer::Usage::TRANSFER_DST,
                    },
                    Data,
                )?;
                unsafe {
                    // New buffer can't be touched by device yet.
                    factory.upload_buffer(
                        &mut buffer,
                        0,
                        &indices,
                        None,
                        BufferState::new(queue)
                            .with_access(gfx_hal::buffer::Access::INDEX_BUFFER_READ)
                            .with_stage(gfx_hal::pso::PipelineStage::VERTEX_INPUT),
                    )?;
                }

                Some(IndexBuffer { buffer, index_type })
            }
        };

        unsafe {
            factory.upload_from_staging_buffer(
                &mut buffer,
                0,
                staging,
                None,
                BufferState::new(queue)
                    .with_access(gfx_hal::buffer::Access::VERTEX_BUFFER_READ)
                    .with_stage(gfx_hal::pso::PipelineStage::VERTEX_INPUT),
            )?;
        }

        Ok(BackendMesh {
            vertex_layouts,
            index_buffer,
            vertex_buffer: buffer,
            prim: self.prim,
            len,
        })
    }
}

fn align_by(align: usize, value: usize) -> usize {
    ((value + align - 1) / align) * align
}


// region - BackendMesh

/// Single mesh is a collection of buffer ranges that provides available attributes.
/// Usually exactly one mesh is used per draw call.
#[derive(Debug)]
pub struct BackendMesh<B: gfx_hal::Backend> {
    vertex_buffer: Escape<Buffer<B>>,
    vertex_layouts: Vec<VertexBufferLayout>,
    index_buffer: Option<IndexBuffer<B>>,
    prim: gfx_hal::Primitive,
    len: u32,
}

impl<B> BackendMesh<B>
where
    B: gfx_hal::Backend,
{
    /// Build new mesh with `MeshBuilder`
    pub fn builder<'a>() -> MeshBuilder<'a> {
        MeshBuilder::new()
    }

    /// gfx_hal::Primitive type of the `Mesh`
    pub fn primitive(&self) -> gfx_hal::Primitive {
        self.prim
    }

    /// Returns the number of vertices that will be drawn
    /// in the mesh.  For a mesh with no index buffer,
    /// this is the same as the number of vertices, or for
    /// a mesh with indices, this is the same as the number
    /// of indices.
    pub fn len(&self) -> u32 {
        self.len
    }

    fn get_vertex_iter<'a>(
        &'a self, formats: &[VertexFormat],
    ) -> Result<impl IntoIterator<Item = (&'a B::Buffer, u64)>, Incompatible> {
        debug_assert!(is_slice_sorted(formats), "Formats: {:#?}", formats);
        debug_assert!(is_slice_sorted_by_key(&self.vertex_layouts, |l| &l.format));

        let mut vertex = smallvec::SmallVec::<[_; 16]>::new();

        let mut next = 0;
        for format in formats {
            if let Some(index) = find_compatible_buffer(&self.vertex_layouts[next..], format) {
                next += index;
                vertex.push(self.vertex_layouts[next].offset);
            } else {
                // Can't bind
                return Err(Incompatible {
                    not_found: format.clone(),
                    in_formats: self
                        .vertex_layouts
                        .iter()
                        .map(|l| l.format.clone())
                        .collect(),
                });
            }
        }

        let buffer = self.vertex_buffer.raw();
        Ok(vertex.into_iter().map(move |offset| (buffer, offset)))
    }

    /// Bind buffers to specified attribute locations.
    pub fn bind<C>(
        &self, first_binding: u32, formats: &[VertexFormat], encoder: &mut EncoderCommon<'_, B, C>,
    ) -> Result<u32, Incompatible>
    where
        C: Supports<Graphics>,
    {
        let vertex_iter = self.get_vertex_iter(formats)?;
        match self.index_buffer.as_ref() {
            Some(index_buffer) => unsafe {
                encoder.bind_index_buffer(index_buffer.buffer.raw(), 0, index_buffer.index_type);
                encoder.bind_vertex_buffers(first_binding, vertex_iter);
            },
            None => unsafe {
                encoder.bind_vertex_buffers(first_binding, vertex_iter);
            },
        }

        Ok(self.len)
    }

    /// Bind buffers to specified attribute locations and issue draw calls with given instance range.
    pub fn bind_and_draw(
        &self, first_binding: u32, formats: &[VertexFormat], instance_range: std::ops::Range<u32>,
        encoder: &mut RenderPassEncoder<'_, B>,
    ) -> Result<u32, Incompatible> {
        let vertex_iter = self.get_vertex_iter(formats)?;
        unsafe {
            match self.index_buffer.as_ref() {
                Some(index_buffer) => {
                    encoder.bind_index_buffer(
                        index_buffer.buffer.raw(),
                        0,
                        index_buffer.index_type,
                    );
                    encoder.bind_vertex_buffers(first_binding, vertex_iter);
                    encoder.draw_indexed(0..self.len, 0, instance_range);
                }
                None => {
                    encoder.bind_vertex_buffers(first_binding, vertex_iter);
                    encoder.draw(0..self.len, instance_range);
                }
            }
        }

        Ok(self.len)
    }
}

// endregion

/// failure::Error type returned by `Mesh::bind` in case of mesh's vertex buffers are incompatible with requested vertex formats.
#[derive(failure::Fail, Clone, Debug)]
#[fail(
    display = "Vertex format {:?} is not compatible with any of {:?}.",
    not_found, in_formats
)]
pub struct Incompatible {
    /// Format that was queried but was not found
    pub not_found: VertexFormat,
    /// List of formats that were available at query time
    pub in_formats: Vec<VertexFormat>,
}

/// Helper function to find buffer with compatible format.
fn find_compatible_buffer(
    vertex_layouts: &[VertexBufferLayout], format: &VertexFormat,
) -> Option<usize> {
    debug_assert!(is_slice_sorted(&*format.attributes));
    for (i, layout) in vertex_layouts.iter().enumerate() {
        debug_assert!(is_slice_sorted(&*layout.format.attributes));
        if is_compatible(&layout.format, format) {
            return Some(i);
        }
    }
    None
}

/// Check is vertex format `left` is compatible with `right`.
/// `left` must have same `stride` and contain all attributes from `right`.
fn is_compatible(left: &VertexFormat, right: &VertexFormat) -> bool {
    if left.stride != right.stride {
        return false;
    }

    // Don't start searching from index 0 because attributes are sorted
    let mut skip = 0;
    right.attributes.iter().all(|r| {
        left.attributes[skip..]
            .iter()
            .position(|l| l == r)
            .map_or(false, |p| {
                skip += p;
                true
            })
    })
}

/// Chech if slice o f ordered values is sorted.
fn is_slice_sorted<T: Ord>(slice: &[T]) -> bool {
    is_slice_sorted_by_key(slice, |i| i)
}

/// Check if slice is sorted using ordered key and key extractor
fn is_slice_sorted_by_key<'a, T, K: Ord>(slice: &'a [T], f: impl Fn(&'a T) -> K) -> bool {
    if let Some((first, slice)) = slice.split_first() {
        let mut cmp = f(first);
        for item in slice {
            let item = f(item);
            if cmp > item {
                return false;
            }
            cmp = item;
        }
    }
    true
}

impl<'a, A> From<Vec<A>> for MeshBuilder<'a>
where
    A: AsVertex + 'a,
{
    fn from(vertices: Vec<A>) -> Self {
        MeshBuilder::new().with_vertices(vertices)
    }
}

macro_rules! impl_builder_from_vec {
    ($($from:ident),*) => {
        impl<'a, $($from,)*> From<($(Vec<$from>,)*)> for MeshBuilder<'a>
        where
            $($from: AsVertex + 'a,)*
        {
            fn from(vertices: ($(Vec<$from>,)*)) -> Self {
                #[allow(unused_mut)]
                let mut builder = MeshBuilder::new();
                #[allow(non_snake_case)]
                let ($($from,)*) = vertices;
                $(builder.add_vertices($from);)*
                builder
            }
        }

        impl_builder_from_vec!(@ $($from),*);
    };
    (@) => {};
    (@ $head:ident $(,$tail:ident)*) => {
        impl_builder_from_vec!($($tail),*);
    };
}

impl_builder_from_vec!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

// endregion