// use std::path::PathBuf;
use crate::block::{Block, Material};
use crate::bundles::camera_control_bundle::{CreativeMovementControlTag, MouseControlTag};

use amethyst::{
    // assets::{AssetStorage, Loader, Handle},
    // assets::{AssetLoaderSystemData, Handle, Loader},
    controls::HideCursor,
    core::{
        math::{Point3, Vector3},
        transform::Transform,
    },
    // assets::RonFormat,
    // core::transform::TransformBundle,
    ecs::{WorldExt},//prelude::Write, EntityBuilder, },
    // error::Error,
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{
        debug_drawing::{DebugLinesComponent},// DebugLine, DebugLines, DebugLinesParams},
        light::{Light, PointLight},//, SunLight},
        // ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        // mtl::{Material as AmethystMaterial, MaterialDefaults},
        palette::{Srgb, Srgba},
        // rendy::{
            // mesh::{MeshBuilder, Normal, Position, Tangent, TexCoord},
            // texture::palette::{load_from_linear_rgba, load_from_srgb, load_from_srgba},
            // util::types::vertex::{Color, PosColor, PosTex},
        // },
        // resources::AmbientColor,
        // shape::Shape,
        // types::{Mesh, MeshData}, //, Texture},
        Camera,
        // ImageFormat,
        // Texture,
    },
    utils::auto_fov::AutoFov,
    window::ScreenDimensions,
    winit::{MouseButton, VirtualKeyCode},
};

use std::f32::consts::{FRAC_PI_6, FRAC_PI_8};

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

        // use crate::render_mesh::Mesh;
        // world.register::<Mesh>();

        spawn_axis(world);
        spawn_blocks(world);
        spawn_lights(world);
        initialize_camera(world);

        // audio::initialise_audio(world, &self.audio_dir);
        // ui::initialize_scoreboard(world, &self.fonts_dir);
    }

    fn handle_event(
        &mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent,
    ) -> SimpleTrans {
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
    transform
        .set_translation_xyz(-1.5, 1.5, 3.0)
        .append_rotation_y_axis(-FRAC_PI_6)
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

// region - Light

// 1 -1 2
fn spawn_lights(world: &mut World) {
    // world.exec(|mut color: Write<'_, AmbientColor>| {
    //     color.0 = Srgba::new(0.3, 0.3, 0.3, 1.0);
    // });

    let light1: Light = PointLight {
        intensity: 14.0,
        color: Srgb::new(1.0, 0.95, 0.9),
        ..PointLight::default()
    }
    .into();

    let mut light1_transform = Transform::default();
    light1_transform.set_translation_xyz(-4.0, 3.0, -6.0);

    let light2: Light = PointLight {
        intensity: 15.0,
        color: Srgb::new(0.8, 0.9, 0.95),
        ..PointLight::default()
    }
    .into();

    let mut light2_transform = Transform::default();
    light2_transform.set_translation_xyz(2.0, 1.0, 1.0);

    let light3: Light = PointLight {
        intensity: 13.0,
        color: Srgb::new(0.8, 1.0, 0.85),
        ..PointLight::default()
    }
    .into();

    let mut light3_transform = Transform::default();
    light3_transform.set_translation_xyz(-1.0, -2.0, 1.0);

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

    world.create_entity().with(debug_lines).build();
}

// endregion

// region - Blocks

fn spawn_blocks(world: &mut World) {
    spawn_block(world, [0, 0, 0], Material::Grass);
    spawn_block(world, [1, 0, 0], Material::Crate);
    // spawn_block(world, [1, -1, 0], Material::Crate);
    // spawn_block(world, [1, 0, 1], Material::Grass);
    // spawn_block(world, [1, -1, 1], Material::Crate);
    // spawn_block(world, [0, -1, 0], Material::Dirt);
    // spawn_block(world, [0, -1, 1], Material::Dirt);
}

fn spawn_block(world: &mut World, position: [i128; 3], material: Material) {
    let mut trans = Transform::default();
    trans.append_translation_xyz(
        position[0] as f32 + 0.5,
        position[1] as f32 + 0.5,
        position[2] as f32 + 0.5,
    );

    let block = Block { position, material };
    block.create_entity(world).with(trans).build();
}

// endregion
