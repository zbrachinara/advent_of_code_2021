use std::fs::File;
use std::io::Read;

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

pub fn solution_part1(f: &mut File) -> u32 {
    println!("{:?}", format(f));
    todo!()
}
