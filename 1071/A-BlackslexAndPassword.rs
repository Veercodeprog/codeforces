use std::io::{self, BufRead};
fn main() {
    let mut line = String::new();

    let stdin = io::stdin();
    let mut input = stdin.lock();
    input.read_line(&mut line).unwrap();
    let t = line.trim().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();
        let k: usize = it.next().unwrap().parse().unwrap();
        let x: usize = it.next().unwrap().parse().unwrap();

        let ans = (k * x) + 1;
        out.push_str(&format!("{}\n", ans));
    }
    print!("{}", out);
}
