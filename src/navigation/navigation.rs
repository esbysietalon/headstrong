use priority_queue::PriorityQueue;
use crate::navigation::successors;
use crate::components::{Id, Physical};
use crate::game_state::Map;

pub struct InputData {
    id: Id, 
    obj: Physical, 
    map: Map,
}

pub fn find_path(origin: (i32, i32), end: (i32, i32), d: InputData) -> Vec<(i32, i32)> {
    let mut path = Vec::new();
    let mut curr = origin;

    let mut frontier = PriorityQueue::new();
    frontier.push(curr, 0);

    while !frontier.is_empty() {
        successors(&frontier.pop().unwrap().0, &d.id, &d.obj, &d.map);    
    }

    

    path
}
