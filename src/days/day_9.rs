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

fn kern((x, y): (usize, usize)) -> Box<dyn Iterator<Item = (usize, usize)>> {
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
}

fn get_near(arr: &Array2D<u8>, (x, y): (usize, usize)) -> Box<[u8]> {
    let it = kern((x, y));

    it.map(|(x, y)| arr.get(x, y))
        .filter_map(|x| x)
        .cloned()
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn find_near(arr: &mut Array2D<u8>, pos: (usize, usize)) -> u32 {
    if let Some(val) = arr.get(pos.0, pos.1).cloned() {
        if val == 9 {
            0
        } else {
            arr[pos] = 9;
            kern(pos).map(|pos| find_near(arr, pos)).sum::<u32>() + 1
        }
    } else {
        0
    }

    // todo!()
}

pub fn solution_part1(mut data: File) -> u32 {
    let data = format(&mut data);

    (0..100)
        .cartesian_product(0..100)
        .map(|pos| (data[pos], get_near(&data, pos)))
        .filter(|(height, near)| near.iter().all(|x| height < x))
        .map(|(x, _)| (x + 1) as u32)
        .sum()
}

pub fn solution_part2(mut data: File) -> (u32, u32, u32) {
    let mut data = format(&mut data);

    let mut sizes = (0..100)
        .cartesian_product(0..100)
        .map(|pos| find_near(&mut data, pos))
        .filter(|x| *x != 0)
        .collect::<Vec<_>>();

    sizes.sort_unstable_by(|a, b| a.cmp(b).reverse());
    (sizes[0], sizes[1], sizes[2])
}
