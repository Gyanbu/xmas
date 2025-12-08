use std::mem;

use crate::common::Operation;

pub fn solve(input: &str) -> String {
    let mut input = input.lines().rev();

    let mut buf = (Operation::Add, 0);
    let mut operations: Vec<(Operation, usize)> = Vec::new();
    for c in input.by_ref().next().unwrap().chars() {
        if ['+', '*'].contains(&c) {
            if buf.1 > 0 {
                buf.1 -= 1;
                operations.push(buf);
                buf.1 = 0;
            }
            buf.0 = match c {
                '+' => Operation::Add,
                '*' => Operation::Multiply,
                _ => unreachable!(),
            };
        }
        buf.1 += 1
    }
    operations.push(buf);

    let mut numbers: Vec<Vec<Vec<Option<u8>>>> = Vec::new();
    for line in input.rev() {
        let mut row = Vec::new();
        let mut chunk = Vec::new();
        let mut chars = line.chars();
        for digits in operations.iter().map(|tuple| tuple.1) {
            for _ in 0..digits {
                let slot = chars.next().unwrap();
                let num = if slot == ' ' {
                    None
                } else {
                    Some(slot as u8 - b'0')
                };
                chunk.push(num);
            }
            row.push(mem::take(&mut chunk));
            chars.next();
        }
        numbers.push(row);
    }

    let mut answer: u128 = 0;
    for (i, (operation, digits)) in operations.into_iter().enumerate() {
        let mut alien_numbers: Vec<u16> = vec![0; digits];
        for col in 0..digits {
            let mut pow = 1;
            for row in (0..numbers.len()).rev() {
                if numbers[row][i][col].is_none() {
                    continue;
                }
                alien_numbers[col] += numbers[row][i][col].unwrap() as u16 * pow;
                pow *= 10;
            }
        }
        let acc = alien_numbers[0] as u128;
        answer += alien_numbers
            .into_iter()
            .skip(1)
            .fold(acc, |acc, x| operation.calc(acc, x as u128));
    }
    answer.to_string()
}
