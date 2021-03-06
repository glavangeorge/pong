extern crate amethyst;

mod pong;
mod system;
use crate::pong::Pong;

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat2D, Event, Pipeline, RenderBundle, Stage, VirtualKeyCode,
};

use amethyst::utils::application_root_dir;

use amethyst::input::InputBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());

    let bindig_path = format!("{}/resources/binding_config.ron", application_root_dir());

    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(bindig_path)?;

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(system::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
