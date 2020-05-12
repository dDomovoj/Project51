// use std::path::PathBuf;
use crate::render_voxel::{Voxel, Material};
use crate::bundles::camera_control_bundle::{CreativeMovementControlTag, MouseControlTag};

use amethyst::{
    // assets::{AssetStorage, Loader, Handle},
    assets::{Loader},
    controls::HideCursor,
    core::{
        math::{Point3, Vector3},
        transform::Transform,
    },
    // assets::RonFormat,
    // core::transform::TransformBundle,
    ecs::{WorldExt, Write}, //prelude::Write, EntityBuilder, },
    // error::Error,
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{
        debug_drawing::DebugLinesComponent, // DebugLine, DebugLines, DebugLinesParams},
        light::{Light, PointLight},         //, SunLight},
        // ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        // mtl::{Material as AmethystMaterial, MaterialDefaults},
        palette::{Srgb, Srgba},
        // rendy::{
        // mesh::{MeshBuilder, Normal, Position, Tangent, TexCoord},
        // texture::palette::{load_from_linear_rgba, load_from_srgb, load_from_srgba},
        // util::types::vertex::{Color, PosColor, PosTex},
        // },
        resources::AmbientColor,
        // shape::Shape,
        // types::{Mesh, MeshData}, //, Texture},
        Camera,
        // ImageFormat,
        // Texture,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    utils::auto_fov::AutoFov,
    window::ScreenDimensions,
    winit::{MouseButton, VirtualKeyCode},
};

use std::f32::consts::{FRAC_PI_4, FRAC_PI_8};

extern crate rand;
use rand::distributions::{Distribution, Uniform};

pub struct GameStart;

const SPHERE_RADIUS: f32 = 6.0_f32;
const CAMERA_DISTANCE_M: f32 = 6.0_f32;

impl SimpleState for GameStart {
    
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        spawn_axis(world);
        // spawn_blocks(world);
        spawn_block_sphere(world, SPHERE_RADIUS);
        spawn_lights(world);
        initialize_camera(world);
        initialize_ui(world);
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
    let distance = CAMERA_DISTANCE_M;
    let mut transform = Transform::default();
    transform
        .set_translation_xyz(2. * distance, 1. * distance, 2. * distance)
        .append_rotation_y_axis(FRAC_PI_4)
        .append_rotation_x_axis(-FRAC_PI_8);

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let auto_fov = AutoFov::new();
    world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(MouseControlTag)
        .with(CreativeMovementControlTag)
        .with(auto_fov)
        .with(transform)
        .build();
}

// endregion

// region - UI

fn initialize_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let transform = UiTransform::new(
        "FPS".to_string(), Anchor::TopLeft, Anchor::TopLeft,
        0., 0., 1., 200., 50.,
    );
    world
        .create_entity()
        .with(transform)
        .with(UiText::new(font.clone(), "".to_string(), [1., 1., 1., 1.], 50.))
        .build();
}

// endregion

// region - Light

// 1 -1 2
fn spawn_lights(world: &mut World) {
    world.exec(|mut color: Write<'_, AmbientColor>| {
        color.0 = Srgba::new(0.15, 0.15, 0.15, 1.0);
    });

    let light1: Light = PointLight {
        intensity: 14.0,
        color: Srgb::new(1.0, 0.95, 0.9),
        ..PointLight::default()
    }
    .into();

    let mut light1_transform = Transform::default();
    // light1_transform.set_translation_xyz(-4.0, 3.0, -6.0);
    light1_transform.set_translation_xyz(-12.0, 9.0, -18.0);

    let light2: Light = PointLight {
        intensity: 15.0,
        color: Srgb::new(0.8, 0.9, 0.95),
        ..PointLight::default()
    }
    .into();

    let mut light2_transform = Transform::default();
    // light2_transform.set_translation_xyz(2.0, 1.0, 1.0);
    light2_transform.set_translation_xyz(6.0, 3.0, 3.0);

    let light3: Light = PointLight {
        intensity: 13.0,
        color: Srgb::new(0.8, 1.0, 0.85),
        ..PointLight::default()
    }
    .into();

    let mut light3_transform = Transform::default();
    // light3_transform.set_translation_xyz(-1.0, -2.0, 1.0);
    light3_transform.set_translation_xyz(-3.0, -6.0, 3.0);

    world.create_entity().with(light1).with(light1_transform).build();

    world.create_entity().with(light2).with(light2_transform).build();

    world.create_entity().with(light3).with(light3_transform).build();
}

// endregion

// region - Debug

fn spawn_axis(world: &mut World) {
    let mut debug_lines = DebugLinesComponent::new();
    let origin = Point3::from(Vector3::new(0.0, 0.0, 0.0));
    let axis_length = 16.0_f32;

    let x_axis_direction = Vector3::new(1.0, 0.0, 0.0);
    let x_axis_color = Srgba::new(1.0, 0.0, 0.0, 1.0);
    debug_lines.add_line(origin, origin + x_axis_direction * axis_length, x_axis_color);
    // debug_lines.add_direction(origin, x_axis_direction, x_axis_color);

    let y_axis_direction = Vector3::new(0.0, 1.0, 0.0);
    let y_axis_color = Srgba::new(0.0, 1.0, 0.0, 1.0);
    // debug_lines.add_direction(origin, y_axis_direction, y_axis_color);
    debug_lines.add_line(origin, origin + y_axis_direction * axis_length, y_axis_color);

    let z_axis_direction = Vector3::new(0.0, 0.0, 1.0);
    let z_axis_color = Srgba::new(0.0, 0.0, 1.0, 1.0);
    // debug_lines.add_direction(origin, z_axis_direction, z_axis_color);
    debug_lines.add_line(origin, origin + z_axis_direction * axis_length, z_axis_color);

    world.create_entity().with(debug_lines).build();
}

// endregion

// region - Blocks

fn _spawn_blocks(world: &mut World) {
    let mut rng = rand::thread_rng();
    let range = 32_i128; // 4, 16, 64
    let chunks = 256_i32; // 1, 8, 256
    let axis = Uniform::from(-range..range);
    for _ in 0..(chunks * 16) {
        let (x, y, z) = (axis.sample(&mut rng), axis.sample(&mut rng), axis.sample(&mut rng));
        // let (x, y, z) = (axis.sample(&mut rng), 0, axis.sample(&mut rng));
        spawn_block(world, [x, y, z], Material::Dirt);
    }
}

fn spawn_block(world: &mut World, position: [i128; 3], material: Material) {
    let mut trans = Transform::default();
    trans.append_translation_xyz(
        position[0] as f32,
        position[1] as f32,
        position[2] as f32,
    );

    let block = Voxel { position, material };
    block.create_entity(world).with(trans).build();
}

fn spawn_block_sphere(world: &mut World, radius: f32) {
    let r = radius.ceil() as i128;
    for x in -r..=r {
        for y in -r..=r {
            for z in -r..=r {
                let distance = {
                    let x = (x as f32) + 0.5;
                    let y = (y as f32) + 0.5;
                    let z = (z as f32) + 0.5;
                    (x * x + y * y + z * z).sqrt()
                };
                if distance <= radius {
                    spawn_block(world, [x, y, z], Material::Grass);
                }
            }
        }
    }
}

// endregion
