//
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut input = stdin.lock();
    input.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let L: usize = it.next().unwrap().parse().unwrap();
        let mut hurdles: Vec<(i64, i64)> = Vec::with_capacity(n);
        for _ in 0..n {
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();

            let l: i64 = it.next().unwrap().parse().unwrap();
            let r: i64 = it.next().unwrap().parse().unwrap();
            hurdles.push((l, r));
        }
        let mut ups: Vec<(i64, i64)> = Vec::with_capacity(m);
        for _ in 0..m {
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();
            let x: i64 = it.next().unwrap().parse().unwrap();
            let v: i64 = it.next().unwrap().parse().unwrap();
            ups.push((x, v));
        }
        let mut heap = BinaryHeap::<i64>::new();
        let mut cur_power: i64 = 1;
        let mut min_pow_ups_needed: i64 = 0;
        let mut possible = true;
        let mut j: usize = 0;

        for &(l, r) in &hurdles {
            while j < m && ups[j].0 < l {
                heap.push(ups[j].1);
                j += 1;
            }
            let need = r - l + 2;
            while cur_power < need {
                if let Some(v) = heap.pop() {
                    cur_power += v;
                    min_pow_ups_needed += 1;
                } else {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            out.push_str(&format!("{min_pow_ups_needed}\n"));
        } else {
            out.push_str("-1\n");
        }
    }
    print!("{}", out);
}
