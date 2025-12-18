use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut out = String::new();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        let mut a = vec![vec![0i64; m]; n];

        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
        }

        let mut winnings: i64 = 0;

        for col in 0..m {
            let mut v = Vec::with_capacity(n);
            for row in 0..n {
                v.push(a[row][col]);
            }

            v.sort();

            let mut pref = 0i64;
            for (i, &x) in v.iter().enumerate() {
                winnings += x * i as i64 - pref;
                pref += x;
            }
        }

        out.push_str(&format!("{}\n", winnings));
    }

    print!("{}", out);
}
