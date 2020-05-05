use crate::render_material::Material::Compound;
use amethyst::renderer::mtl::Material as AmethystMaterial;
use amethyst::renderer::pod;
use amethyst::renderer::types::Texture as AmethystTexture;
use amethyst::assets::{Asset, Handle};
use amethyst::core::ecs::prelude::DenseVecStorage;

pub trait IMaterial {
    fn count(&self) -> usize;
}

pub enum Material {
    Simlpe(SimpleMaterial),
    Compound(CompoundMaterial),
}

impl IMaterial for Material {
    fn count(&self) -> usize {
        match self {
            Simple => 1,
            Compound(material) => material.components.len(),
        }
    }
}

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

// impl Asset for Material {
//     const NAME: &'static str = "renderer::Material";
//     type Data = Self;
//     type HandleStorage = DenseVecStorage<Handle<Self>>;
// }

impl Asset for Material {
    const NAME: &'static str = "Material";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

pub struct SimpleMaterial {
    pub albedo: Handle<AmethystTexture>
}

pub struct CompoundMaterial {
    components: Vec<SimpleMaterial>,
}

// fn create_default_mat<B: Backend>(world: &mut World) -> Material {
//     use crate::mtl::TextureOffset;

//     use amethyst_assets::Loader;

//     let loader = world.fetch::<Loader>();

//     let albedo = load_from_srgba(Srgba::new(0.5, 0.5, 0.5, 1.0));
//     let emission = load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
//     let normal = load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
//     let metallic_roughness = load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
//     let ambient_occlusion = load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
//     let cavity = load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

//     let tex_storage = world.fetch();

//     let albedo = loader.load_from_data(albedo.into(), (), &tex_storage);
//     let emission = loader.load_from_data(emission.into(), (), &tex_storage);
//     let normal = loader.load_from_data(normal.into(), (), &tex_storage);
//     let metallic_roughness = loader.load_from_data(metallic_roughness.into(), (), &tex_storage);
//     let ambient_occlusion = loader.load_from_data(ambient_occlusion.into(), (), &tex_storage);
//     let cavity = loader.load_from_data(cavity.into(), (), &tex_storage);

//     Material {
//         alpha_cutoff: 0.01,
//         albedo,
//         emission,
//         normal,
//         metallic_roughness,
//         ambient_occlusion,
//         cavity,
//         uv_offset: TextureOffset::default(),
//     }
// }
