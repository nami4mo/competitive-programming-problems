#![allow(unused_imports, dead_code, unused_macros)]
use ordered_float::OrderedFloat;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        al: [[i64; w]; h],
    }
    let inf = std::i64::MAX / 2;
    let mut ans = inf;
    for yy in 0..h {
        for xx in 0..w {
            let v = al[yy][xx];
            let mut dp = vec![vec![vec![inf; w + h + 1]; w]; h];
            if al[0][0] > v {
                dp[0][0][1] = al[0][0];
            } else if al[0][0] < v {
                dp[0][0][0] = 0;
            } else {
                if yy == 0 && xx == 0 {
                    dp[0][0][1] = v;
                } else {
                    dp[0][0][0] = 0;
                }
            }
            for y in 0..h {
                for x in 0..w {
                    if y == 0 && x == 0 {
                        continue;
                    }
                    let a = al[y][x];
                    for j in 0..(w + h) {
                        let (val, dj) = if a > v {
                            (a, 1)
                        } else if a < v {
                            (0, 0)
                        } else {
                            if y >= yy && x >= xx {
                                (a, 1)
                            } else {
                                (0, 0)
                            }
                        };
                        if dj == 1 && j == 0 {
                            continue;
                        }
                        let mut mi = inf;
                        if x != 0 {
                            mi = dp[y][x - 1][j - dj].min(mi);
                        }
                        if y != 0 {
                            mi = dp[y - 1][x][j - dj].min(mi);
                        }
                        dp[y][x][j] = dp[y][x][j].min(mi + val);
                    }
                }
            }
            ans = ans.min(dp[h - 1][w - 1][k]);
        }
    }
    println!("{}", ans);
}
