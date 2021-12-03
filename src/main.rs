use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;

mod day_1;

fn main() {

    let mut data = Vec::new();
    File::open("data/day_1").unwrap().read_to_end(&mut data).unwrap();
    let s = String::from_utf8(data).unwrap();

    let out = day_1::solution(day_1::format(&s).borrow());

    println!("solution to day 1: {:?}", out);
}
