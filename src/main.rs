use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir},
    Error,
};

#[derive(Default)]
struct Main { 

}

impl SimpleState for Main {

}

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;

    let main = Main::default();
    let mut game = Application::build(assets_dir, main)?.build(game_data)?;
    game.run();
    Ok(())
}
