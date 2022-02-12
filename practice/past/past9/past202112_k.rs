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

#[fastout]
fn main() {
    input!(n: usize, m: usize, q: usize, k: usize, al: [Usize1; k]);
    let mut gl = vec![vec![]; n];
    for _ in 0..m {
        input! {u: Usize1, v: Usize1}
        gl[u].push((v, 1));
        gl[v].push((u, 1));
    }
    let dists = (0..k).map(|i| dijkstra(al[i], n, &gl)).collect::<Vec<_>>();
    for _ in 0..q {
        input! {s: Usize1, t :Usize1}
        let ans = (0..k).map(|i| dists[i][s] + dists[i][t]).min().unwrap();
        println!("{}", ans);
    }
}
