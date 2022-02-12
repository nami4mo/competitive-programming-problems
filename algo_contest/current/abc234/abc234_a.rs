#![allow(unused_imports)]
use proconio::{input, marker::Usize1};

fn f(x: u64) -> u64 {
    x * x + 2 * x + 3
}

fn main() {
    input! {
        //
        t: u64
    }
    let ans = f(f(f(t) + t) + f(f(t)));
    println!("{}", ans);
}
