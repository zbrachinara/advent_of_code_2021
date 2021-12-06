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

fn mcb(data: &[u16], bit: u8) -> u8 {
    // returns 1 or 0
    let half_size = data.len() / 2;
    let bitmask = 1 << bit;
    let ones = data
        .iter()
        .fold(0, |acc, num| if num & bitmask > 0 { acc + 1 } else { acc });

    match ones {
        x if x >= half_size => 1,
        x if x < half_size => 0,
        _ => unreachable!(),
    }
}

fn lcb(data: &[u16], bit: u8) -> u8 {
    let half_size = data.len() / 2;
    let bitmask = 1 << bit;
    let ones = data
        .iter()
        .fold(0, |acc, num| if num & bitmask > 0 { acc + 1 } else { acc });

    match ones {
        x if x < half_size => 1,
        x if x >= half_size => 0,
        _ => unreachable!(),
    }
}

pub fn solution_part2(data: &str) -> (u16, u16) {
    let data = format(data);

    // calculate oxygen level
    let oxygen = {
        let mut data = Vec::from(data.clone());
        let mut bit = 12;
        while data.len() > 1 {
            bit -= 1;
            let mcb = u16::from(mcb(data.as_slice(), bit));
            data.retain(|byte| {
                (byte & (1 << bit)) == mcb << bit
            });
            println!("{:?}", data);
        }

        data[0]
    };

    let co2 = {
        let mut data = Vec::from(data.clone());
        let mut bit = 12;
        while data.len() > 1 {
            bit -= 1;
            let lcb = u16::from(lcb(data.as_slice(), bit));
            data.retain(|byte| {
                (byte & (1 << bit)) == lcb << bit
            });
            println!("{:?}", data);
        }

        data[0]
    };

    (oxygen, co2)
}
