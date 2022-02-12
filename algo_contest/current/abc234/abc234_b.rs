#![allow(unused_imports)]
use libm::sqrt;
use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        //
        n: usize,
        xyl: [(i64,i64); n]
    }
    let mut ans = 0;
    for i in 0..n {
        let (x, y) = xyl[i];
        for j in 0..n {
            let (x1, y1) = xyl[j];
            let d2 = (x - x1) * (x - x1) + (y - y1) * (y - y1);
            ans = ans.max(d2);
        }
    }
    println!("{}", sqrt(ans as f64));
}
