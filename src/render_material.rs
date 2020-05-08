use amethyst::assets::{Asset, Handle};
use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::renderer::mtl::TextureOffset;
// use amethyst::renderer::mtl::{
//     Material as AmethystMaterial, MaterialDefaults as AmethystMaterialDefaults, StaticTextureSet,
// };
// use amethyst::renderer::pod;
// use amethyst::renderer::pod::Material as PodMaterial;
// use amethyst::renderer::submodules::{MaterialId, MaterialSub};
use amethyst::renderer::types::Texture as AmethystTexture;
use glsl_layout::*;

// region - MaterialComposition

pub struct MaterialComposition {
    pub components: Vec<Handle<Material>>,
}

impl Component for MaterialComposition {
    type Storage = DenseVecStorage<Self>;
}

// endregion

// /// A physically based Material with metallic workflow, fully utilized in PBR render pass.
// #[derive(Debug, Clone, PartialEq)]
// pub struct Material {
//     /// Alpha cutoff: the value at which we do not draw the pixel
//     pub alpha_cutoff: f32,
//     /// Diffuse map.
//     pub albedo: Handle<Texture>,
//     /// Emission map.
//     pub emission: Handle<Texture>,
//     /// Normal map.
//     pub normal: Handle<Texture>,
//     /// Metallic-roughness map. (B channel metallic, G channel roughness)
//     pub metallic_roughness: Handle<Texture>,
//     /// Ambient occlusion map.
//     pub ambient_occlusion: Handle<Texture>,
//     /// Cavity map.
//     pub cavity: Handle<Texture>,
//     /// Texture offset
//     pub uv_offset: TextureOffset,
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub diffuse: Handle<AmethystTexture>,
    pub uv_offset: TextureOffset,
}

amethyst::assets::register_format_type!(Material);
impl Asset for Material {
    const NAME: &'static str = "custom:Material";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

/// A resource providing default textures for `Material`.
/// These will be be used by the renderer in case a texture
/// handle points to a texture which is not loaded already.
/// Additionally, you can use it to fill up the fields of
/// `Material` you don't want to specify.
#[derive(Debug, Clone)]
pub struct MaterialDefaults(pub Material);

// region - Shader Material

/// Material Uniform
/// ```glsl,ignore
/// uniform Material {
///    UvOffset uv_offset;
///    float alpha_cutoff;
/// };
/// ```
#[derive(Clone, Copy, Debug, AsStd140)]
#[repr(C, align(16))]
pub struct ShaderMaterial {
    /// UV offset of material
    pub uv_offset: amethyst::renderer::pod::TextureOffset,
    // /// Material alpha cutoff
    // pub alpha_cutoff: float,
}

impl ShaderMaterial {
    /// Helper function from amethyst_rendy 'proper' type to POD type.
    pub fn from_material(mat: &Material) -> Self {
        ShaderMaterial {
            uv_offset: amethyst::renderer::pod::TextureOffset::from_offset(&mat.uv_offset),
            // alpha_cutoff: mat.alpha_cutoff,
        }
    }
}

// endregion

// region - ITextureSet

/// Trait providing generic access to a collection of texture handles
pub trait ITextureSet<'a>:
    Clone + Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static
{
    /// Iterator type to access this texture sets handles
    type Iter: Iterator<Item = &'a Handle<AmethystTexture>>;

    /// Returns an iterator to the textures associated with a given material.
    fn textures(mat: &'a Material) -> Self::Iter;

    /// ALWAYS RETURNS 1
    fn len() -> usize {
        1
    }
}

/// Type alias for a tuple collection of a complete PBR texture set.
pub type FullTextureSet = (
    TexDiffuse,
    // TexEmission,
    // TexNormal,
    // TexMetallicRoughness,
    // TexAmbientOcclusion,
    // TexCavity,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TexDiffuse {}

impl<'a> ITextureSet<'a> for TexDiffuse {
    type Iter = std::iter::Once<&'a Handle<AmethystTexture>>;
    #[inline(always)]
    fn textures(mat: &'a Material) -> Self::Iter {
        std::iter::once(&mat.diffuse)
    }
}

// macro_rules! impl_texture {
//     ($name:ident, $prop:ident) => {
//         #[doc = "Macro Generated Texture Type"]
//         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//         pub struct $name;
//         impl<'a> ITextureSet<'a> for $name {
//             type Iter = std::iter::Once<&'a Handle<AmethystTexture>>;
//             #[inline(always)]
//             fn textures(mat: &'a Material) -> Self::Iter {
//                 std::iter::once(&mat.$prop)
//             }
//         }
//     };
// }

// impl_texture!(TexDiffuse, diffuse);

macro_rules! recursive_iter {
    (@value $first:expr, $($rest:expr),*) => { $first.chain(recursive_iter!(@value $($rest),*)) };
    (@value $last:expr) => { $last };
    (@type $first:ty, $($rest:ty),*) => { std::iter::Chain<$first, recursive_iter!(@type $($rest),*)> };
    (@type $last:ty) => { $last };
}

macro_rules! impl_texture_set_tuple {
    ($($from:ident),*) => {
        impl<'a, $($from,)*> ITextureSet<'a> for ($($from),*,)
        where
            $($from: ITextureSet<'a>),*,
        {
            type Iter = recursive_iter!(@type $($from::Iter),*);
            #[inline(always)]
            fn textures(mat: &'a Material) -> Self::Iter {
                recursive_iter!(@value $($from::textures(mat)),*)
            }
            fn len() -> usize {
                $($from::len() + )* 0
            }
        }
    }
}

impl_texture_set_tuple!(A);

// endregion
