use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();

    // Read t
    line.clear();
    input.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        // Read n
        line.clear();
        input.read_line(&mut line).unwrap();
        let n: usize = line.trim().parse().unwrap();

        // Read the line containing 2*n timestamps
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();

        let mut a: Vec<i64> = Vec::with_capacity(2 * n);
        for _ in 0..(2 * n) {
            a.push(it.next().unwrap().parse::<i64>().unwrap());
        }
        let mut prefix = vec![0i64; 2 * n + 1];
        // alternating prefix: +a[0] -a[1] +a[2] -a[3] ...
        let mut prefix_alt = vec![0i64; 2 * n + 1];

        for i in 0..2 * n {
            prefix[i + 1] = prefix[i] + a[i];
            if i % 2 == 0 {
                //cause we add then substract in middle sum
                prefix_alt[i + 1] = prefix_alt[i] - a[i];
            } else {
                prefix_alt[i + 1] = prefix_alt[i] + a[i];
            }
        }
        for k in 1..=n {
            let base = prefix[2 * n] - prefix[2 * n - k] - prefix[k];
            //          But the middle contribution you want for each k is:
            //
            // -a[k] + a[k+1] - a[k+2] - ... (it GLOBAL PATTERN of prefix_alt sum always starts with a plus at a[k])
            // in midlle should start with + only but we calculatte in a patter - and then + so
            // alternatively we change signs
            //
            let raw = prefix_alt[2 * n - k] - prefix_alt[k];
            let mid_actual = if k % 2 == 0 { -raw } else { raw };
            let ans = base + mid_actual;
            out.push_str(&format!("{} ", ans));
        }
        out.push('\n');
    }
    print!("{out}");
}
