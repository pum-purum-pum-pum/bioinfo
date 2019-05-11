use std::{fs::File, io::{BufWriter, Write}, io::prelude::*,};

fn reversal(ar: &mut [i32], from: usize, to: usize) {
    ar[from..to + 1].reverse();
    for val in ar[from..to + 1].iter_mut() {*val *= -1;}
}

fn print_perm(ar: &[i32], writer: &mut std::io::BufWriter<&std::fs::File>) {
    let sign = |i: &i32| { if *i > 0 {"+"} else {"-"} };
    let mut iter = ar.iter();
    let i = iter.next().unwrap();
    write!(writer, "({}{}", sign(i), i.abs()).unwrap();
    for i in iter { write!(writer, " {}{}", sign(i), i.abs()).unwrap(); }
    writeln!(writer, ")").unwrap();
}

fn main() -> Result<(), String> {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let is_separator = |c| c == ' ' || c == '\n' || c == '(' || c == ')';
    let tokens = input.split(is_separator);
    let mut p = vec!();
    for i in tokens {
        if i == "" {continue};
        p.push(i.parse::<i32>().unwrap());
    }
    let file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(&file);
    for i in 0..p.len() {
        let index = p.iter().position(|v| v.abs() == i as i32 + 1).unwrap();
        if i == index {
            if p[i] < 0 { 
                p[i] *= -1;
                print_perm(&p, &mut writer);
            }
            continue
        };
        reversal(&mut p, i, index);
        print_perm(&p, &mut writer);
        if p[i] < 0 { 
            p[i] *= -1;
            print_perm(&p, &mut writer);
        }
    }
    Ok(())
}