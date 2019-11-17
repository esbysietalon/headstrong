use priority_queue::PriorityQueue;
use crate::navigation::successors;
use crate::components::{Id, Physical};
use crate::game_state::Map;

pub struct InputData {
    pub id: Id, 
    pub obj: Physical, 
    pub map: Map,
}

pub fn find_path(origin: (i32, i32), end: (i32, i32), d: InputData) -> Option<(Vec<(i32, i32)>, u32)> {
    let mut path = Vec::new();
    let mut curr = origin;

    let mut frontier = PriorityQueue::new();
    frontier.push(curr, 0);

    while !frontier.is_empty() {
        successors(&frontier.pop().unwrap().0, &d.id, &d.obj, &d.map);    
    }

    

    Some((path, 0))
}
