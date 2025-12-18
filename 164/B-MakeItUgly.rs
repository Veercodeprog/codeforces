use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    let mut out = String::new();
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let n: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let a: Vec<i64> = input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let all_same = a
            .first()
            .map_or(true, |&first| a.iter().all(|&x| x == first));
        if all_same || a.len() == 1 {
            out.push_str("-1\n");
            continue;
        }

        let even_elements = if n % 2 == 0 { true } else { false };

        if even_elements && (a[n - 2] != a[n - 1] || a[0] != a[1]) {
            out.push_str("-1\n");
            continue;
        };
        let can_beautiful = true;
        let mut an = true;
        for i in 1..n - 1 {
            if a[i + 1] != a[i - 1] {
                an = false;
                break;
            }
        }
        if an {
            let mut l = 0;
            let mut r = 0;
            let al = a[0];
            let ar = a[n - 1];
            while a[l] == al {
                l += 1;
            }
            while a[n - 1 - r] == ar {
                r += 1;
            }
            let ans = l.min(r);
            out.push_str(&format!("{ans}\n"));
        } else {
            let ans = -1;
            out.push_str(&format!("{ans}\n"));
        }

        //can beautiful is possible only if for 2..n-1 loop a
    }
    print!("{}", out);
}
