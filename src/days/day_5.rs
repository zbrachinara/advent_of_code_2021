use itermore::IterMore;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Point(i32, i32);

const MAX: i32 = 1000;

fn between(check: i32, left: i32, right: i32) -> bool {
    if left > right {
        right <= check && check <= left
    } else {
        left <= check && check <= right
    }
}

fn format(r: &mut impl Read) -> BTreeMap<Point, Point> {
    let data = {
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        s
    };

    data.split_whitespace()
        .filter(|s| *s != "->")
        .map(|s| s.split(','))
        .flatten()
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .chunks::<4>()
        .map(|points| (Point(points[0], points[1]), Point(points[2], points[3])))
        .collect::<BTreeMap<_, _>>()
}

pub fn solution_part1(f: &mut File) -> u32 {
    let map = format(f);

    // remove the elements that aren't horizontal or vertical
    let map = map
        .into_iter()
        .filter(|(Point(a, b), Point(c, d))| a == c || b == d)
        .inspect(|points| println!("{:?}", points))
        .collect::<BTreeMap<_, _>>();

    let mut count = 0;
    (0..MAX).for_each(|x| {
        (0..MAX).for_each(|y| {
            if let Some(points) = map
                .iter()
                .filter(|(Point(a, b), Point(c, d))| between(x, *a, *c) && between(y, *b, *d))
                .nth(1)
            {
                // println!("{:?}", points);
                count += 1;
            }
        })
    });

    count
}
