use svg::node::element::path::Parameters;
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