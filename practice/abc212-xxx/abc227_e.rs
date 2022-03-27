#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

// #[fastout]
fn main() {
    input! {
        //
        s: Chars,
        k: usize,
    }
    let ss = s
        .iter()
        .map(|&c| match c {
            'K' => 0,
            'E' => 1,
            _ => 2,
        })
        .collect_vec();

    let n = ss.len();
    let cnts = (0..3)
        .map(|i| ss.iter().filter(|&&v| v == i).count())
        .collect_vec();

    let xmax = 450;
    let offset = 50;
    let mut dp = vec![vec![vec![vec![0; xmax + offset]; cnts[1] + 1]; cnts[0] + 1]; n + 1];
    // dp[i文字目まで][0の個数][1の個数][swap数] = これを満たす文字列の個数
    dp[0][0][0][0] = 1;

    for i in 0..n {
        for c0 in 0..cnts[0] + 1 {
            for c1 in 0..cnts[1] + 1 {
                if i < c0 + c1 {
                    break;
                }
                let c2 = i - c0 - c1;
                let mut used_c = vec![c0, c1, c2];
                let mut rem_s = vec![];
                for &si in &ss {
                    if used_c[si] == 0 {
                        rem_s.push(si);
                    } else {
                        used_c[si] -= 1;
                    }
                }
                let nexts = (0..3)
                    .map(|i| (0..rem_s.len()).find(|&ri| rem_s[ri] == i))
                    .collect_vec();
                for x in 0..xmax + 1 {
                    if nexts[0].is_some() {
                        dp[i + 1][c0 + 1][c1][x + nexts[0].unwrap()] += dp[i][c0][c1][x];
                    }
                    if nexts[1].is_some() {
                        dp[i + 1][c0][c1 + 1][x + nexts[1].unwrap()] += dp[i][c0][c1][x];
                    }
                    if nexts[2].is_some() {
                        dp[i + 1][c0][c1][x + nexts[2].unwrap()] += dp[i][c0][c1][x];
                    }
                }
            }
        }
    }
    let ans = (0..k.min(xmax) + 1)
        .map(|ki| dp[n][cnts[0]][cnts[1]][ki])
        .sum::<usize>();
    println!("{}", ans);
}
