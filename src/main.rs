use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;

mod days;
use days::*;

fn main() {

    let mut data = Vec::new();
    File::open("data/day_1").unwrap().read_to_end(&mut data).unwrap();
    let s = String::from_utf8(data).unwrap();


    println!("solution to day 1 part 1: {:?}", day_1::solution_part1(day_1::format(&s).borrow()));
    println!("solution to day 1 part 2: {:?}", day_1::solution_part2(day_1::format(&s).borrow()));
}
