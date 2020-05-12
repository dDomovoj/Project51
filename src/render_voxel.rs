// use std::path::PathBuf;
// use crate::bundles::camera_control_bundle::{MouseControlTag, CreativeMovementControlTag};

use amethyst::{
    assets::AssetLoaderSystemData, //, Handle, Loader},
    assets::{AssetStorage, Handle, Loader},
    ecs::{EntityBuilder, Read, WorldExt, Write},
    // controls::HideCursor,
    core::{
    //     transform::Transform,
        math::{Point3, Vector3},
    },
    // error::Error,
    // input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{
        // mtl::{Material as AmethystMaterial, MaterialDefaults},
        // palette::{Srgb, Srgba, LinSrgba},
        // rendy::mesh::{Normal, Position, TexCoord}, //, MeshBuilder},
        // transparent::Transparent,
        // types::{Mesh, MeshData},//, Texture},
        types::Texture,
        ImageFormat,
    },
    // window::ScreenDimensions,
    // winit::{MouseButton, VirtualKeyCode},
};

// use std::f32::consts::{FRAC_PI_8, FRAC_PI_6};

use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::render_cache::{MaterialCache, MeshCache, TextureCache};
use crate::render_material::{Material as RenderMaterial, MaterialComposition, MaterialDefaults};
use crate::render_mesh::{CompositeMesh, Indices, Mesh, MeshBuilder, MeshData, Vertex};
use crate::render_visibility::BoundingSphere;

use amethyst::ecs::shred::SystemData;

pub enum Material {
    Dirt,
    Grass,
    Crate,
}

pub struct Voxel {
    pub position: [i128; 3],
    pub material: Material,
}

impl Component for Voxel {
    type Storage = DenseVecStorage<Self>;
}

impl Voxel {
    fn texture_name(&self) -> &str {
        match &self.material {
            Material::Grass => "grass_block_side",
            Material::Crate => "crate",
            Material::Dirt => "dirt",
        }
    }

    pub fn create_entity<'a>(&self, world: &'a mut World) -> EntityBuilder<'a> {
        // let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| loader.load_from_data(block_mesh(), ()));
        // let mesh_element = {
        //     world.exec(|loader: AssetLoaderSystemData<MeshElement>| loader.load_from_data(block_mesh(), ()))
        // };
        let mesh_element = {
            MeshCache::item(0, world, |res: &mut World| {
                res.exec(|loader: AssetLoaderSystemData<Mesh>| loader.load_from_data(block_mesh(), ()))
            })
        };
        let mesh = CompositeMesh {
            elements: vec![mesh_element],
        };

        let texture = {
            TextureCache::item(0, world, |res: &mut World| {
                res.exec(|loader: AssetLoaderSystemData<Texture>| {
                    loader.load(
                        format!("texture/{}.png", self.texture_name()),
                        ImageFormat::default(),
                        (),
                    )
                })
            })
        };

        // let mat = world.exec(|loader: AssetLoaderSystemData<AmethystMaterial>|
        //     loader.load_from_data(AmethystMaterial {
        //         albedo: texture,
        //         ..default_mat.clone()
        //     }, ())
        // );
        let mat_elt = {
            MaterialCache::item(0, world, |res: &mut World| {
                let default_mat = res.read_resource::<MaterialDefaults>().0.clone();
                let mut materials_asset = <Write<'_, AssetStorage<RenderMaterial>>>::fetch(res);
                let data = RenderMaterial {
                    diffuse: texture,
                    ..default_mat.clone()
                };
                materials_asset.insert(data)
            })
        };
        let mat = MaterialComposition {
            components: vec![mat_elt],
        };

        let bounds = BoundingSphere {
            center: Vector3::new(0.5, 0.5, 0.5).into(),
            radius: 0.8705505633_f32
        };

        world.create_entity().with(mesh).with(mat)
            .with(bounds)
        // .with(Transparent::default())
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn block_mesh() -> MeshData {
    
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let vertices: Vec<Vertex> = vec!(
      // Face 1 (front)
      Vertex { xyz: [0.0, 0.0, 0.0], norm: [0.0, 0.0, -1.0], uv: [1.0, 1.0] }, /* bottom left */
      Vertex { xyz: [0.0, 1.0, 0.0], norm: [0.0, 0.0, -1.0], uv: [1.0, 0.0] }, /* top left */
      Vertex { xyz: [1.0, 0.0, 0.0], norm: [0.0, 0.0, -1.0], uv: [0.0, 1.0] }, /* bottom right */
      Vertex { xyz: [1.0, 1.0, 0.0], norm: [0.0, 0.0, -1.0], uv: [0.0, 0.0] }, /* top right */
      // Face 2 (top)
      Vertex { xyz: [0.0, 1.0, 0.0], norm: [0.0, 1.0, 0.0], uv: [1.0, 1.0] }, /* bottom left */
      Vertex { xyz: [0.0, 1.0, 1.0], norm: [0.0, 1.0, 0.0], uv: [1.0, 0.0] }, /* top left */
      Vertex { xyz: [1.0, 1.0, 0.0], norm: [0.0, 1.0, 0.0], uv: [0.0, 1.0] }, /* bottom right */
      Vertex { xyz: [1.0, 1.0, 1.0], norm: [0.0, 1.0, 0.0], uv: [0.0, 0.0] }, /* top right */
      // Face 3 (back)
      Vertex { xyz: [0.0, 0.0, 1.0], norm: [0.0, 0.0, 1.0], uv: [0.0, 1.0] }, /* bottom left */
      Vertex { xyz: [0.0, 1.0, 1.0], norm: [0.0, 0.0, 1.0], uv: [0.0, 0.0] }, /* top left */
      Vertex { xyz: [1.0, 0.0, 1.0], norm: [0.0, 0.0, 1.0], uv: [1.0, 1.0] }, /* bottom right */
      Vertex { xyz: [1.0, 1.0, 1.0], norm: [0.0, 0.0, 1.0], uv: [1.0, 0.0] }, /* top right */
      // Face 4 (bottom)
      Vertex { xyz: [0.0, 0.0, 0.0], norm: [0.0, -1.0, 0.0], uv: [1.0, 1.0] }, /* bottom left */
      Vertex { xyz: [0.0, 0.0, 1.0], norm: [0.0, -1.0, 0.0], uv: [1.0, 0.0] }, /* top left */
      Vertex { xyz: [1.0, 0.0, 0.0], norm: [0.0, -1.0, 0.0], uv: [0.0, 1.0] }, /* bottom right */
      Vertex { xyz: [1.0, 0.0, 1.0], norm: [0.0, -1.0, 0.0], uv: [0.0, 0.0] }, /* top right */
      // Face 5 (left)
      Vertex { xyz: [0.0, 0.0, 1.0], norm: [-1.0, 0.0, 0.0], uv: [1.0, 1.0] }, /* bottom left */
      Vertex { xyz: [0.0, 1.0, 1.0], norm: [-1.0, 0.0, 0.0], uv: [1.0, 0.0] }, /* top left */
      Vertex { xyz: [0.0, 0.0, 0.0], norm: [-1.0, 0.0, 0.0], uv: [0.0, 1.0] }, /* bottom right */
      Vertex { xyz: [0.0, 1.0, 0.0], norm: [-1.0, 0.0, 0.0], uv: [0.0, 0.0] }, /* top right */
      // Face 6 (right)
      Vertex { xyz: [1.0, 0.0, 0.0], norm: [1.0, 0.0, 0.0], uv: [1.0, 1.0] }, /* bottom left */
      Vertex { xyz: [1.0, 1.0, 0.0], norm: [1.0, 0.0, 0.0], uv: [1.0, 0.0] }, /* top left */
      Vertex { xyz: [1.0, 0.0, 1.0], norm: [1.0, 0.0, 0.0], uv: [0.0, 1.0] }, /* bottom right */
      Vertex { xyz: [1.0, 1.0, 1.0], norm: [1.0, 0.0, 0.0], uv: [0.0, 0.0] }, /* top right */
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let indices: Vec::<u32> = vec!(
        0,  1,  2,  2,  1,  3, // front
        4,  5,  6,  7,  6,  5, // top
        10,  9,  8,  9, 10, 11, // back
        12, 14, 13, 15, 13, 14, // bottom
        16, 17, 18, 19, 18, 17, // left
        20, 21, 22, 23, 22, 21, // right
    );

    MeshBuilder::new()
        .with_vertices(vertices)
        .with_indices(Indices::U32(indices.into()))
        .into()
}
