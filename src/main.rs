use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        palette::Srgb,
        plugins::{RenderSkybox, RenderToWindow},
        // types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Error,
};

mod block;
mod bundles;
mod game_start;
// mod render_bundle;
mod render_mesh;
mod render_pass;
mod render_plugins;
mod render_shader;
mod render_system;
mod render_vertex;
mod systems;

use crate::bundles::camera_control_bundle::CameraControlBundle;
use crate::game_start::GameStart;
use crate::render_mesh::DefaultExtendedBackend;
use crate::render_pass::Render3D;
use crate::render_plugins::RenderDebugLines;
use crate::render_system::{MeshProcessorSystem};

#[macro_use]
extern crate guard;

// #[macro_use]
// extern crate glsl_layout;

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");

    let display_config_path = app_root.join("config/display.ron");

    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(
            CameraControlBundle::<StringBindings>::new()
                .with_speed(3.0)
                .with_sensitivity(0.1, 0.1)
                .with_side_input_axis(Some(String::from("move_side")))
                .with_forward_input_axis(Some(String::from("move_forward")))
                .with_up_input_axis(Some(String::from("move_up"))),
        )?
        .with_bundle(TransformBundle::new().with_dep(&["mouse_rotation", "creative_movement"]))?
        .with_bundle(
            RenderingBundle::<DefaultExtendedBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.18, 0.11, 0.85),
                ))
                // .with_plugin(RenderShaded3D::default())
                .with_plugin(Render3D::default())
                .with_plugin(RenderDebugLines::default())
        )?
        .with(
            MeshProcessorSystem::<DefaultExtendedBackend>::default(),
            "extended_mesh_processor",
            &[],
        );
        // .with(ExtendedRenderingSystem::<DefaultExtendedBackend>::default(), "mesh_processor", &[]);

    let mut game = Application::build(assets_dir, GameStart)?.build(game_data)?;
    game.run();
    Ok(())
}
