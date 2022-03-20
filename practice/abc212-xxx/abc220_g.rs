#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::gcd;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::HashMap;

// #[fastout]
fn main() {
    input! {
        n: usize,
        pl: [(i64, i64, i64); n],
    }

    let ans = pl
        .iter()
        .tuple_combinations()
        .map(|(pi, pj)| {
            let (a, b) = (pi.0 - pj.0, pi.1 - pj.1);
            let c = (pi.0 * pi.0 - pj.0 * pj.0 + pi.1 * pi.1 - pj.1 * pj.1) * (-1); // ax + by + c*2 = 0
            let mp = (pi.0 + pj.0, pi.1 + pj.1); // midpoint * 2

            let cost = pi.2 + pj.2;
            let siga = if a > 0 { 1 } else { -1 };
            let sigb = if b > 0 { 1 } else { -1 };
            if a == 0 {
                let g = gcd(b, c);
                ((sigb * a / g, sigb * b / g, sigb * c / g), (mp, cost))
            } else if b == 0 {
                let g = gcd(a, c);
                ((siga * a / g, siga * b / g, siga * c / g), (mp, cost))
            } else {
                let g = gcd(a, gcd(b, c));
                ((siga * a / g, siga * b / g, siga * c / g), (mp, cost))
            }
        })
        .into_group_map()
        .iter()
        .filter_map(|(_, costs)| {
            let costs = costs
                .clone()
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|(_, v)| v.into_iter().max().unwrap())
                .sorted_by_key(|x| Reverse(*x))
                .collect_vec();
            if costs.len() < 2 {
                None
            } else {
                Some(costs[0] + costs[1])
            }
        })
        .max()
        .unwrap_or(-1);
    println!("{}", ans);
}
