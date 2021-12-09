use array2d::Array2D;
use itertools::{chain, Itertools};
use std::fs::File;
use std::io::Read;
use std::iter::once;

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

fn get_near(arr: &Array2D<u8>, (x, y): (usize, usize)) -> Box<[u8]> {
    let mut it: Box<dyn Iterator<Item = _>> = {
        let it = [(x + 1, y), (x, y + 1)].into_iter();
        if x > 0 && y > 0 {
            Box::new(chain(it, [(x - 1, y), (x, y - 1)]))
        } else if x > 0 {
            Box::new(chain(it, once((x - 1, y))))
        } else if y > 0 {
            Box::new(chain(it, once((x, y - 1))))
        } else {
            Box::new(it)
        }
    };

    it.map(|(x, y)| arr.get(x, y))
        .filter_map(|x| x)
        .cloned()
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

pub fn solution_part1(mut data: File) -> u32 {
    let data = format(&mut data);

    (0..100)
        .cartesian_product(0..100)
        .map(|pos| (data[pos], get_near(&data, pos)))
        .filter(|(height, near)| near.iter().all(|x| height < x))
        .map(|(x, _)| (x+1) as u32)
        .sum()
}
