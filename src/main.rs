use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        palette::Srgb,
        plugins::{RenderSkybox, RenderToWindow},
        RenderingBundle,
    },
    utils::application_root_dir,
    Error,
};
use amethyst::utils::fps_counter::{FpsCounter, FpsCounterBundle};
use amethyst::ui::{RenderUi, UiBundle};

mod voxel;
mod bundles;
mod game_start;

#[macro_use]
mod render_macros;
mod render_material;
mod render_material_sub;
mod render_mesh;
mod render_pass;
mod render_plugins;
mod render_shader;
mod render_system;
mod render_vertex;
mod render_cache;
mod systems;

use crate::bundles::camera_control_bundle::CameraControlBundle;
use crate::game_start::GameStart;
use crate::render_plugins::RenderDebugLines;
use crate::render_system::{ExtendedRenderingSystem, MeshProcessorSystem};

// use amethyst::renderer::plugins::RenderShaded3D as Render3D;
// use amethyst::renderer::types::DefaultBackend as DefaultExtendedBackend;
use crate::render_pass::Render3D;
use crate::render_mesh::DefaultExtendedBackend;

use crate::systems::ui::UISystem;

#[macro_use]
extern crate guard;

fn main() -> Result<(), Error> {
    amethyst::start_logger(amethyst::LoggerConfig {
        // level_filter: amethyst::LogLevelFilter::Debug,
        ..Default::default()
    });

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");

    let display_config_path = app_root.join("config/display.ron");

    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?)?
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
                .with_plugin(Render3D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(
            ExtendedRenderingSystem::<DefaultExtendedBackend>::default(),
            "extended_rendering_system",
            &[],
        )
        .with(
            UISystem::default(),
            "ui_system",
            &[],
        )
        .with(
            MeshProcessorSystem::<DefaultExtendedBackend>::default(),
            "extended_mesh_processor",
            &["extended_rendering_system"],
        );
        // .with_system_desc(
        //     UiGlyphsSystemDesc::<DefaultExtendedBackend>::default(),
        //     "ui_glyph_system",
        //     &[],
        // );

    let mut game = Application::build(assets_dir, GameStart)?.build(game_data)?;
    game.run();
    Ok(())
}
