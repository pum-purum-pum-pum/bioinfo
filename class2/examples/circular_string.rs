use std::{
    fs::File,
    io::{BufWriter, Write},
    collections::{HashMap, HashSet},
    io::prelude::*,
    iter::FromIterator,
};

pub fn find_euler_path(
    num_nodes: usize,
    mut v: usize,
    edges: & Vec<(usize, usize)>,
    deg: & Vec<usize>,
    in_deg: & Vec<usize>,
    out_deg: & Vec<usize>,
    res: &mut Vec<usize>,
) {
    let mut free_nodes: HashSet<(usize, usize)> = HashSet::from_iter(edges.iter().map(|x| *x));
    for u in 0..num_nodes {
        if out_deg[u] > in_deg[u] {
            v  = u;
            break;
        }
    };
    let mut history = vec!();
    history.push(v);
    while history.len() > 0 {
        let w = *history.last().unwrap();
        for u in 0..num_nodes {
            if free_nodes.get(&(w, u)).is_some() {
                history.push(u);
                free_nodes.remove(&(w, u));
                break;
            }
        }
        if w == *history.last().unwrap() {
            history.pop();
            res.push(w);
        }
    }
}

#[derive(Default)]
pub struct Generator{
    res: Vec<String>,
}
impl Generator {
    pub fn generate_strs(&mut self, cur_str: String, k: usize) {
        if cur_str.len() == k {
            // eprintln!("{:?}", cur_str);
            self.res.push(cur_str);
            return;
        }
        self.generate_strs(cur_str.clone() + "0", k);
        self.generate_strs(cur_str.clone() + "1", k);
    }

    pub fn get_kmers(&mut self) -> Vec<String> {
        return self.res.clone();
    }
}

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let is_separator = |c| c == ' ' || c == '\n';
    let mut tokens = input.split(is_separator);
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut generator = Generator{res: vec!()};
    generator.generate_strs("".to_string(), k);
    let mut kmers = generator.get_kmers();
    // eprintln!("{:?}", kmers);
    let pref = |kmer: &str| {kmer[..kmer.len() - 1].to_string()};
    let suf = |kmer: &str| {kmer[1..].to_string()};
    // loop {
    //     match tokens.next() {
    //         Some(kmer) => {
    //             kmers.push(kmer);
    //         },
    //         None => {
    //             break
    //         }
    //     }
    // };

    let (nodes, node_to_id) = { // * construct pref+suf nodes
        let mut nodes = vec!();
        for kmer in kmers.iter() {
            nodes.push(pref(kmer));
            nodes.push(suf(kmer));
        };
        // * glue nodes
        nodes.sort(); 
        nodes.dedup(); 
        let mut node_to_id = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            node_to_id.insert(node.to_string(), i);
        }
        (nodes, node_to_id)
    };
    let (edges, deg, in_deg, out_deg) = { // * construct edges
        let mut deg = vec![0usize; nodes.len()];
        let mut in_deg = vec![0usize; nodes.len()];
        let mut out_deg = vec![0usize; nodes.len()];
        let mut edges = vec!();
        for kmer in kmers.iter() {
            let start_id = node_to_id.get(&pref(kmer)).unwrap();
            let end_id = node_to_id.get(&suf(kmer)).unwrap();
            out_deg[*start_id] += 1;
            in_deg[*end_id] += 1;
            deg[*start_id] += 1;
            deg[*end_id] += 1;
            edges.push((*start_id, *end_id))
        }
        (edges, deg, in_deg, out_deg)
    };
    // eprintln!("{:?}", edges);
    // eprintln!("{:?}", deg);
    let mut path: Vec<usize> = vec!();
    find_euler_path(nodes.len(), 0usize, &edges, &deg, &in_deg, &out_deg, &mut path);
    let mut res = String::new();
    let path: Vec<&usize> = path.iter().rev().collect();
    let mut iter = path.iter();
    res += &nodes[**iter.next().unwrap()];
    for i in  iter {
        let cur_node = &nodes[**i];
        res += &cur_node[cur_node.len() - 1..];
        eprintln!("{:?}", res);
        eprintln!("{:?}", &cur_node[cur_node.len() - 1..]);
        // eprintln!("{:?}", nodes[*i]);
    }
    eprintln!("----");
    eprintln!("{:?}", &res[..res.len() - k + 1]);
    // eprintln!("{:?}", edges);
    // eprintln!("{:?}", nodes);
    // eprintln!("{:?}", kmers);
    Ok(())
}