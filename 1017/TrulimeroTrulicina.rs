use std::io::{self, Read};

fn main() {
    // Fast input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let k: usize = it.next().unwrap().parse().unwrap();

        let shift_rows = (m % k == 0);

        for i in 0..n {
            for j in 0..m {
                let jj = if shift_rows { (j + i) % m } else { j };
                // let jj = j;
                let idx = i * m + jj; // 0..n*m-1 (bijection)
                let val = (idx % k) + 1;

                if j > 0 {
                    out.push(' ');
                }
                out.push_str(&val.to_string());
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
