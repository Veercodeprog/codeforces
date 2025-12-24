use std::io::{self, BufRead};

struct ProblemBank {
    counts: [usize; 7],
}

impl ProblemBank {
    fn new(s: &str) -> Self {
        let mut counts = [0usize; 7];
        for c in s.chars() {
            let idx = (c as u8 - b'A') as usize;
            counts[idx] == 1;
        }
        ProblemBank { counts }
    }

    fn missing_problems(&self, rounds: usize) -> usize {
        self.counts
            .iter()
            .map(|&count| (rounds as isize - count as isize).max(0) as usize)
            .sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();

    // Read t
    input.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let first_line: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = first_line[0];
        let m = first_line[1];

        line.clear();
        input.read_line(&mut line).unwrap();
        let s = line.trim();

        let bank = ProblemBank::new(s);
        let answer = bank.missing_problems(m);
        println!("{}", answer);
    }
}
