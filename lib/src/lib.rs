use std::{env, fs};

pub fn read_input() -> String {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap_or('.'.to_string());
    fs::read_to_string(format!("{}/input.txt", dir)).expect("input.txt not found!")
}
