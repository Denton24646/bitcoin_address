use std::io::Write;

pub struct Point {
    pub x: Vec<u8>,
    pub y: Vec<u8>,
}

pub trait Concat {
    fn concatenate(&self) -> [u8; 65];
}

impl Concat for Point {
    // concatenate x and y points
    // add one byte prefix equal to 4 per convention
    fn concatenate(&self) -> [u8; 65] {
        let mut z: [u8; 65] = [0; 65];
        (&mut z[0..1]).write(&[b'4']).unwrap();
        (&mut z[1..]).write(&self.x[..]).unwrap();
        (&mut z[33..]).write(&self.y[..]).unwrap();
        z
    }
}
