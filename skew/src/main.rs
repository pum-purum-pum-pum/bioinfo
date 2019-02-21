
use std::{
    fs::File,
    io::{BufWriter, Write},
    collections::{HashMap, HashSet},
    io::prelude::*,
};

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let skew = 0;
    for i in &input {

    }

    Ok(())
}
