use amethyst::ecs::prelude::{Component, VecStorage};
use priority_queue::PriorityQueue;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Mover{
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
    pub acceleration: [f32; 2],
    move_goals: PriorityQueue<(i32, i32), Priority>,
}

impl Mover{
    pub fn new(width: f32, height: f32) -> Mover {
        Mover {
            width,
            height,
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            move_goals: PriorityQueue::new(),
        }
    }
    pub fn add_goal(&mut self, goal: (i32, i32), ord: Priority){
        self.move_goals.push(goal, ord);
    }
    pub fn get_goal(&self) -> Option<(f32, f32)> {
        let goal = self.move_goals.peek();

        match goal {
            None => None,
            Some(((x, y), _)) => Some((*x as f32, *y as f32)),
        }
    }
    pub fn pop_goal(&mut self) {
        self.move_goals.pop();
    }
         
}

impl Component for Mover{
    type Storage = VecStorage<Self>;
}