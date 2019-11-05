extern crate amethyst;

use std::thread;
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
    static ref GCONFIG: Config = Config { stage_height: 0.0, stage_width: 0.0};
}


#[derive(Default)]
pub struct LoadingState{
    pub config_path: String,
    pub config: Arc<Mutex<Option<Config>>>,
    pub use_config: Config,
    pub loading: Arc<AtomicBool>,
}

#[derive(Default)]
pub struct PlayState{

}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();

    let s_w = GCONFIG.stage_width;
    let s_h = GCONFIG.stage_height;
    
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
        self.config = Arc::new(Mutex::new(None));
        println!("Started loading");
    }
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>> ) -> SimpleTrans{
        let path = self.config_path.clone();
        let load = self.loading.clone();
        let conf_ref = Arc::clone(&self.config);
    
        if (*self.loading).load(Ordering::Relaxed) {
            match self.config.try_lock(){
                Err(_) => Trans::None,
                x => {
                    self.use_config = *(*x.unwrap()).as_ref().unwrap();
                    println!("Loaded config: {:?}", self.use_config);
                    Trans::Quit
                }
            }
        }else{
            let thr = thread::spawn(move || {
                let data = conf_ref.try_lock();
                match data {
                    Err(_) => None,
                    Ok(mut x) => {
                        if (*load).load(Ordering::Relaxed) {
                            None
                        }else{
                            println!("Starting load thread!");
                            let contents = fs::read_to_string(path)
                                .expect("Error reading config file");
                            let loaded: Config = from_str(&contents)
                                .expect("Error loading config file");
                            (*x).replace(loaded);
                            println!("Loaded!");
                            (*load).store(true, Ordering::Relaxed);
                            Some(true)
                        }
                    }
                }
            });

            if (*self.loading).load(Ordering::Relaxed) {
                match self.config.try_lock(){
                    Err(_) => Trans::None,
                    x => {
                        thr.join().expect("Error encountered while joining thread");
                        self.use_config = *(*x.unwrap()).as_ref().unwrap();
                        println!("Loaded config: {:?}", self.use_config);
                        Trans::Quit
                    }
                }
            }else{
                println!("Loading..");
                Trans::None
            }
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