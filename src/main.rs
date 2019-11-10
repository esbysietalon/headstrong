extern crate amethyst;

mod game_state;
mod systems;
mod components;

use game_state::*;

use amethyst::{
    Logger,
    GameDataBuilder,
    Application,
    input::{InputBundle, StringBindings},
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
    let binding_path = app_root.join("config").join("bindings.ron");
    let display_config_path = app_root.join("config").join("display.ron");
    let game_config_path = app_root.join("config").join("globals.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::BallSystem, "ball_system", &[])
        .with(systems::BounceSystem, "ball_bounce_system", &["paddle_system", "ball_system"])
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
