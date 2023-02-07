

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

    // Foreach in Rust
    let mut v = vec![1u8, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}