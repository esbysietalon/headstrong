use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::prelude::{Join, Read, Write, ReadStorage, System, SystemData, WriteStorage},
};
use crate::game_state::{Config, Map};
use crate::components::{Physical};

pub struct PhysicalSystem;

impl<'s> System<'s> for PhysicalSystem{
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Physical>,
        Read<'s, Config>,
        Write<'s, Map>,
    );

    fn run(&mut self, (transforms, objs, config, map): Self::SystemData) {
        //TODO read objs and transforms and apply to map
    }
}
