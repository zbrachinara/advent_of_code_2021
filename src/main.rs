#![allow(dead_code)]

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
    // println!("solution to day 7 part 1: {:?}", day_7::solution_part1(&mut File::open("data/day_7").unwrap()));
    // println!("solution to day 7 part 2: {:?}", day_7::solution_part2(&mut File::open("data/day_7").unwrap()));
    // println!("solution to day 3 part 1: {:?}", day_3::solution_part1(File::open("data/day_3").unwrap()));
    // println!("solution to day 3 part 2: {:?}", day_3::solution_part2(File::open("data/day_3").unwrap()));

    // println!("solution to day 8 part 1: {:?}", day_8::solution_part1(File::open("data/day_8").unwrap()));

    // println!("solution to day 9 part 1: {:?}", day_9::solution_part1(File::open("data/day_9").unwrap()));
    // println!("solution to day 9 part 2: {:?}", day_9::solution_part2(File::open("data/day_9").unwrap()));

    println!("solution to day 10 part 1: {:?}", day_10::solution_part1(File::open("data/day_10").unwrap()));
}
