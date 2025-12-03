pub type Input = Vec<Vec<u8>>;

pub fn parse(input: String) -> Input {
    input
        .lines()
        .map(|bank| bank.chars().map(|battery| battery as u8 - b'0').collect())
        .collect()
}
