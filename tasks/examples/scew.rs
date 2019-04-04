
use std::{
    fs::File,
    io::prelude::*,
};

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut skew = 0;
    let mut pref_skew = vec!(skew);
    for i in input.chars() {
        match i {
            'C' => skew -= 1,
            'G' => skew += 1,
            _ => ()
        }
        pref_skew.push(skew);
    };
    let min_skew = pref_skew.iter().min().unwrap();
    eprintln!("{:?}", min_skew);
    let mut res = vec!();
    for (id, value) in pref_skew.iter().enumerate() {
        if value == min_skew {
            res.push(id) 
        }
    };
    eprintln!("hello there");
    for i in res.iter() {
        print!("{} ", i);
    }
    println!();

    Ok(())
}
