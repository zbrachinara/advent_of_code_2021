use std::fs::File;
use std::io::Read;

fn format(f: &mut impl Read) -> Vec<u32> {
    {
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s
    }
    .split(',')
    .map(|s| u32::from_str_radix(s.trim(), 10).unwrap())
    .collect()
}

fn abs_diff(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn deviation(data: &[u32], point: u32) -> u32 {
    data.iter().map(|n| abs_diff(point, *n)).sum()
}

fn standard_deviation(data: &[u32], point: u32) -> u32 {
    data.iter().map(|n| {
        let x = abs_diff(point, *n);
        x * (x + 1) / 2
    }).sum()
}

pub fn solution_part1(f: &mut File) -> u32 {
    let mut vec = format(f);
    let len = vec.len();

    let (_, out, _) = vec.as_mut_slice().select_nth_unstable(len / 2);
    let out_copied = *out;
    deviation(vec.as_slice(), out_copied)
}

pub fn solution_part2(f: &mut File) -> (u32, u32) {
    let mut vec = format(f);
    let pos = vec.iter().sum::<u32>() as f32 / vec.len() as f32;

    let answers = (pos.floor() as u32, pos.ceil() as u32);
    (
        standard_deviation(vec.as_slice(), answers.0),
        standard_deviation(vec.as_slice(), answers.1),
    )
}
