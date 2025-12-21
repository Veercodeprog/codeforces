//
use std::collections::HashSet;
use std::io::{self, BufRead};

pub struct WebOfLies {
    //1 indexed , contains all friends of i
    adj: Vec<HashSet<usize>>,
    higher_cnt: Vec<i32>,
    remain: i32,
}

impl WebOfLies {
    fn new(n: usize) -> Self {
        Self {
            adj: (0..=n).map(|_| HashSet::new()).collect(),
            higher_cnt: vec![0; n + 1],
            remain: n as i32,
        }
    }
    #[inline]
    fn dec_remain_if_zero(&mut self, i: usize) {
        if self.higher_cnt[i] == 0 {
            self.remain -= 1;
        }
    }

    #[inline]
    fn inc_remain_if_zero(&mut self, i: usize) {
        if self.higher_cnt[i] == 0 {
            self.remain += 1;
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].insert(v);
        self.adj[v].insert(u);
        let a = u.min(v);
        self.dec_remain_if_zero(a);
        self.higher_cnt[a] += 1;
    }
    pub fn remove_edge(&mut self, u: usize, v: usize) {
        self.adj[u].remove(&v);
        self.adj[v].remove(&u);

        let a = u.min(v);
        self.higher_cnt[a] -= 1;
        self.inc_remain_if_zero(a);
    }
    pub fn remaining_count(&self) -> i32 {
        self.remain
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let mut it = line.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut ds = WebOfLies::new(n);

    for _ in 0..m {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        ds.add_edge(u, v);
    }
    line.clear();
    input.read_line(&mut line).unwrap();
    let q: usize = line.split_whitespace().next().unwrap().parse().unwrap();

    let mut out = String::new();
    for _ in 0..q {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut it = line.split_whitespace();

        let t: i32 = it.next().unwrap().parse().unwrap();
        if t == 1 {
            let u: usize = it.next().unwrap().parse().unwrap();
            let v: usize = it.next().unwrap().parse().unwrap();
            ds.add_edge(u, v);
        } else if t == 2 {
            let u: usize = it.next().unwrap().parse().unwrap();
            let v: usize = it.next().unwrap().parse().unwrap();
            ds.remove_edge(u, v);
        } else {
            out.push_str(&format!("{}\n", ds.remaining_count()));
        }
    }

    print!("{out}");
}
