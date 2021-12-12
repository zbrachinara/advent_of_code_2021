use std::fs::File;
use std::io::Read;
use derive_more::Deref;
use array2d::Array2D;
use itertools::Itertools;

#[derive(Deref, Debug)]
struct State(Array2D<u8>);

fn format(data: &mut dyn Read) -> State {
    let arr = {
        let mut s = String::new();
        data.read_to_string(&mut s).unwrap();
        s
    }.split_whitespace()
        .map(|s| s.chars().map(|c| u8::from_str_radix(&String::from(c), 10).unwrap()).collect_vec()).collect_vec();

    State(Array2D::from_rows(arr.as_slice()))
}

pub fn solution_part1(mut f: File) -> u32 {
    dbg!(format(&mut f));

    todo!()
}