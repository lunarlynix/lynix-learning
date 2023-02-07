

mod new_module;
use std::io::{self, Write};

use new_module::prelude::*;
use serde::{Serialize};
use serde_json::to_string;

#[derive(Serialize)]
pub struct Test {
    pub name: String,
    pub number: i32,
    pub number2: u8,
}

fn main() {
    hello();

    // Input from User Console in Rust
    let mut input = String::new();
    print!("Please input something: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", to_string(&Test{name: input, number: 200, number2: 3}).unwrap());

    // Foreach in Rust
    let mut v = vec![1u8, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}