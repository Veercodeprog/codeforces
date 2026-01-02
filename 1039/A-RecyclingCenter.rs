use std::io::{self, BufRead};
fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut input = stdin.lock();
    input.read_line(&mut line).unwrap();
    let t = line.trim().parse::<usize>().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();
        let n: usize = it.next().unwrap().parse().unwrap();
        let c: usize = it.next().unwrap().parse().unwrap();
        line.clear();
        input.read_line(&mut line).unwrap();

        let mut a: Vec<i128> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        a.sort_unstable();
        let mut coins = 0i64;
        for i in (0..a.len()).rev() {
            if a[i] > c as i128 {
                coins += 1;
                a.pop();
            } else {
                a.pop();
                for x in a.iter_mut() {
                    *x = *x * 2;
                }
            }
        }
        out.push_str(&format!("{}\n", coins));
    }
    print!("{out}");
}
