use std::io::{self, Read};
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 1 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
fn is_power_of_two(x: u64) -> bool {
    x != 1 && (x & (x - 1)) == 0
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    let mut out = String::new();
    for _ in 1..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        let n: u64 = it.next().unwrap().parse().unwrap();
        let m: u64 = it.next().unwrap().parse().unwrap();

        let r = n % m;
        if r == 1 {
            out.push_str("1\n");
            continue;
        }
        let g = gcd(r, m);
        let a = r / g;
        let den = m / g;
        if !is_power_of_two(den) {
            out.push_str("0\n");
            continue;
        }
        let pop = a.count_ones() as u64;
        let ans = pop * m - r;
        out.push_str(&format!("{ans}\n"));
    }
    print!("{out}");
}
