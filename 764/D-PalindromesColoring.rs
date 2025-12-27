use std::collections::HashMap;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();
        let n: usize = it.next().unwrap().parse().unwrap();
        let k: usize = it.next().unwrap().parse().unwrap();
        line.clear();
        input.read_line(&mut line).unwrap();

        let s: String = line.trim().parse().unwrap();
        let mut freq: HashMap<char, usize> = HashMap::new();
        for ch in s.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }
        let pairs: usize = freq.values().map(|&v| v / 2).sum();
        let mut singles: usize = freq.values().map(|&v| v % 2).sum();
        // Distribute pairs as evenly as possible:
        let mut ans_length: usize = 2 * (pairs / k);
        let leftover_pairs: usize = pairs % k;
        singles += 2 * leftover_pairs;
        if singles >= k {
            ans_length += 1;
        }
        out.push_str(&format!("{}\n", ans_length));
    }
    print!("{}", out);
}
