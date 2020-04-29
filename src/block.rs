// use std::path::PathBuf;
// use crate::bundles::camera_control_bundle::{MouseControlTag, CreativeMovementControlTag};

use amethyst::{
    // assets::RonFormat,
    // core::transform::TransformBundle,
    ecs::{WorldExt, EntityBuilder},
    // assets::{AssetStorage, Loader, Handle},
    assets::{AssetLoaderSystemData},//, Handle, Loader},
    // controls::HideCursor,
    // core::{
    //     transform::Transform,
    //     math::{Point2, Point3, UnitQuaternion, Vector2, Vector3},
    // },
    // error::Error,
    // input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{
        // debug_drawing::{DebugLine, DebugLines, DebugLinesComponent, DebugLinesParams},
        ImageFormat, Texture,
        // light::{Light, PointLight, SunLight},
        // ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        mtl::{Material as AmethystMaterial, MaterialDefaults},
        // palette::{Srgb, Srgba, LinSrgba},
        rendy::{
            mesh::{Normal, Position, /*Tangent, */TexCoord},
            // texture::palette::{load_from_srgba, load_from_srgb, load_from_linear_rgba},
            // util::types::vertex::{PosTex, PosColor, Color},
        },
        // shape::{Shape},
        // types::{Mesh, MeshData},//, Texture},
        // Camera,
    },
    // window::ScreenDimensions,
    // winit::{MouseButton, VirtualKeyCode},
};

// use std::f32::consts::{FRAC_PI_8, FRAC_PI_6};

use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::render_mesh::{Mesh, MeshData, MeshBuilder};

// use crate::render_vertex::MaterialIdx;

pub enum Material {
    Dirt,
    Grass,
    Crate
}

pub struct Block {
    pub position: [i128; 3],
    pub material: Material
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

impl Block {

    // fn texture_name(&self) -> &str {
    //     match &self.material {
    //         Material::Grass => "grass_block_side",
    //         Material::Crate => "crate",
    //         Material::Dirt => "dirt",
    //     }
    // }

    pub fn create_entity<'a>(&self, world: &'a mut World) -> EntityBuilder<'a> {
        // let default_mat = world.read_resource::<MaterialDefaults>().0.clone();
        let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
            loader.load_from_data(block_mesh(), (),)
        });

        // let texture = world.exec(|loader: AssetLoaderSystemData<Texture>| {
        //     loader.load(
        //         format!("texture/{}.png", self.texture_name()),
        //         ImageFormat::default(),
        //         (),
        //     )
        // });

        // let mat = world.exec(|loader: AssetLoaderSystemData<AmethystMaterial>| {
        //     loader.load_from_data(
        //         AmethystMaterial {
        //             albedo: texture,
        //             ..default_mat.clone()
        //         },
        //         (),
        //     )
        // });

        world.create_entity()
            .with(mesh)
            // .with(mat)
    }

}

#[rustfmt::skip]
fn block_mesh() -> MeshData {
    let v: [[f32; 3]; 8] = [
        [-0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], 
        [0.5, -0.5, 0.5], [0.5, -0.5, -0.5],
        [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5], 
        [0.5, 0.5, 0.5], [0.5, 0.5, -0.5]
    ];

    let pos: [[f32; 3]; 36] = [
        v[2], v[1], v[3],  v[2], v[0], v[1], // D - v
        v[7], v[4], v[6],  v[7], v[5], v[4], // U - v
        v[6], v[0], v[2],  v[6], v[4], v[0], // F - v
        v[3], v[5], v[7],  v[3], v[1], v[5], // B - v
        v[4], v[1], v[0],  v[4], v[5], v[1], // L - v
        v[7], v[2], v[3],  v[7], v[6], v[2], // R - v
    ];

    let n: [[f32; 3]; 6] = [
        [0.0, -1.0, 0.0],   // D - v
        [0.0, 1.0, 0.0],    // U - v
        [0.0, 0.0, 1.0],    // F - v
        [0.0, 0.0, -1.0],   // B - v
        [-1.0, 0.0, 0.0],   // L - v
        [1.0, 0.0, 0.0],    // R - v
    ];

    let norm: [[f32; 3]; 36] = [
        n[0], n[0], n[0], n[0], n[0], n[0], // D - v
        n[1], n[1], n[1], n[1], n[1], n[1], // U - v
        n[2], n[2], n[2], n[2], n[2], n[2], // F - v
        n[3], n[3], n[3], n[3], n[3], n[3], // B - v
        n[4], n[4], n[4], n[4], n[4], n[4], // L - v
        n[5], n[5], n[5], n[5], n[5], n[5], // R - v
    ];

    // let t: [[f32; 4]; 6] = [
    //     [1.0, 0.0, 1.0, 1.0], // D - v
    //     [1.0, 0.0, 1.0, 1.0], // U - v
    //     [1.0, 1.0, 0.0, 1.0], // F - v
    //     [1.0, 1.0, 0.0, 1.0], // B - v
    //     [0.0, 1.0, 1.0, 1.0], // L - v
    //     [0.0, 1.0, 1.0, 1.0], // R - v
    // ];

    // let tn: [[f32; 4]; 36] = [
    //     t[0], t[0], t[0], t[0], t[0], t[0], // D - v
    //     t[1], t[1], t[1], t[1], t[1], t[1], // U - v
    //     t[2], t[2], t[2], t[2], t[2], t[2], // F - v
    //     t[3], t[3], t[3], t[3], t[3], t[3], // B - v
    //     t[4], t[4], t[4], t[4], t[4], t[4], // L - v
    //     t[5], t[5], t[5], t[5], t[5], t[5], // R - v
    // ];

    // let m: [u32; 36] = [
    //     0, 0, 0, 0, 0, 0, // D - v
    //     1, 1, 1, 1, 1, 1, // U - v
    //     2, 2, 2, 2, 2, 2, // F - v
    //     3, 3, 3, 3, 3, 3, // B - v
    //     4, 4, 4, 4, 4, 4, // L - v
    //     5, 5, 5, 5, 5, 5, // R - v
    // ];

    // let tex: [[f32; 2]; 36] = [
    //     [0., 1.], [1., 0.], [0., 0.],  [0., 1.], [1., 1.], [1., 0.], // D - v
    //     [1., 0.], [0., 1.], [1., 1.],  [1., 0.], [0., 0.], [0., 1.], // U - v
    //     [1., 0.], [0., 1.], [1., 1.],  [1., 0.], [0., 0.], [0., 1.], // F - v
    //     [1., 1.], [0., 0.], [1., 0.],  [1., 1.], [0., 1.], [0., 0.], // B - v
    //     [1., 0.], [0., 1.], [1., 1.],  [1., 0.], [0., 0.], [0., 1.], // L - v
    //     [1., 0.], [0., 1.], [1., 1.],  [1., 0.], [0., 0.], [0., 1.], // R - v
    // ];

    let pos: Vec<Position> = pos.iter().map(|&coords| { Position(coords) }).collect();
    let norm: Vec<Normal> = norm.iter().map(|&coords| { Normal(coords) }).collect();
    // let tn: Vec<Tangent> = tn.iter().map(|&coords| { Tangent(coords) }).collect();
    // let m: Vec<MaterialIdx> = m.iter().map(|&idx| { MaterialIdx(idx) }).collect();
    // let tex: Vec<TexCoord> = tex.iter().map(|&coords| { TexCoord(coords) }).collect();
    MeshBuilder::new()
        .with_vertices(pos)
        .with_vertices(norm)
        // .with_vertices(tn)
        // .with_vertices(m)
        // .with_vertices(tex)
        .into()
}
