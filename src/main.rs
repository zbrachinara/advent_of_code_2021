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
    // let s = read_file("data/day_3");

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

    // println!("solution to day 4 part 1: {:?}", day_4::solution_part1(&mut File::open("data/day_4").unwrap()));
    // println!("solution to day 4 part 2: {:?}", day_4::solution_part2(&mut File::open("data/day_4").unwrap()));

    // println!("solution to day 5 part 1: {:?}", day_5::solution_part1(&mut File::open("data/day_5").unwrap()));
    // println!("solution to day 5 part 2: {:?}", day_5::solution_part2(&mut File::open("data/day_5").unwrap()));
    // println!("solution to day 6 part 1: {:?}", day_6::solution_part1(&mut File::open("data/day_6").unwrap()));
    // println!("solution to day 6 part 2: {:?}", day_6::solution_part2(&mut File::open("data/day_6").unwrap()));
}
