extern crate amethyst;

use crate::components;

use std::thread;
use std::thread::JoinHandle;
use std::fs;

use std::sync::{Arc};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use serde::Deserialize;
use ron::de::from_str;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    core::timing::Time,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;


#[derive(Copy, Clone, Debug, Deserialize, Default)]
pub struct Config{
    pub stage_height: f32,
    pub stage_width: f32
}

#[derive(Default)]
pub struct LoadingState{
    pub config_path: String,
    pub loading: Arc<AtomicBool>,
    pub load_thread: Option<JoinHandle<(Config)>>,
}

#[derive(Default)]
pub struct PlayState{
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    
    let s_w = world.read_resource::<Config>().stage_width;
    let s_h = world.read_resource::<Config>().stage_height;

    transform.set_translation_xyz(s_w * 0.5, s_h * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(s_w, s_h))
        .with(transform)
        .build();
}

fn initialise_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>){
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = world.read_resource::<Config>().stage_height / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(world.read_resource::<Config>().stage_width - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(components::Paddle::new(components::Side::Left, PADDLE_WIDTH, PADDLE_HEIGHT))
        .with(left_transform)
        .build();
    
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(components::Paddle::new(components::Side::Right, PADDLE_WIDTH, PADDLE_HEIGHT))
        .with(right_transform)
        .build();
}
fn initialise_ball(world: &mut World, sprite_sheet_handler: Handle<SpriteSheet>){
    let mut local_transform = Transform::default();

    let s_w = world.read_resource::<Config>().stage_width;
    let s_h = world.read_resource::<Config>().stage_height;

    local_transform.set_translation_xyz(s_w / 2.0, s_h / 2.0, 0.0);
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handler,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(components::Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    //loading spritesheet
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "res/textures/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "res/textures/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
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
                (*load).store(true, Ordering::Relaxed);
                println!("Loaded!");
                loaded
            }else{
                Config::default()
            }
        }));
        println!("Started loading");
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>> ) -> SimpleTrans{
        if (*self.loading).load(Ordering::Relaxed) {
            let loaded = self.load_thread.take().unwrap().join().expect("Error encountered while joining thread");
            println!("Loaded config: {:?}", loaded);
            data.world.insert(loaded);
            Trans::Switch(Box::new(PlayState::default()))
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
        println!("Entering play state..");
        let world = data.world;
        
        self.ball_spawn_timer.replace(1.0);

        //manual register because no Systems use the Paddle Component
        world.register::<components::Ball>();


        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            }else{
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }
}