#![allow(unused_imports)]
use std::collections::VecDeque;

use im_rc::HashSet;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {n: usize, es: [(Usize1, Usize1); n-1]}
    let mut gl = vec![vec![]; n];
    for &(u, v) in &es {
        gl[u].push(v);
        gl[v].push(u);
    }

    let mut vl = vec![(0, 0); n];
    fn dfs(
        node: usize,
        pare: usize,
        vl: &mut Vec<(usize, usize)>,
        gl: &Vec<Vec<usize>>,
        mut curr: usize,
    ) -> usize {
        // dbg!(node);
        if gl[node].len() == 1 && node != 0 {
            vl[node] = (curr, curr);
            return curr + 1;
        }
        let (mut left, mut right) = (9999999, 0);
        for &neib in &gl[node] {
            if neib == pare {
                continue;
            }
            let new_curr = dfs(neib, node, vl, gl, curr);
            curr = new_curr;
            left = vl[neib].0.min(left);
            right = vl[neib].1.max(right);
        }
        vl[node] = (left, right);
        curr
    }
    dfs(0, !0, &mut vl, &gl, 1);
    for (a, b) in vl {
        println!("{} {}", a, b);
    }
}
