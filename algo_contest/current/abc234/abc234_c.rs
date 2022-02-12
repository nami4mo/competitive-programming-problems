#![allow(unused_imports)]
use libm::sqrt;
use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        //
        mut k: u64,
    }
    let mut ans = vec![];
    while k > 0 {
        if k & 1 == 1 {
            ans.push(2);
        } else {
            ans.push(0);
        }
        k /= 2;
    }
    ans.reverse();
    for &a in &ans {
        print!("{}", a);
    }
    println!();
}
