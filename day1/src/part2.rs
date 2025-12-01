use super::parse_input;

pub fn solve() -> usize {
    let input = parse_input();

    let mut answer = 0;
    let mut dial = 50;
    for instrution in input {
        let step = if instrution > 0 { 1 } else { -1 };
        for _ in 0..instrution.abs() {
            dial = (dial + step) % 100;
            if dial == 0 {
                answer += 1;
            }
        }
    }
    answer
}
