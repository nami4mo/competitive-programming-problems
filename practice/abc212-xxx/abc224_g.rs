#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        a: usize,
        b: usize,
    }
    let mut ok = t;
    let mut ng = 0;
    let bnf = (b * n) as f64;
    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        let thre = (t - mid) * a;
        let target_area = t - mid;
        let exp_warp = bnf / target_area as f64;
        // let exp_move = a * (target_area * (target_area - 1) / 2) / target_area;
        let exp_move = a * (target_area - 1) / 2;
        let exp = exp_move as f64 + exp_warp;
        if thre as f64 <= exp {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if ok <= s && s <= t {
        let d = t - s;
        let ans = d * a;
        println!("{}", ans);
    } else {
        let target_area = t - ok + 1;
        let exp_warp = bnf / target_area as f64;
        let exp_move = a * (target_area - 1) / 2;
        let ans = exp_move as f64 + exp_warp;
        println!("{}", ans);
    }

    // println!("{}", ok);
}
