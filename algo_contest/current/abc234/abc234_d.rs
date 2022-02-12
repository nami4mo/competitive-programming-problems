#![allow(unused_imports)]
use libm::sqrt;
use num_integer::Roots;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        //
        n: usize,
        k: usize,
        pl: [Usize1; n],
    }
    let mut used = vec![false; n];
    let mut curr = n;
    for i in 0..k {
        used[pl[i]] = true;
        curr = curr.min(pl[i]);
    }
    println!("{}", curr + 1);
    for i in k..n {
        let p = pl[i];
        if curr < p {
            used[p] = true;
            loop {
                curr += 1;
                if used[curr] {
                    break;
                }
            }
        }
        println!("{}", curr + 1);
    }
}
