#![allow(dead_code)]

use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod days;
use days::*;

fn read_file<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let mut data = Vec::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    String::from_utf8(data).unwrap()
}

fn main() {
    let s = read_file("data/day_3");

    // println!(
    //     "solution to day 1 part 1: {:?}",
    //     day_1::solution_part1(day_1::format(&s).borrow())
    // );
    // println!(
    //     "solution to day 1 part 2: {:?}",
    //     day_1::solution_part2(day_1::format(&s).borrow())
    // );

    // println!(
    //     "solution to day 2 part 1: {:?}",
    //     day_2::solution_part1(day_2::format(&s).borrow())
    // );
    // println!(
    //     "solution to day 2 part 2: {:?}",
    //     day_2::solution_part2(day_2::format(&s).borrow())
    // );

    println!("solution to day 3 part 1: {:?}", day_3::solution_part1(&s));
    println!("solution to day 3 part 2: {:?}", day_3::solution_part2(&s));
}
