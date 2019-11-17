use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    ecs::prelude::{Join, Read, Write, ReadStorage, System, SystemData, WriteStorage},
};
use crate::game_state::{Config, Map};
use crate::components::{Physical, Id};

pub struct PhysicalSystem;

impl<'s> System<'s> for PhysicalSystem{
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Physical>,
        ReadStorage<'s, Id>,
        Read<'s, Config>,
        Write<'s, Map>,
    );

    fn run(&mut self, (transforms, objs, ids, config, mut map): Self::SystemData) {
        //TODO read objs and transforms and apply to map
        map.storage = vec![(0.0, Id::nil()); (map.width * map.height) as usize];
        for (transform, obj, id) in (&transforms, &objs, &ids).join() {
            let occupado = obj.get_taken_space((transform.translation().x, transform.translation().y));
            
            for (x, y) in occupado {
                if x < 0 || y < 0 || x >= map.width as i32 || y >= map.height as i32 {
                    continue;
                }
                let index = (x + y * (map.width as i32)) as usize;
                
                map.storage[index] = (1.0, id.clone());
            }
        }    
    }
}
