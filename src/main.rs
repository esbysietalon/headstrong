#[macro_use]
extern crate lazy_static;
extern crate amethyst;

mod game_state;

use game_state::*;

use amethyst::{
    Logger,
    GameDataBuilder,
    Application,

    core::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend, 
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    Logger::from_config(Default::default())
        .level_for("amethyst_rendy", amethyst::LogLevelFilter::Warn)
        .start();
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let game_config_path = app_root.join("config").join("globals.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut load_state = LoadingState::default();
    load_state.config_path = game_config_path.to_str().unwrap().to_string();

    let mut game = Application::new(app_root, load_state, game_data)?;
    game.run();
    
    Ok(())
}
