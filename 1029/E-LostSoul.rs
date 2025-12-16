use std::collections::HashSet;
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
        let a_input: Vec<i64> = input
            .split_whitespace()
            .take(n)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b_input: Vec<i64> = input
            .split_whitespace()
            .take(n)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut a = vec![0; n + 1];
        let mut b = vec![0; n + 1];
        for i in 0..n {
            a[i + 1] = a_input[i];
            b[i + 1] = b_input[i];
        }
        let mut suffix_a1 = HashSet::new();
        let mut suffix_b1 = HashSet::new();

        let mut suffix_a2 = HashSet::new();

        let mut suffix_b2 = HashSet::new();

        let mut ans = 0;
        for i in (1..=n).rev() {
            let mut can_match = false;
            if a[i] == b[i] {
                can_match = true;
            }

            if !can_match {
                if suffix_a1.contains(&a[i]) || suffix_b1.contains(&b[i]) {
                    can_match = true;
                }
            } // Case 3: with removal (j >= i+2)
            if !can_match {
                if suffix_b2.contains(&a[i]) || suffix_a2.contains(&b[i]) {
                    can_match = true;
                }
            } // if !can_match && i + 2 <= n {
              //     for j in (i + 2)..=n {
              //         if a[i] == b[j] || b[i] == a[j] {
              //             can_match = true;
              //             break;
              //         }
              //     }
              // }
            if can_match {
                ans = i;
                break;
            }
            suffix_a1.insert(a[i]);
            suffix_b1.insert(b[i]);
            if i + 1 <= n {
                suffix_a2.insert(a[i + 1]);
                suffix_b2.insert(b[i + 1]);
            }
        }
        println!("{}", ans);
    }
}
