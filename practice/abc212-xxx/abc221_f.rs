#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet, VecDeque};

fn get_max_ind(vl: &Vec<usize>) -> usize {
    let (max_index, _max) =
        vl.iter()
            .enumerate()
            .fold((std::usize::MIN, std::usize::MIN), |(i_a, a), (i_b, &b)| {
                if b > a {
                    (i_b, b)
                } else {
                    (i_a, a)
                }
            });
    max_index
}

const MOD: usize = 998244353;

// #[fastout]
fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n-1],
    }
    let mut gl = vec![vec![]; n];
    for &(u, v) in &es {
        gl[u].push(v);
        gl[v].push(u);
    }
    fn dfs(node: usize, pare: usize, gl: &Vec<Vec<usize>>, dists: &mut Vec<usize>, d: usize) {
        dists[node] = d;
        for &neib in &gl[node] {
            if neib == pare {
                continue;
            }
            dfs(neib, node, gl, dists, d + 1);
        }
    }

    let mut d1 = vec![0; n];
    dfs(0, !0, &gl, &mut d1, 0);
    let p1 = get_max_ind(&d1);

    let mut d2 = vec![0; n];
    dfs(p1, !0, &gl, &mut d2, 0);
    let p2 = get_max_ind(&d2);

    let mut d1 = vec![0; n];
    dfs(p2, !0, &gl, &mut d1, 0);
    let p1 = get_max_ind(&d1);

    let diam = d1[p1];
    let rad = diam / 2;

    fn dfs2(node: usize, pare: usize, gl: &Vec<Vec<usize>>, dists: &mut Vec<usize>, d: usize) {
        dists.push(d);
        for &neib in &gl[node] {
            if neib == pare {
                continue;
            }
            dfs2(neib, node, gl, dists, d + 1);
        }
    }

    // return;
    if d1[p1] % 2 == 0 {
        let center = (0..n).find(|&i| d1[i] == rad && d2[i] == rad).unwrap();
        let mut cnts = vec![];
        for &top in &gl[center] {
            let mut dists = vec![];
            dfs2(top, center, &gl, &mut dists, 1);
            let cnt = dists.iter().filter(|&&v| v == rad).count();
            cnts.push(cnt);
        }
        let mut ans = 1;
        for &c in &cnts {
            ans *= c + 1;
            ans %= MOD;
        }
        ans += MOD;
        ans -= cnts.iter().sum::<usize>();
        ans -= 1;
        ans = (ans % MOD + MOD) % MOD;
        println!("{}", ans);
    } else {
        let center1 = (0..n).find(|&i| d1[i] == rad + 1 && d2[i] == rad).unwrap();
        let center2 = (0..n).find(|&i| d1[i] == rad && d2[i] == rad + 1).unwrap();

        fn solve(center: usize, not_center: usize, rad: usize, gl: &Vec<Vec<usize>>) -> usize {
            let mut dists = vec![];
            dfs2(center, not_center, &gl, &mut dists, 0);
            let cnt = dists.iter().filter(|&&v| v == rad).count();
            cnt
        }

        let a1 = solve(center1, center2, rad, &gl);
        let a2 = solve(center2, center1, rad, &gl);
        let ans = a1 * a2 % MOD;
        println!("{}", ans);
    }

    // println!("{} {}", p1, p2);
}
