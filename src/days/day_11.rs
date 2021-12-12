use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};
use derive_more::{Deref, DerefMut};
// use array2d::Array2D;
use itertools::Itertools;

#[derive(Debug)]
struct Vec2D<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl <T> Vec2D<T> {
    pub fn from_rows(data: Vec<Vec<T>>) -> Self {
        assert_ne!(data.len(), 0);
        assert_ne!(data[0].len(), 0);

        let rows = data.len();
        let cols = data[0].len();

        Vec2D {
            data: data.into_iter().map(|vec| vec.into_iter()).flatten().collect_vec(),
            rows,
            cols,
        }

    }

    pub fn iter_mut_row_major(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    fn flatten_coords(&self, (x, y): (usize, usize)) -> Option<usize> {
        if x < self.rows && y < self.cols {
            Some(y * self.cols + x)
        } else {
            None
        }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        if let Some(index) = self.flatten_coords(index) {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
        if let Some(index) = self.flatten_coords(index) {
            Some(&mut self.data[index])
        } else {
            None
        }
    }
}

impl <T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl <T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[derive(Deref, DerefMut, Debug)]
struct State(Vec2D<u8>);

impl State {

   fn increase_light(&mut self) {
        self.iter_mut_row_major().for_each(|squid| *squid += 1);
   }

}

fn format(data: &mut dyn Read) -> State {
    let arr = {
        let mut s = String::new();
        data.read_to_string(&mut s).unwrap();
        s
    }.split_whitespace()
        .map(|s| s.chars().map(|c| u8::from_str_radix(&String::from(c), 10).unwrap()).collect_vec()).collect_vec();

    State(Vec2D::from_rows(arr))
}

pub fn solution_part1(mut f: File) -> u32 {
    let mut state = format(&mut f);

    state.increase_light();

    todo!()
}