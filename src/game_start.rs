// use std::path::PathBuf;
use crate::bundles::camera_control_bundle::{MouseControlTag, CreativeMovementControlTag};

use amethyst::{
    // assets::RonFormat,
    // core::transform::TransformBundle,
    // ecs::WorldExt,
    // assets::{AssetStorage, Loader, Handle},
    assets::{AssetLoaderSystemData, Handle, Loader},
    controls::HideCursor,
    core::{
        transform::Transform,
        math::{Point2, Point3, UnitQuaternion, Vector2, Vector3},
    },
    error::Error,
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{
        debug_drawing::{DebugLine, DebugLines, DebugLinesComponent, DebugLinesParams},
        ImageFormat, Texture,
        light::{Light, PointLight, SunLight},
        // ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        mtl::{Material, MaterialDefaults},
        palette::{Srgb, Srgba, LinSrgba},
        rendy::{
            mesh::{MeshBuilder, Normal, Position, Tangent, TexCoord},
            texture::palette::{load_from_srgba, load_from_srgb, load_from_linear_rgba},
            util::types::vertex::{PosTex, PosColor, Color},
        },
        shape::{Shape},
        types::{Mesh, MeshData},//, Texture},
        Camera,
    },
    window::ScreenDimensions,
    winit::{MouseButton, VirtualKeyCode},
};

use std::f32::consts::{FRAC_PI_8, FRAC_PI_6};

pub struct GameStart;

impl SimpleState for GameStart {
    
    // pub fn new(fonts_dir: PathBuf, audio_dir: PathBuf) -> Pong {
    //     Pong {
    //         ball_spawn_timer: None,
    //         sprite_sheet_handle: None,
    //         fonts_dir,
    //         audio_dir
    //     }
    // }

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // // Load the spritesheet necessary to render the graphics.
        // // `spritesheet` is the layout of the sprites on the image;
        // // `texture` is the pixel data.
        // self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        spawn_axis(world);
        spawn_block(world);
        spawn_lights(world);
        initialize_camera(world);

        // audio::initialise_audio(world, &self.audio_dir);
        // ui::initialize_scoreboard(world, &self.fonts_dir);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            }
        }
        Trans::None
    }
}

// region - Camera

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.append_rotation_y_axis(-FRAC_PI_6)
        .append_rotation_x_axis(-FRAC_PI_8);

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(MouseControlTag)
        .with(CreativeMovementControlTag)
        .with(transform)
        .build();
}

// endregion

// region - Light

fn spawn_lights(world: &mut World) {
    let light1: Light = PointLight {
        intensity: 4.0,
        color: Srgb::new(1.0, 0.95, 0.9),
        ..PointLight::default()
    }
    .into();

    let mut light1_transform = Transform::default();
    light1_transform.set_translation_xyz(-3.0, 4.0, -4.0);

    let light2: Light = PointLight {
        intensity: 5.0,
        color: Srgb::new(0.8, 0.9, 0.95),
        ..PointLight::default()
    }
    .into();

    let mut light2_transform = Transform::default();
    light2_transform.set_translation_xyz(3.0, 0.0, 3.0);

    let light3: Light = PointLight {
        intensity: 3.0,
        color: Srgb::new(0.8, 1.0, 0.85),
        ..PointLight::default()
    }
    .into();

    let mut light3_transform = Transform::default();
    light3_transform.set_translation_xyz(0.0, -3.0, -1.0);

    world
        .create_entity()
        .with(light1)
        .with(light1_transform)
        .build();

    world
        .create_entity()
        .with(light2)
        .with(light2_transform)
        .build();

    world
        .create_entity()
        .with(light3)
        .with(light3_transform)
        .build();
}

// endregion

// region - Debug

fn spawn_axis(world: &mut World) {
    // let x_axis = DebugLine::new(PosColor { 
    //     position: Position::from([0.0, 0.0, 0.0]),
    //     color: Color::from([1.0, 0.0, 0.0, 1.0])
    // }, PosColor { 
    //     position: Position::from([100.0, 0.0, 0.0]),
    //     color: Color::from([1.0, 0.0, 0.0, 1.0])
    // });

    let mut debug_lines = DebugLinesComponent::new();
    let origin = Point3::from(Vector3::new(0.0, 0.0, 0.0));

    let x_axis_direction = Vector3::new(1.0, 0.0, 0.0);
    let x_axis_color = Srgba::new(1.0, 0.0, 0.0, 1.0);
    debug_lines.add_direction(origin, x_axis_direction, x_axis_color);

    let y_axis_direction = Vector3::new(0.0, 1.0, 0.0);
    let y_axis_color = Srgba::new(0.0, 1.0, 0.0, 1.0);
    debug_lines.add_direction(origin, y_axis_direction, y_axis_color);

    let z_axis_direction = Vector3::new(0.0, 0.0, 1.0);
    let z_axis_color = Srgba::new(0.0, 0.0, 1.0, 1.0);
    debug_lines.add_direction(origin, z_axis_direction, z_axis_color);

    world
        .create_entity()
        .with(debug_lines)
        .build();
}

// endregion

// region - Mesh

fn spawn_spheres(world: &mut World) {
    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

        println!("Load mesh");
        let (mesh, albedo) = {
            let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
                loader.load_from_data(
                    Shape::Sphere(12, 12)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    (),
                )
            });
            let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
                loader.load_from_data(
                    load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
                    (),
                )
            });

            (mesh, albedo)
        };

        println!("Create spheres");
        for i in 0..5 {
            for j in 0..5 {
                let roughness = 1.0f32 * (i as f32 / 4.0f32);
                let metallic = 1.0f32 * (j as f32 / 4.0f32);

                let mut pos = Transform::default();
                pos.set_translation_xyz(2.0f32 * (i - 2) as f32, 2.0f32 * (j - 2) as f32, 0.0);

                let mtl = world.exec(
                    |(mtl_loader, tex_loader): (
                        AssetLoaderSystemData<'_, Material>,
                        AssetLoaderSystemData<'_, Texture>,
                    )| {
                        let metallic_roughness = tex_loader.load_from_data(
                            load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0))
                                .into(),
                            (),
                        );

                        mtl_loader.load_from_data(
                            Material {
                                albedo: albedo.clone(),
                                metallic_roughness,
                                ..mat_defaults.clone()
                            },
                            (),
                        )
                    },
                );

                world
                    .create_entity()
                    .with(pos)
                    .with(mesh.clone())
                    .with(mtl)
                    .build();
            }
        }
}

fn spawn_block(world: &mut World) {
    let default_mat = world.read_resource::<MaterialDefaults>().0.clone();
    let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
        loader.load_from_data(block_mesh(), (),)
    });

    // let albedo = world.exec(|loader: AssetLoaderSystemData<Texture>| {
    //     loader.load_from_data(
    //         load_from_srgba(Srgba::new(1.0, 0.0, 0.0, 0.5)).into(),
    //         (),
    //     )
    // });

    let texture = world.exec(|loader: AssetLoaderSystemData<Texture>| {
        loader.load(
            "texture/grass_block_side.png",
            ImageFormat::default(),
            (),
        )
    });

    let mat = world.exec(|loader: AssetLoaderSystemData<Material>| {
        loader.load_from_data(
            Material {
                // albedo,
                albedo: texture,
                ..default_mat.clone()
            },
            (),
        )
    });

    let mut trans = Transform::default();
    trans.set_translation_xyz(1.0, -1.0, -2.0);
    world
        .create_entity()
        .with(mesh)
        .with(mat)
        .with(trans)
        .build();
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

    let tex: [[f32; 2]; 36] = [
        [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],  [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], // D - 
        [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],  [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], // U - 
        [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],  [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], // F - 
        [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],  [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], // B - 
        [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],  [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], // L - 
        [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],  [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], // R - 

        // [1.0, 1.0], [0.0, 0.0], [1.0, 0.0],  [1.0, 1.0], [0.0, 1.0], [0.0, 0.0],
    ];

    let pos: Vec<Position> = pos.iter().map(|&coords| { Position(coords) }).collect();
    let norm: Vec<Normal> = norm.iter().map(|&coords| { Normal(coords) }).collect();
    let tex: Vec<TexCoord> = tex.iter().map(|&coords| { TexCoord(coords) }).collect();
    MeshBuilder::new()
        .with_vertices(pos)
        .with_vertices(norm)
        .with_vertices(tex)
        .into()
}


// endregion