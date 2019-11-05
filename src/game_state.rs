extern crate amethyst;

use std::thread;
use std::thread::JoinHandle;
use std::fs;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use serde::Deserialize;
use ron::de::from_str;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Copy, Clone, Debug, Deserialize, Default)]
pub struct Config{
    stage_height: f32,
    stage_width: f32
}

lazy_static!{
    static ref GCONFIG: Mutex<Config> = Mutex::new(Config { stage_height: 0.0, stage_width: 0.0});
}


#[derive(Default)]
pub struct LoadingState{
    pub config_path: String,
    pub loading: Arc<AtomicBool>,
    pub load_thread: Option<JoinHandle<()>>,
}

#[derive(Default)]
pub struct PlayState{

}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();

    let s_w = GCONFIG.lock().unwrap().stage_width;
    let s_h = GCONFIG.lock().unwrap().stage_height;
    
    transform.set_translation_xyz(s_w * 0.5, s_h * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(s_w, s_h))
        .with(transform)
        .build();
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        self.loading = Arc::new(AtomicBool::new(false));
        let load = self.loading.clone();
        let path = self.config_path.clone(); 
        self.load_thread.replace(thread::spawn(move || {        
            if !(*load).load(Ordering::Relaxed) {
                println!("Starting load thread!");
                let contents = fs::read_to_string(path)
                    .expect("Error reading config file");
                let loaded: Config = from_str(&contents)
                    .expect("Error loading config file");
                GCONFIG.lock().unwrap().stage_width = loaded.stage_width;
                GCONFIG.lock().unwrap().stage_height = loaded.stage_height;
                (*load).store(true, Ordering::Relaxed);
                println!("Loaded!");
            }
        }));
        println!("Started loading");
    }
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>> ) -> SimpleTrans{
        if (*self.loading).load(Ordering::Relaxed) {
            self.load_thread.take().unwrap().join().expect("Error encountered while joining thread");
            println!("Loaded config: {:?}", GCONFIG.lock().unwrap());
            Trans::Quit
        }else{
            println!("Loading..");
            Trans::None
        }
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        println!("Stopped loading");
    }
}

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;
        
        initialise_camera(world);
    }
}