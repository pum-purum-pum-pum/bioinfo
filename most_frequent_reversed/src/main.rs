use std::{
    fs::File,
    io::{BufWriter, Write},
    collections::{HashMap, HashSet},
    io::prelude::*,
};

#[macro_use] 
extern crate itertools;
use itertools::Itertools;

fn hamming(a: &str, b: &str) -> usize {
    let mut res = 0;
    for (i, j) in a.chars().zip(b.chars()) {
        if i != j {
            res += 1
        }
    };
    return res;
}

fn count_occurences(genome: &str, pattern: &str, d: usize) -> usize {
    let mut res = 0usize;
    for i in 0..genome.len() - pattern.len() + 1 {
        if hamming(&genome[i..i + pattern.len()], &pattern) <= d {
            res += 1;
        }
    };
    res
}

fn complement(genome: &str) -> String {
    let mut complement_table = HashMap::new();
    complement_table.insert('A', 'T');
    complement_table.insert('C', 'G');
    complement_table.insert('G', 'C');
    complement_table.insert('T', 'A');
    let mut result = String::new();
    for i in genome.chars() {
        result.push(*complement_table.get(&i).unwrap());
    }
    return result;
}

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let is_separator = |c| c == ' ' || c == '\n';
    let mut tokens = input.split(is_separator);
    let genome = tokens.next().unwrap();
    let genome_rev_complement = complement(&genome).chars().rev().collect::<String>();
    let mut next_usize = || -> usize {
        tokens.next().unwrap().parse().unwrap()
    };
    let k = next_usize();
    let d = next_usize();
    let alphabet_str = "ACGT";
    let mut alphabet = alphabet_str.chars();
    let mut count_to_str: HashMap<usize, Vec<String>> = HashMap::new();
    for pattern_char_vec in (0..k)
                        .map(|_| alphabet.clone())
                        .multi_cartesian_product() {
        let pattern: String = pattern_char_vec.iter().collect();
        // let rev_pattern: String = pattern_char_vec.iter().rev().collect();
        // eprintln!("{:?} {:?}", pattern, rev_pattern);
        let cnt = count_occurences(genome, &pattern, d) + count_occurences(&genome_rev_complement, &pattern, d);
        match count_to_str.get_mut(&cnt) {
            Some(patterns) => {
                patterns.push(pattern);
            }
            None => {
                count_to_str.insert(cnt, vec![pattern]);
            }
        }   
    };
    let res = &count_to_str[count_to_str.keys().max().unwrap()];
    for i in res {
        print!("{} ", i);
    }
    println!();
    // eprintln!("{:?}", count_to_str);
    Ok(())
}
