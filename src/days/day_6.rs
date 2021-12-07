use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct State([u64; 9]);
impl State {
    fn from_slice(slice: &[u8]) -> Self {
        let mut state = [0; 9];
        slice.iter().for_each(|age| {
            state[*age as usize] += 1;
        });
        State(state)
    }

    fn step(&self) -> Self {
        let [s0, s1, s2, s3, s4, s5, s6, s7, s8] = self.0;
        Self([ s1, s2, s3, s4, s5, s6, s0 + s7, s8, s0 ])
    }

    fn number(&self) -> u64 {
        self.0.iter().sum()
    }
}

fn format(f: &mut impl Read) -> Vec<u8> {
    {
        let mut s = String::new();
        f.read_to_string(&mut s);
        s
    }
    .split(',')
    .map(|s| u8::from_str_radix(s.trim(), 10).unwrap())
    .collect()
}


pub fn solution_part1(f: &mut File) -> u64 {
    let ages = format(f);
    let mut state = State::from_slice(&ages);

    (0..80).for_each(|_| {
        state = state.step()
    });

    state.number()
}

pub fn solution_part2(f: &mut File) -> u64 {
    let ages = format(f);
    let mut state = State::from_slice(&ages);

    (0..256).for_each(|_| {
        state = state.step()
    });

    state.number()
}
