use amethyst::ui::{UiBundle};
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::window::{WindowBundle};
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    utils::application_root_dir,
    Error,
    ui::UiGlyphsSystemDesc,
};

use amethyst::assets::Processor;

mod bundles;
mod game_start;

#[macro_use]
mod render_macros;

mod render_backend;
mod render_cache;
mod render_chunk;
mod render_graph;
mod render_material;
mod render_material_sub;
mod render_mesh;
mod render_pass;
mod render_shader;
mod render_system;
mod render_vertex;
mod render_visibility;
mod render_voxel;
mod systems;

use crate::bundles::camera_control_bundle::CameraControlBundle;
use crate::game_start::GameStart;
use crate::render_graph::RenderGraph;
use crate::render_system::{ExtendedRenderingSystem, MeshProcessorSystem, TextureProcessorSystem};
use crate::render_material::Material;

use crate::render_backend::DefaultExtendedBackend as DefaultBackend;
use crate::render_visibility::VisibilitySortingSystem;

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
        .with_bundle(UiBundle::<StringBindings>::new())?
        // .with_bundle(HotReloadBundle::default())?
        .with_bundle(FpsCounterBundle::default())?
        // The below Systems, are used to handle some rendering resources.
        // Most likely these must be always called as last thing.
        .with_system_desc(UiGlyphsSystemDesc::<DefaultBackend>::default(), "ui_glyph_system", &[])
        .with(VisibilitySortingSystem::new(), "visibility_sorting_system", &[])
        .with(
            MeshProcessorSystem::<DefaultBackend>::default(),
            "mesh_processor",
            &[],
        )
        .with(
            TextureProcessorSystem::<DefaultBackend>::default(),
            "texture_processor",
            &[],
        )
        .with(UISystem::default(), "ui_system", &[])
        .with(Processor::<Material>::new(), "material_processor", &[])
        .with_bundle(WindowBundle::from_config_path(display_config_path)?)?
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        .with_thread_local(ExtendedRenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));

    let mut game = Application::build(assets_dir, GameStart)?.build(game_data)?;
    game.run();
    Ok(())
}
