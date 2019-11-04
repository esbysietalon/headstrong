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

#[derive(Default)]
pub struct LoadingState{
    pub config_path: String,
    pub config: Arc<Mutex<Option<Config>>>,
    pub use_config: Config,
    pub loading: Arc<AtomicBool>,
}


impl SimpleState for LoadingState{
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
                let mut data = conf_ref.try_lock();
                match data {
                    Err(_) => None,
                    Ok(mut x) => {
                        if((*load).load(Ordering::Relaxed)){
                            None
                        }else{
                            println!("Starting load thread!");
                            let contents = fs::read_to_string(path)
                                .expect("Error reading config file");
                            let loaded: Config = from_str(&contents)
                                .expect("Error loading config file");
                            
                            thread::sleep_ms(10000);
                            //println!("{:?}", x);
                            (*x).replace(loaded);
                            //println!("{:?}", x);
                            println!("Loaded!");
                            (*load).store(true, Ordering::Relaxed);
                            Some(true)
                        }
                    }
                }
            });

            Trans::None
        }
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>){
        println!("Stopped loading");
    }
}