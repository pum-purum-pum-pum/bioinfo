use std::{
    fs::File,
    io::{BufWriter, Write},
    collections::{HashMap, HashSet},
    io::prelude::*,
};
// #[macro_use]
// extern crate lazy_static;

// #[macro_use] 
// extern crate itertools;
// use itertools::Itertools;

pub fn profile_from_motifs(motifs: &Vec<String>, k: usize) -> Vec<Vec<f32>> {
    let mut res = vec![vec![0f32; k], vec![0f32; k], vec![0f32; k], vec![0f32; k]];
    let mut motifs_iters = vec!();
    for motif in motifs.iter() {
        motifs_iters.push(motif.chars());
    }
    let p = 1f32 / motifs.len() as f32;
    for i in 0..k {
        // eprintln!("@@");
        for motif_id in 0..motifs_iters.len() {
            match motifs_iters[motif_id].next().unwrap() {
                'A' => {
                    res[0][i] += p;
                }
                'C' => {
                    res[1][i] += p;
                }
                'G' => {
                    res[2][i] += p;
                }
                'T' => {
                    res[3][i] += p;
                }
                _ => ()
            }
        }
    }
    return res;
}

pub fn get_prob(pattern: &str, profile: &Vec<Vec<f32>>) -> f32 {
    let mut res = 1f32;
    for (i, c) in pattern.chars().enumerate() {
        match c {
            'A' => {
                res *= profile[0][i];
            }
            'C' => {
                res *= profile[1][i];
            }
            'G' => {
                res *= profile[2][i];
            }
            'T' => {
                res *= profile[3][i];
            }
            _ => ()
        }
    }
    return res;
}

pub fn get_most_probable(genome: &str, k: usize, profile: &Vec<Vec<f32>>) -> String {
    let mut max_prob = 0f32;
    let mut result = genome[0..k].to_string();
    for i in 0..genome.len() - k {
        let current_prob = get_prob(&genome[i..i + k], profile);
        if max_prob < current_prob {
            max_prob = current_prob;
            result = genome[i..i + k].to_string();
        }
    }
    return result;
}

pub fn score(motifs: &Vec<String>, k: usize, t: usize) -> usize {
    let mut motifs_iterators = vec!();
    for motif in motifs.iter() {
        motifs_iterators.push(motif.chars());
    };
    let mut res = 0usize;
    for _ in 0..k {
        let mut A = 0usize;
        let mut C = 0usize;
        let mut G = 0usize;
        let mut T = 0usize;
        for i in 0..motifs_iterators.len() {
            match motifs_iterators[i].next().unwrap() {
                'A' => {
                    A += 1usize;
                }
                'C' => {
                    C += 1usize;
                }
                'G' => {
                    G += 1usize;
                }
                'T' => {
                    T += 1usize;
                }
                _ => ()
            }
        }
        res += t - A.max(C).max(G).max(T);
    }
    return res;
}

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let is_separator = |c| c == ' ' || c == '\n';
    let mut tokens = input.split(is_separator);
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let t: usize = tokens.next().unwrap().parse().unwrap();
    // let genome = tokens.next().unwrap();
    let mut best_motifs = vec!();
    let mut genomes = vec!();
    for i in 0..t {
        let genome = tokens.next().unwrap();
        genomes.push(genome);
        // eprintln!("{}", genome);
        best_motifs.push(genome[0..k].to_string());
    }
    // eprintln!("{:?}", best_motifs);
    // eprintln!("{:?}", genomes[0].len() - k);
    for first_id in 0..genomes[0].len() - k {
        // eprintln!("!");
        let mut motifs = vec!();
        motifs.push(genomes[0][first_id..first_id + k].to_string());
        for i in 1..t {
            // eprintln!("{:?} {:?}", k, motifs);
            let profile = profile_from_motifs(&motifs, k);
            motifs.push(get_most_probable(genomes[i], k, &profile));
        }
        // eprintln!("{:?} < {:?}", score(&motifs, k, t), score(&best_motifs, k, t)); 
        if score(&motifs, k, t) < score(&best_motifs, k, t) {
            best_motifs = motifs;
        }
    };
    for motif in best_motifs.iter() {
        println!("{}", motif);
    }
    // println!("{}", );
    Ok(())
}