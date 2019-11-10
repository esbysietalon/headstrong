use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};
use crate::game_state::{Config};
use crate::components::{Mover};

use crate::components::Priority;

use angular::atan2;
use rand::Rng;

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Mover>,
        Read<'s, Config>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, movers, config, time): Self::SystemData) {
        for (mover, transform) in (&movers, &mut transforms).join(){
            //println!("x-vel: {} y-vel: {}", mover.velocity[0], mover.velocity[1]);
            transform.prepend_translation_x(mover.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(mover.velocity[1] * time.delta_seconds());
        }
    }
}

pub struct GoalSystem;

impl<'s> System<'s> for GoalSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Mover>,
        Read<'s, Config>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut movers, config, time): Self::SystemData) {
        for (mover, transform) in (&mut movers, &mut transforms).join(){
            let goal = mover.get_goal();
            let px = transform.translation().x;
            let py = transform.translation().y;
            match goal {
                None => {
                    mover.velocity[0] = 0.0;
                    mover.velocity[1] = 0.0;
                },
                Some((x, y)) => {
                    let use_x = x - px;
                    let use_y = y - py;
                    
                    let use_dist = use_x * use_x + use_y * use_y;
                    
                    if use_dist <= 4.0 {
                        mover.pop_goal();
                        mover.velocity[0] = 0.0;
                        mover.velocity[1] = 0.0;
                    }else{
                        let ang = atan2(use_y, use_x);

                        mover.velocity[0] = ang.cos() * time.delta_seconds() * 4000.0;
                        mover.velocity[1] = ang.sin() * time.delta_seconds() * 4000.0;
                    }
                },
            }
        }
    }
}

pub struct SimpleIdle;

impl<'s> System<'s> for SimpleIdle{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Mover>,
        Read<'s, Config>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut movers, config, time): Self::SystemData) {
        for (mover, transform) in (&mut movers, &mut transforms).join(){
            match mover.get_goal() {
                None => {
                    let mut rng = rand::thread_rng();
                    //println!("adding goal");
                    mover.add_goal((rng.gen_range(0.0, config.stage_width) as i32, rng.gen_range(0.0, config.stage_height) as i32), Priority::Low);
                },
                _ => {},
            }
        }
    }
}