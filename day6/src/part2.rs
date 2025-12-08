use std::mem;

use crate::common::Operation;

pub fn solve(input: &str) -> String {
    let mut input = input.lines();
    let iter_length = input.clone().count();

    let mut numbers: Vec<Vec<[Option<u8>; 3]>> = Vec::new();
    for line in input.by_ref().take(iter_length - 1) {
        let mut row = Vec::new();
        let mut chunk = [None; 3];
        for (i, slot) in line.chars().enumerate() {
            if (i + 1) % 4 == 0 {
                row.push(mem::take(&mut chunk));
            }
            let num = if slot == ' ' {
                None
            } else {
                Some(slot as u8 - b'0')
            };
            chunk[i % 4 % 3] = num;
        }
        row.push(chunk);
        numbers.push(row);
    }

    let operations: Vec<Operation> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|operation| match operation {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!(),
        })
        .collect();

    let digits = numbers.len();
    let mut answer: u128 = 0;
    for (i, operation) in operations.into_iter().enumerate() {
        let mut alien_numbers: Vec<u16> = vec![];
        for col in 0..3 {
            let mut pow = 1;
            for row in (0..digits).rev() {
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
