use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let c: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;

        let mut a: Vec<i128> = (0..n)
            .map(|_| it.next().unwrap().parse::<i64>().unwrap() as i128)
            .collect();
        a.sort_unstable(); // ascending

        let mut mul: i128 = 1;
        let mut coins: i64 = 0;

        while !a.is_empty() {
            let limit = c / mul; // max original weight that is currently free
                                 // find rightmost index with a[idx] <= limit
            let mut idx: Option<usize> = None;
            for i in (0..a.len()).rev() {
                if a[i] <= limit {
                    idx = Some(i);
                    break;
                }
            }

            if let Some(i) = idx {
                a.remove(i); // destroy largest free
            } else {
                a.pop(); // destroy largest (paid)
                coins += 1;
            }

            mul *= 2;
        }

        out.push_str(&format!("{coins}\n"));
    }

    print!("{out}");
}
