use array2d::Array2D;
use std::fs::File;
use std::io::Read;

fn format(data: &mut impl Read) -> Array2D<u8> {
    let data = {
        let mut s = String::new();
        data.read_to_string(&mut s);
        s
    }
    .split_whitespace()
    .map(|s| {
        s.chars()
            .map(|c| u8::from_str_radix(&String::from(c), 10).unwrap())
            .collect::<Vec<_>>()
    })
    // .flatten()
    .collect::<Vec<_>>();

    Array2D::from_rows(data.as_slice())
}

pub fn solution_part1(mut data: File) {
    let data = format(&mut data);


    println!("{:?}", data);
}
