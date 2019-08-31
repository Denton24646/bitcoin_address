use std::io::Write;
use std::str;

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

#[test]
fn it_concatenates() {
    let point_x = String::from("0x50863AD64A87AE8A2FE83C1AF1A8403CB53F53E486D8511DAD8A04887E5B2352");
    let point_y = String::from("0x2CD470243453A299FA9E77237716103ABC11A1DF38855ED6F2EE187E9C582BA6");
    println!("X = {}", point_x);
    println!("Y = {}", point_y);


    let p = Point {
        x: (*point_x.as_bytes()).to_vec(),
        y: (*point_y.as_bytes()).to_vec(),
    };

    let result:[u8; 65] = p.concatenate();
    let r = str::from_utf8(&result).unwrap();
    assert_eq!(r, "40x50863AD64A87AE8A2FE83C1AF1A8400x2CD470243453A299FA9E7723771610")
}
