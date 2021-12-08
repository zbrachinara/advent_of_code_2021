use std::fmt::{Debug, Formatter};
use bitvec::prelude::*;
use derive_more::Deref;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::mem::{transmute, MaybeUninit};

#[repr(u8)]
#[derive(Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Segment {
    fn from(s: char) -> Self {
        match s {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => panic!("Invalid segment name"),
        }
    }
}

impl Into<u8> for Segment {
    fn into(self) -> u8 {
        unsafe { transmute(self) }
    }
}

#[derive(Deref)]
struct Signal(BitVec);

impl From<&str> for Signal {
    fn from(s: &str) -> Self {
        let mut segments = [false; 7];
        s.chars().map(Segment::from).for_each(|seg| {
            segments[Into::<u8>::into(seg) as usize] = true;
        });
        Self(segments.iter().collect())
    }
}

impl Debug for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0.as_raw_slice()[0])
    }
}

fn collect_signals(s: &str, signal_slice: &mut [MaybeUninit<Signal>]) {
    s.split_whitespace()
        .map(Signal::from)
        .enumerate()
        .for_each(|(index, signal)| {
            signal_slice[index].write(signal);
        });
}

fn format(f: &mut impl Read) -> Vec<([Signal; 10], [Signal; 4])> {
    {
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s
    }
    .split('\n')
    .map(|s| {
        let mut it = s.split('|');

        let mut observed: [MaybeUninit<Signal>; 10] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let mut output: [MaybeUninit<Signal>; 4] = unsafe { MaybeUninit::uninit().assume_init() };

        collect_signals(it.next().unwrap(), &mut observed);
        collect_signals(it.next().unwrap(), &mut output);

        unsafe { (transmute(observed), transmute(output)) }
    })
    .collect::<Vec<_>>()
}

pub fn solution_part1(mut f: File) {
    let data = format(&mut f);
    println!("{:?}", data);
}
