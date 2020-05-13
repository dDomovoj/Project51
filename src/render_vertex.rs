use amethyst::renderer::rendy::util::types::vertex::{
    AsVertex,
    VertexFormat, Normal, Position, TexCoord,
};

use std::fmt::Debug;

use glsl_layout::*;

use amethyst::core::{math::Matrix4, Transform};

use amethyst::renderer::rendy::mesh::Model;
// use gfx_hal::format::Format;
// use amethyst::renderer::pod::{*, Tint};
// use amethyst::renderer::resources::Tint as TintComponent;

// //! GPU POD data types.
// use crate::{
//     mtl,
//     resources::Tint as TintComponent,
//     sprite::{SpriteRender, SpriteSheet},
//     types::Texture,
// };
// use amethyst::assets::{AssetStorage, Handle};
use amethyst::core::math::convert;

// region - Vertex

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vertex {
    pub xyz: [f32; 3],
    pub norm: [f32; 3],
    pub uv: [f32; 2],
}

impl AsVertex for Vertex {
    fn vertex() -> VertexFormat {
        VertexFormat::new((Position::vertex(), Normal::vertex(), TexCoord::vertex()))
    }
}

// region - Shader

/// Material Instance-rate vertex arguments.
/// ```glsl,ignore
///  mat4 model;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, packed)]
pub struct VertexArgs {
    /// Instance-rate model matrix
    pub model: mat4,
    // /// Instance-rate `Tint`
    // pub tint: vec4,
}

impl AsVertex for VertexArgs {
    fn vertex() -> VertexFormat {
        // VertexFormat::new((Model::vertex(), Tint::vertex(), MaterialIdx::vertex()))
        VertexFormat::new(Model::vertex())
    }
}

impl VertexArgs {
    /// Populate `MaterialVertexArgs` from the supplied `Transform` and `TintComponent`
    #[inline]
    pub fn from_object_data(
        transform: &Transform,
        // tint: Option<&TintComponent>,
    ) -> Self {
        let model: [[f32; 4]; 4] = convert::<_, Matrix4<f32>>(*transform.global_matrix()).into();
        VertexArgs {
            model: model.into(),
            // tint: tint.map_or([1.0; 4].into(), |t| t.0.into_pod()),
        }
    }
}

// endregion
