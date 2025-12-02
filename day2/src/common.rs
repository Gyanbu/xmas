pub type Input = Vec<(u64, u64)>;

pub fn parse(input: String) -> Input {
    input
        .trim()
        .split(',')
        .map(|range| {
            let range = range.split_once('-').unwrap();
            (range.0.parse().unwrap(), range.1.parse().unwrap())
        })
        .collect()
}
