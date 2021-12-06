fn format(data: &str) -> Box<[u16]> {
    data.split('\n')
        .map(|num| u16::from_str_radix(num, 2).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

pub fn solution_part1(data: &str) -> (u16, u16) {
    let nums = format(data);
    let data_size = nums.len();
    let mut frequency = [0; 16];

    nums.iter().for_each(|num| {
        // iterates over each bit in num,
        // and if it is set, increment that entry in frequency
        frequency.iter_mut().enumerate().for_each(|(i, entry)| {
            *entry += (num >> i) % 2;
        });
    });

    let gamma = frequency.iter().enumerate().fold(0, |acc, (i, bit)| {
        if usize::from(*bit) >= data_size / 2 {
            acc + (1 << i)
        } else {
            acc
        }
    });
    let epsilon = !gamma & (0..=11).fold(0, |acc, bit| acc | (1 << bit));

    (gamma, epsilon)
}
