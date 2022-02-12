#![allow(unused_imports, dead_code, unused_macros)]
use im_rc::{HashMap, HashSet};
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;

trait Bisect {
    type Item: Ord + Copy;
    fn bisect_left(&self, x: Self::Item) -> usize;
    fn bisect_right(&self, x: Self::Item) -> usize;

    /* --- count --- */
    fn less_eq_cnt(&self, x: Self::Item) -> usize; //     <= x
    fn less_cnt(&self, x: Self::Item) -> usize; //        < x
    fn greater_eq_cnt(&self, x: Self::Item) -> usize; //  >= x
    fn greater_cnt(&self, x: Self::Item) -> usize; //     > x

    /* --- nearest value --- */
    fn less_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
    fn less_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
    fn greater_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
    fn greater_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
}

impl<T: Ord + Copy> Bisect for Vec<T> {
    type Item = T;
    fn bisect_left(&self, x: Self::Item) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;
            if x <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn bisect_right(&self, x: Self::Item) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;
            if x < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn less_eq_cnt(&self, x: Self::Item) -> usize {
        self.bisect_right(x)
    }
    fn less_cnt(&self, x: Self::Item) -> usize {
        self.bisect_left(x)
    }
    fn greater_eq_cnt(&self, x: Self::Item) -> usize {
        self.len() - self.bisect_left(x)
    }
    fn greater_cnt(&self, x: Self::Item) -> usize {
        self.len() - self.bisect_right(x)
    }
    fn less_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_right(x);
        match ind {
            0 => None,
            _ => Some((ind - 1, self[ind - 1])),
        }
    }
    fn less_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_left(x);
        match ind {
            0 => None,
            _ => Some((ind - 1, self[ind - 1])),
        }
    }
    fn greater_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_left(x);
        if ind == self.len() {
            None
        } else {
            Some((ind, self[ind]))
        }
    }
    fn greater_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_right(x);
        if ind == self.len() {
            None
        } else {
            Some((ind, self[ind]))
        }
    }
}

// #[fastout]
fn main() {
    input!(n: usize, p: i64, mut al: [i64; n]);
    al.reverse();
    for i in 0..n {
        al[i] -= i as i64;
    }
    let mut dp = vec![1e10 as i64; n + 1];
    dp[0] = -1e10 as i64;
    let mut ans = 0;
    for &a in &al {
        if a > p - (n as i64 - 1) || a < 0 {
            continue;
        }
        let ind = dp.less_eq_nearest(a).unwrap().0;
        dp[ind + 1] = a;
        ans = ans.max(ind + 1);
    }
    println!("{}", n - ans);
}
