use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut a: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        a.sort_unstable();
        let mn = a[0];
        let mut cnt = 0usize;

        for &x in &a {
            if x == mn {
                cnt += 1;
            } else {
                break;
            }
        }
        let ans = if cnt == 1 {
            true
        } else {
            let mut has_escape = false;
            for &x in &a[cnt..] {
                if x % mn != 0 {
                    has_escape = true;
                    break;
                }
            }
            has_escape
        };
        out.push_str(if ans { "YES\n" } else { "NO\n" });
    }
    print!("{out}");
}
