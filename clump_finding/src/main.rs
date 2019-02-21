#![feature(slice_concat_ext)]

use std::{
    fs::File,
    io::{BufWriter, Write},
    collections::{HashMap, HashSet},
    io::prelude::*,
};
use std::slice::SliceConcatExt;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let is_separator = |c| c == ' ' || c == '\n';
    let mut tokens = input.split(is_separator);
    let genome = tokens.next().unwrap();
    let mut next_usize = || -> usize {
        tokens.next().unwrap().parse().unwrap()
    };
    let k = next_usize();
    let l = next_usize();
    let t = next_usize();
    let mut k_mers: HashMap<String, usize> = HashMap::new();

    pub fn add(
        counter: &mut HashMap<String, usize>,
        value: &str
    ) {
        match counter.get_mut(value) {
            Some(value) => {
                *value = *value + 1;
            }
            None => {
                counter.insert(value.to_string(), 1);
            }
        };
    }

    pub fn remove(
        counter: &mut HashMap<String, usize>,
        value: &str
    ) {
        match counter.get_mut(value) {
            Some(value) => {
                *value = *value - 1;
            }
            None => ()
        }
    }

    pub fn check_and_save(
        counter: &HashMap<String, usize>,
        value: &str,
        storage: &mut HashSet<String>,
        desired_value: usize,
    ) {
        match counter.get(value) {
            Some(count) => {
                if desired_value == *count {
                    storage.insert(value.to_string());
                }
            }
            None => ()
        }
    }

    // * calc first l-sized string
    for i in 0..l - k + 1 {
        let k_mer = &genome[i..i + k];
        add(&mut k_mers, k_mer);
    }
    // *
    let mut result: HashSet<String> = HashSet::new();
    for k_mer in k_mers.keys() {
        check_and_save(& k_mers, k_mer, &mut result, t);
    }

    // * sliding window of lenght l
    for i in 1..genome.len() - l + 1 {
        let left_remove = &genome[i - 1..i + k - 1];
        let right_add = &genome[i + l - k..i + l];
        add(&mut k_mers, right_add);
        remove(&mut k_mers, left_remove);
        check_and_save(&mut k_mers, right_add, &mut result, t);
        check_and_save(&mut k_mers, left_remove, &mut result, t);
    }
    
    let mut file = File::create("foo.txt")?;
    let mut writer = BufWriter::new(&file);
    let result_list: Vec<String> = result.into_iter().collect();
    writeln!(writer, "{:}", result_list.join(" "));
    Ok(())
}