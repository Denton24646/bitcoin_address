fn main() {
    //println!("Hello, world!");
}

struct Point {
    x: [byte; 32],
    y: [byte; 32]
}

pub trait Concat {
    fn concatenate(&self) -> [byte; 64];
}

impl Concat for Point {
    // concatenate x and y points
    // add one byte prefix equal to 4 per convention
    fn concatenate(&self) -> [byte; 64] {
        let mut z: [byte; 65];
        z[0] = 4;
        z[1..32] = p.x;
        z[33..] = p.y;
        return z
    }
}
