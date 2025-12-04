pub type Input = Vec<Vec<bool>>;

pub fn parse(input: String) -> Input {
    input
        .lines()
        .map(|row| row.chars().map(|c| c == '@').collect())
        .collect()
}
