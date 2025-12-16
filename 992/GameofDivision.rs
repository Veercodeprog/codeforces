use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        let n: usize = it.next().unwrap().parse().unwrap();
        let k: i32 = it.next().unwrap().parse().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut cnt: Vec<i32> = vec![0i32; k as usize];
        for &x in &a {
            cnt[(x % k) as usize] += 1;
        }

        let mut ans: Option<usize> = None;
        for (idx, &x) in a.iter().enumerate() {
            if cnt[(x % k) as usize] == 1 {
                ans = Some(idx + 1);
                break;
            }
        }
        let mut out = String::new();
        if let Some(i) = ans {
            out.push_str("YES\n");
            out.push_str(&format!("{i}\n"));
        } else {
            out.push_str("NO\n");
        }
        print!("{out}");
    }
}
