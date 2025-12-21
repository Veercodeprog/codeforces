use std::io;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = parts[0];
    let m = parts[1];
    let q = parts[2];

    let g = gcd(n, m);
    let inner_per_group = n / g; // sectors per group in inner ring
    let outer_per_group = m / g; // sectors per group in outer ring
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let query: Vec<u64> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let sx = query[0];
        let sy = query[1];
        let ex = query[2];
        let ey = query[3];

        let start_group = if sx == 1 {
            (sy - 1) / inner_per_group
        } else {
            (sy - 1) / outer_per_group
        };

        let end_group = if ex == 2 {
            (ey - 1) / outer_per_group
        } else {
            (ey - 1) / inner_per_group
        };

        if start_group == end_group {
            println!("YES");
        } else {
            println!("No");
        }
    }
}
