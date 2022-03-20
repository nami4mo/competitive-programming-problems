#![allow(unused_imports)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// #[fastout]
fn main() {
    input! {
        //
        n: usize,
        m: usize,
        al: [u64; n]
    }
    let sbit = (0..30).find(|&i| 2usize.pow(i) >= n.max(m)).unwrap();
    let s = 2usize.pow(sbit);
    let mut dp = vec![0; s];
    for i in 0..n {
        dp[s - (n - i)] = al[i];
    }
    for bit in 0..sbit {
        for j in 0..s {
            if j & (1 << bit) > 0 {
                dp[j ^ (1 << bit)] ^= dp[j];
            }
        }
    }
    for i in 0..m {
        print!("{} ", dp[i]);
    }
    println!();
}
