use svg::node::element::path::{Parameters, Data};
use glam::*;

pub trait ToParameters {
    fn from(&self) -> Parameters;
}

impl ToParameters for Vec2 {
    fn from(&self)->Parameters {
        Parameters::from((self.x, self.y))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct OrderedPair{
    p1x : i32,
    p1y : i32,
    p2x : i32,
    p2y : i32,
}

impl OrderedPair{
    pub fn new(p1: Vec2, p2: Vec2) -> OrderedPair{
        let p1x = (p1.x * 1000_f32) as i32;
        let p1y = (p1.y * 1000_f32) as i32;
        let p2x = (p2.x * 1000_f32) as i32;
        let p2y = (p2.y * 1000_f32) as i32;

        if p1x > p2x {
            return OrderedPair {p1x:p2x, p1y:p2y, p2x:p1x, p2y: p1y};
        }
        else if p1x == p2x {
            if p1y > p2y {
                return OrderedPair {p1x:p2x, p1y:p2y, p2x:p1x, p2y: p1y};
            }
        }

        return OrderedPair {p1x:p1x, p1y:p1y, p2x:p2x, p2y: p2y};
    }
}

pub enum Instruction {
    MoveTo(Vec2),
    LineTo(Vec2),
}

impl Instruction {
    fn execute_instruction(&self, data: Data) -> Data {
        match self {
            Instruction::MoveTo(position) => {
                data.move_to(ToParameters::from(position))
            },

            Instruction::LineTo(position) => {
                data.line_to(ToParameters::from(position))
            },
        }
    }
}

pub fn print_circle_to_instructions(position : Vec2, radius : f32, nb_vertice: i32, instructions : &mut Vec<Instruction>) {
    let co = (std::f32::consts::TAU / nb_vertice as f32).cos();
    let si = (std::f32::consts::TAU / nb_vertice as f32).sin();
    let mut x = 0_f32;
    let mut y = 1_f32;
    instructions.push(Instruction::MoveTo(position + Vec2::new(x, y) * radius));
    for _ in 0..nb_vertice {
        let nx = x * co - y * si;
        let ny = x * si + y * co;
        
        x = nx;
        y = ny;
        instructions.push(Instruction::LineTo(position + Vec2::new(x, y) * radius));
    }
}

pub fn smooth(positions : Vec<Vec2>, nb_points: usize, sharpness: f32) -> Vec<Vec2> {
    let mut smoothed = Vec::new();
    if positions.len() < 2 {
        return positions;
    }
    
    smoothed.push(positions[0]);
    
    for index in 1..(positions.len() - 1) {
        let p1 = positions[index - 1];
        let p2 = positions[index];
        let p3 = positions[index + 1];

        let p1 = (p1 + p2) / 2_f32;
        let p3 = (p3 + p2) / 2_f32;
        let p1 = p1 + (p2 - p1) * sharpness;
        let p3 = p3 + (p2 - p3) * sharpness;

        let dp12 = p2 - p1;
        let dp23 = p3 - p2;

        for pt in 0..nb_points {
            let s = pt as f32 / nb_points as f32;
            let l1 = p1 + dp12 * s;
            let l2 = p2 + dp23 * s;
            
            smoothed.push(l1 + (l2 - l1) * s);
        }
    }

    smoothed.push(positions[positions.len() - 1]);
    
    return smoothed;
}