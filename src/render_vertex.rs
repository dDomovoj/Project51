use amethyst::renderer::rendy::util::types::vertex::{
    AsVertex,
    VertexFormat, // AsAttribute, Normal, Position, TexCoord,
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

// /// Type for material idx of vertex.
// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)] //, AsStd140)]
// #[repr(C, align(4))]
// pub struct MaterialIdx(pub u32);
// impl<T> From<T> for MaterialIdx
// where
//     T: Into<u32>,
// {
//     fn from(from: T) -> Self {
//         MaterialIdx(from.into())
//     }
// }

// impl AsAttribute for MaterialIdx {
//     const NAME: &'static str = "mtl_idx";
//     const FORMAT: Format = Format::R32Uint;
// }

// /// Instance-rate joints offset
// /// ```glsl,ignore
// ///  uint joints_offset;
// /// ```
// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
// #[repr(C, align(4))]
// pub struct JointsOffset {
//     /// `u32` joints offset value
//     pub joints_offset: u32,
// }

// impl AsAttribute for JointsOffset {
//     const NAME: &'static str = "joints_offset";
//     const FORMAT: Format = Format::R32Uint;
// }

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

// /// CustomUniformArgs
// /// A Uniform we pass into the shader containing the current scale.
// /// Uniform in shader:
// /// layout(std140, set = 0, binding = 0) uniform CustomUniformArgs {
// ///    uniform float scale;
// /// };
// #[derive(Clone, Copy, Debug, AsStd140)]
// #[repr(C, align(4))]
// pub struct CustomUniformArgs {
//     /// The value each vertex is scaled by.
//     pub scale: float,
// }
