#![allow(unused_imports, dead_code, unused_macros)]
use im_rc::{HashMap, HashSet};
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(start: usize, n: usize, gl: &Vec<Vec<(usize, u64)>>) -> Vec<u64> {
    const INF: u64 = 1e18 as u64;
    let mut d = vec![INF; n];
    d[start] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    while let Some(Reverse(v)) = heap.pop() {
        let (dist, i) = v;
        if d[i] < dist {
            continue;
        }
        for &(to, cost) in &gl[i] {
            if d[to] > d[i] + cost {
                d[to] = d[i] + cost;
                heap.push(Reverse((d[to], to)));
            }
        }
    }
    d
}
// #[fastout]
fn main() {
    input! {n: usize, m: usize}
    let mut f_st = HashSet::new();
    let mut abcl = vec![];
    for _ in 0..m {
        input! {a: Usize1, b: Usize1, c: u64}
        abcl.push((a, b, c));
        f_st.insert(a);
        f_st.insert(b);
    }
    f_st.insert(0);
    f_st.insert(n - 1);
    let mut fl = f_st.into_iter().collect::<Vec<_>>();
    fl.sort();
    let f2i = fl
        .iter()
        .enumerate()
        .map(|x| (*x.1, x.0))
        .collect::<HashMap<usize, usize>>();
    let nf = fl.len();
    let mut gl = vec![vec![]; nf];
    for &(a, b, c) in &abcl {
        gl[f2i[&a]].push((f2i[&b], c));
        gl[f2i[&b]].push((f2i[&a], c));
    }
    for i in 0..nf - 1 {
        let d = (fl[i + 1] - fl[i]) as u64;
        gl[i].push((i + 1, d));
        gl[i + 1].push((i, d));
    }
    // dbg!(&gl);
    let dists = dijkstra(0, nf, &gl);
    println!("{}", dists[nf - 1]);
}
