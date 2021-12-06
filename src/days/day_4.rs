use array2d::Array2D;
use std::io::{BufRead, Read};

struct Game {
    rules: Vec<u32>,
    boards: Vec<Board>,
}

struct Board {
    data: Array2D<u32>,
}

fn format(mut file: impl Read + BufRead) -> Game {
    let mut buf = Vec::new();
    file.read_until(b'\n', &mut buf);

    let rules = String::from_utf8(buf)
        .unwrap()
        .split(", ")
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", rules);

    todo!()
}
