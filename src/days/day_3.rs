fn format(data: &str) -> Box<[u16]> {
    data.split('\n')
        .map(|num| u16::from_str_radix(num, 2).unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
