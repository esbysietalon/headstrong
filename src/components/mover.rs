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
    pub jerk: f32,
    pos_goals: PriorityQueue<(i32, i32), Priority>,
    move_vec: Vec<(i32, i32)>,
    pub session: u32,
}

impl Mover{
    pub fn new(width: f32, height: f32) -> Mover {
        Mover {
            width,
            height,
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            jerk: 5.0,
            pos_goals: PriorityQueue::new(),
            move_vec: Vec::new(),
            session: 0,
        }
    }
    pub fn diff_move_vec(&self, path: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut out = Vec::new();
        //println!("curr vec is {:?}", self.move_vec);
        //println!("diff vec is {:?}", path);
        for pos in path {
            let mut in_move_vec = false;
            for opos in self.move_vec.clone() {
                if *pos == opos {
                    in_move_vec = true;
                    break;
                }
            }
            if !in_move_vec {
                out.push(pos.clone());
            }
        }
        out
    }
    pub fn is_move_vec_empty(&self) -> bool {
        self.move_vec.is_empty()
    }
    pub fn set_move_vec(&mut self, vec: Vec<(i32, i32)>) {
        //println!("setting move vec to {:?}", vec);
        self.move_vec = vec;
    }
    pub fn get_move(&self) -> Option<(i32, i32)> {
        if self.move_vec.is_empty() {
            None
        }else{
            Some(self.move_vec[0])
        }
    }
    pub fn pop_move(&mut self) {
        self.move_vec.remove(0);
    }
    pub fn add_goal(&mut self, goal: (i32, i32), ord: Priority){
        self.pos_goals.push(goal, ord);
    }
    pub fn get_goal(&self) -> Option<(i32, i32)> {
        let goal = self.pos_goals.peek();

        match goal {
            None => None,
            Some(((x, y), _)) => Some((*x, *y)),
        }
    }
    pub fn pop_goal(&mut self) {
        self.pos_goals.pop();
    }
         
}

impl Component for Mover{
    type Storage = VecStorage<Self>;
}