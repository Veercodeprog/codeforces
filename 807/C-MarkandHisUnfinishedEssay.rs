use std::io::{self, BufRead};
fn main() {
    let mut line = String::new();
    let mut stdin = io::stdin();
    let mut input = stdin.lock();
    input.read_line(&mut line).unwrap();
    let t = line.trim().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace(); // inside: tied to this `line` [web:95]

        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let q: usize = it.next().unwrap().parse().unwrap();

        // line.clear();
        // input.read_line(&mut line).unwrap();
        line.clear();
        input.read_line(&mut line).unwrap();
        // let mut s = line.trim().to_string();
        let mut it = line.split_whitespace();
        let base_str = it.next().unwrap().as_bytes().to_vec();
        let mut left = vec![0i64; m + 1];
        let mut right = vec![0i64; m + 1];
        let mut prefix_len = vec![0i64; m + 1];
        prefix_len[0] = base_str.len() as i64;
        for op in 1..=m {
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();
            left[op] = it.next().unwrap().parse().unwrap();
            right[op] = it.next().unwrap().parse().unwrap();
            let added_len = right[op] - left[op] + 1;
            prefix_len[op] = prefix_len[op - 1] + added_len;
        }
        for i in 0..q {
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut it = line.split_whitespace();
            let mut pos: i64 = it.next().unwrap().parse().unwrap();

            // rewind from last operation to first
            for op in (1..=m).rev() {
                if pos > prefix_len[op - 1] {
                    let offset_in_appended = pos - prefix_len[op - 1]; // 1-indexed
                    pos = left[op] + offset_in_appended - 1;
                }
            }

            // now pos is in the original string
            let ch = base_str[(pos - 1) as usize] as char;
            out.push(ch);
            out.push('\n')
        }
    }
    print!("{out}");
}
