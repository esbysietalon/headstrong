use amethyst::ecs::prelude::{Component, VecStorage};

macro_rules! sign {
    ($x:expr) => {
        if $x > 0 {
            1
        } else if $x < 0 {
            -1
        } else {
            0
        }
    };
}


pub struct Physical{
    taken_space: Vec<(i32, i32)>,
    lines: Vec<((i32, i32), (i32, i32))>,
}

impl Physical{
    pub fn new(lines: Vec<((i32, i32), (i32, i32))>) -> Physical {
        let given_taken_space = Physical::generate_taken_space(lines.clone());
        Physical {
            taken_space: given_taken_space,
            lines,
        }
    }
    fn generate_taken_space(lines: Vec<((i32, i32), (i32, i32))>) -> Vec<(i32, i32)>{
        let mut first = true;
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        for ((x0, y0), (x1, y1)) in &lines {
            if first {
                x_min = if *x0 < *x1 { *x0 } else { *x1 };
                x_max = if *x0 < *x1 { *x1 } else { *x0 };
                y_min = if *y0 < *y1 { *y0 } else { *y1 };
                y_max = if *y0 < *y1 { *y1 } else { *y0 };
                first = false;
            }
            if *x0 < x_min {
                x_min = *x0;
            }
            if *x1 < x_min {
                x_min = *x1;
            }
            if *x0 < x_max {
                x_max = *x0;
            }
            if *x1 < x_max {
                x_max = *x1;
            }
            if *y0 < y_min {
                y_min = *y0;
            }
            if *y1 < y_min {
                y_min = *y1;
            }
            if *y0 < y_max {
                y_max = *y0;
            }
            if *y1 < y_max {
                y_max = *y1;
            }
        }
        let mut out_taken_space = Vec::new();
        let mut inside = false;
        let mut last_sign = 0;
        for y in y_min..y_max+1{
            for x in x_min..x_max+1 { 
                let mut xprodprod:i32 = 1;
                for ((x0, y0), (x1, y1)) in &lines {
                    let xl = *x1 - *x0;
                    let yl = *y1 - *y0;
                    xprodprod *= xl * y - x * yl;
                }
 
                if last_sign == 0 {
                    last_sign = sign!(xprodprod);
                }else{
                    let new_sign = sign!(xprodprod);
                    if new_sign != last_sign {
                        inside = !inside;
                    }
                    last_sign = new_sign;
                }

                if inside {
                    out_taken_space.push((x, y));
                }
            }
        }
        out_taken_space
    }
    pub fn set_lines(&mut self, lines: Vec<((i32, i32), (i32, i32))>) {
        self.lines = lines;
    }
    pub fn modify_lines(&mut self, lines: Vec<((i32, i32), (i32, i32))>, mod_arg: &str) {
        if mod_arg == "add" {
            //TODO add lines that are not yet in self.lines
            
        } else if mod_arg == "remove" {
            //TODO removes lines that are in self.lines
            
        } else {
            self.lines = lines;
        }
        
    }
    pub fn get_taken_space(&self, pos: (f32, f32)) -> Vec<(i32, i32)> {
        let (x, y) = pos;
        let x = x as i32;
        let y = y as i32;
        let mut out = Vec::new();
        for (i, j) in self.taken_space.clone() {
            out.push((i + x, j + y));
        }
        out
    }
         
}

impl Component for Physical{
    type Storage = VecStorage<Self>;
}