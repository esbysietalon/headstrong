extern crate amethyst;

use std::env;
use std::fs;

use serde::Deserialize;
use ron::de::from_str;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Debug, Deserialize, Default)]
pub struct Config{
    stage_height: f32,
    stage_width: f32
}

#[derive(Default)]
pub struct LoadingState{
    pub config_path: String,
    pub config: Config,
}

impl SimpleState for LoadingState{
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        println!("Started loading");
        
    }
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>> ) -> SimpleTrans{
        let contents = fs::read_to_string(&self.config_path)
            .expect("Error while loading file");
        self.config = match from_str(&contents){
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to load config: {}", e);
            }
        };
        println!("Loaded config: {:?}", &self.config);
        Trans::None
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        println!("Stopped loading");
    }
}