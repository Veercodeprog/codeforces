use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<i64> = input
            .split_whitespace()
            .take(n)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b: Vec<i64> = input
            .split_whitespace()
            .take(n)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut ans = 0;
    }
}
