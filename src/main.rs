use std::string::String;
use std::str;

use crate::point::Concat;

extern crate crypto;

mod point;
mod hash;

fn main() {
    let point_x = String::from("0x50863AD64A87AE8A2FE83C1AF1A8403CB53F53E486D8511DAD8A04887E5B2352");
    let point_y = String::from("0x2CD470243453A299FA9E77237716103ABC11A1DF38855ED6F2EE187E9C582BA6");
    println!("X = {}", point_x);
    println!("Y = {}", point_y);


    let p = point::Point {
        x: (*point_x.as_bytes()).to_vec(),
        y: (*point_y.as_bytes()).to_vec(),
    };

    let concat:[u8; 65] = p.concatenate();
    let c = str::from_utf8(&concat).unwrap();
    println!("concat = {}", c);

    let sha = hash::get_sha256(&concat);
    println!("sha = {}", sha);

    let ripemd = hash::get_ripemd(&sha);
    println!("ripemd = {}", ripemd);
}


