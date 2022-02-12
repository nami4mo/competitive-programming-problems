#![allow(unused_imports)]
use im_rc::HashMap;
use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize}
    let mut ans = vec![0; 6];
    for i in 0..n {
        input! {s: char, v: String}
        let val = (s as u32 - 'A' as u32) as usize;
        if v == "AC" && ans[val] == 0 {
            ans[val] = i + 1;
        }
    }
    for &a in &ans {
        println!("{}", a);
    }
}
