use bitvec::prelude::*;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct BitArray {
    data: Vec<BitVec>,
}

impl BitArray {
    fn num_cols(&self) -> usize {
        if self.data.len() > 0 {
            self.data[0].len()
        } else {
            0
        }
    }

    fn num_rows(&self) -> usize {
        self.data.len()
    }

    fn columns<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = bool> + 'a> {
        let (cols, rows) = (self.num_cols(), self.num_rows());
        (0..cols).map(move |col| (0..rows).map(move |row| self.data[row][col]))
    }

    fn rows<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = bool> + 'a> {
        self.data
            .iter()
            .map(move |row| row.iter().map(|reftype| *reftype))
    }
}

fn format(data: &mut impl Read) -> BitArray {
    let data = {
        let mut s = String::new();
        data.read_to_string(&mut s);
        s
    }
    .split('\n')
    .map(|s| {
        s.chars()
            .rev()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!("Bad data"),
            })
            .collect::<BitVec>()
    })
    .collect::<Vec<_>>();
    BitArray { data }
}

pub fn solution_part1(mut f: File) -> (u16, u16) {
    let data = format(&mut f);
    let half_len = data.num_rows() / 2;

    let gamma = data
        .columns()
        .map(|it| {
            let ones = it.filter(|x| *x).fold(0, |acc, _| acc + 1);
            ones >= half_len
        })
        // .inspect(|b| if *b { print!("1") } else { print!("0") })
        .enumerate()
        .fold(0, |acc, (i, b)| acc + (if b { 1 } else { 0 } << i));
    let epsilon = !gamma & u16::pow(2, 12) - 1;

    (gamma, epsilon)
}
