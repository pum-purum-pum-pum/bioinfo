use std::{fs::File, io::{BufWriter, Write}, io::prelude::*,};

fn is_seq(a: i32, b: i32) -> bool {
    a < b && b - a == 1 && a >= 0 && b >= 0
}

fn is_break(a: i32, b: i32) -> bool {
    if is_seq(a, b) || is_seq(-b, -a) {
        return false;
    }
    true
}

fn main() -> Result<(), String> {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let is_separator = |c| c == ' ' || c == '\n' || c == '(' || c == ')';
    let tokens = input.split(is_separator);
    let mut p = vec!();
    p.push(0);
    for i in tokens {
        if i == "" {continue};
        p.push(i.parse::<i32>().unwrap());
    }
    p.push(p.len() as i32);
    let file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(&file);

    let mut res = 0;
    for i in 0..p.len() - 1 {
        if is_break(p[i], p[i + 1]) {
            res += 1;
        }
    }
    println!("{}", res);
    Ok(())
}