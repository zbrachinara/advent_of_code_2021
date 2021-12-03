#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

pub struct Instruction {
    direction: Direction,
    quantity: u32,
}

pub fn format(data: &str) -> Box<[Instruction]> {
    data.split("\n")
        .map(|instruction| {
            let mut parts = instruction.split(' ');

            let direction = match parts.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Incorrect type found"),
            };
            let quantity = u32::from_str_radix(&parts.next().unwrap(), 10).unwrap();

            Instruction {
                direction,
                quantity,
            }
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

pub fn solution_part1(data: &[Instruction]) -> (u32, u32) {
    let mut position = 0;
    let mut depth = 0;

    data.iter().for_each(
        |Instruction {
             direction,
             quantity,
         }| {
            match direction {
                Direction::Forward => position += quantity,
                Direction::Up => depth -= quantity,
                Direction::Down => depth += quantity,
            }
        },
    );

    (position, depth)
}
