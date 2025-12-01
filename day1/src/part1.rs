use super::parse_input;

pub fn solve() -> usize {
    let input = parse_input();

    let mut answer = 0;
    let mut dial = 50;
    for instrution in input {
        dial = (dial + instrution) % 100;
        if dial == 0 {
            answer += 1;
        }
    }
    answer
}
