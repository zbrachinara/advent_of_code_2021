use itermore::IterMore;

pub fn format(data: &str) -> Box<[u32]> {
    data.split("\n")
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

pub fn solution_part1(data: &[u32]) -> u32 {
    data.windows(2)
        .fold(
            0,
            |acc, arr| {
                if arr[0] < arr[1] {
                    acc + 1
                } else {
                    acc
                }
            },
        )
}

pub fn solution_part2(data: &[u32]) -> u32 {
    data.windows(3)
        .map(|arr| arr.iter().sum::<u32>())
        .windows::<2>()
        .fold(0, |acc, arr| {
            if arr[0] < arr[1] {
                acc + 1
            } else {
                acc
            }
        })
}
