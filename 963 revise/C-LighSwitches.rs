use std::io::{self, Read};

fn main() {
    // Read entire stdin at once (fast for Codeforces).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    // Split by whitespace to parse ints easily.
    let mut it = s.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap(); // number of test cases
    let mut out = String::new(); // collect outputs and print once

    for _ in 0..t {
        // n rooms, k minutes ON then k minutes OFF
        let n: usize = it.next().unwrap().parse().unwrap();
        let k: i64 = it.next().unwrap().parse().unwrap();

        // Each room's light pattern repeats every 2k minutes.
        let p: i64 = 2 * k;

        // Read installation times a[i]
        let mut a: Vec<i64> = Vec::with_capacity(n);
        let mut max_a: i64 = 0; // last installation time (earliest possible answer >= this)
        for _ in 0..n {
            let x: i64 = it.next().unwrap().parse().unwrap();
            if x > max_a {
                max_a = x;
            }
            a.push(x);
        }

        // cnt[r] = how many installation times have remainder r modulo p (= 2k)
        // We'll use this to check if there exists a remainder r where ALL rooms are ON.
        let mut cnt = vec![0i64; p as usize];
        for &x in &a {
            // Make modulo positive (Rust % can be negative if x were negative; safe anyway)
            let r = ((x % p) + p) % p;
            cnt[r as usize] += 1;
        }

        // We need to handle circular intervals on [0, p-1].
        // Trick: duplicate the array so that circular windows become normal ranges.
        let mut cnt2 = vec![0i64; (2 * p) as usize];
        for i in 0..(2 * p) as usize {
            cnt2[i] = cnt[i % (p as usize)];
        }

        // Prefix sums for cnt2 so any range sum can be computed in O(1).
        // pref[i] = sum of cnt2[0..i-1]
        let mut pref = vec![0i64; cnt2.len() + 1];
        for i in 0..cnt2.len() {
            pref[i + 1] = pref[i] + cnt2[i];
        }

        // Remainder of max_a modulo p.
        // Later, if we choose target remainder r, we can compute the smallest s >= max_a with s % p == r.
        let max_r = ((max_a % p) + p) % p;

        // best will store the minimum valid time s, if it exists.
        let mut best: Option<i64> = None;

        // Try every possible remainder r in [0, p-1] as candidate remainder for answer time s.
        for r in 0..p {
            // For a given target remainder r (meaning s % p == r),
            // a room is ON at that remainder iff its start remainder z = a[i] % p lies in:
            // [r - k + 1, r] on the circle (length k window).
            //
            // We evaluate that window using the duplicated array by shifting by +p.
            let end = r + p; // inclusive end index in the duplicated domain
            let start = end - k + 1; // inclusive start index

            // Range sum on [start, end] inclusive:
            let sum = pref[(end as usize) + 1] - pref[start as usize];

            // If sum == n, then every room's start remainder is inside that k-length window,
            // meaning all rooms are ON at remainder r.
            if sum == n as i64 {
                // Compute earliest s >= max_a such that s % p == r.
                let delta = (r - max_r + p) % p; // how far to move forward from max_a
                let s_ans = max_a + delta;

                // Keep the minimum answer across all valid r.
                best = match best {
                    None => Some(s_ans),
                    Some(cur) => Some(cur.min(s_ans)),
                };
            }
        }

        // Output the answer for this test case.
        match best {
            Some(ans) => out.push_str(&format!("{ans}\n")),
            None => out.push_str("-1\n"),
        }
    }

    // Print all answers at once.
    print!("{out}");
}
