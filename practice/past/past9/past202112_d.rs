#![allow(unused_imports)]
use proconio::{input, marker::Usize1};
use std::cmp::Ordering;

fn main() {
    input! {n: usize, al: [i64; n], bl: [i64; n]}
    let mut abil = vec![];
    for i in 0..n {
        abil.push((al[i], bl[i], i + 1));
    }
    // abil.sort_by_key(|x| (-x.0 - x.1, -x.0, x.2));
    abil.sort_by(|x, y| {
        if x.0 + x.1 != y.0 + y.1 {
            (y.0 + y.1).cmp(&(x.0 + x.1))
        } else if x.0 != y.0 {
            (y.0).cmp(&x.0)
        } else {
            x.2.cmp(&y.2)
        }
    });
    for &a in &abil {
        print!("{} ", a.2);
    }
    println!();
}
