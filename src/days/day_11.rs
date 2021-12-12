use derive_more::{Deref, DerefMut};
use std::cell::RefMut;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use crate::vec2d::Vec2D;
use itertools::Itertools;


#[derive(Deref, DerefMut)]
struct State(Vec2D<u8>);

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for num in row {
                write!(f, "{} ", num)?
            }
            writeln!(f)?
        }

        Ok(())
    }
}

impl State {
    fn increase_light(&mut self) {
        self.iter_mut_row_major().for_each(|squid| *squid += 1);
    }

    fn single_flash(&mut self) {}
}

fn format(data: &mut dyn Read) -> State {
    let arr = {
        let mut s = String::new();
        data.read_to_string(&mut s).unwrap();
        s
    }
    .split_whitespace()
    .map(|s| {
        s.chars()
            .map(|c| u8::from_str_radix(&String::from(c), 10).unwrap())
            .collect_vec()
    })
    .collect_vec();

    State(Vec2D::from_rows(arr))
}

pub fn solution_part1(mut f: File) -> u32 {
    let mut state = format(&mut f);

    println!("{:?}", state);
    state.increase_light();
    println!("{:?}", state);

    todo!()
}
