use std::fmt::{Debug, Display};

pub type Input = Worksheet;

pub fn parse(input: String) -> Input {
    let mut input = input.lines();
    let iter_length = input.clone().count();

    let mut numbers: Vec<Vec<usize>> = Vec::new();
    for line in input.by_ref().take(iter_length - 1) {
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        numbers.push(row);
    }

    let mut operations: Vec<Operation> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|operation| match operation {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!(),
        })
        .collect();

    Worksheet {
        numbers,
        operations,
    }
}

pub enum Operation {
    Add,
    Multiply,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
        }
    }
}

impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Operation {
    pub fn calc(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
pub struct Worksheet {
    pub numbers: Vec<Vec<usize>>,
    pub operations: Vec<Operation>,
}
