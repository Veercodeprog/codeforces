use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    let mut output = String::new();
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let n: i64 = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let a: Vec<i64> = input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let ans = a.iter().max().unwrap();
        output.push_str(&format!("{}\n", ans));
    }

    print!("{}", output);
}
