#![allow(unused_imports)]
use libm::sqrt;
use num_integer::Roots;
use proconio::{fastout, input, marker::Chars, marker::Usize1};

pub struct Comb {
    #[allow(dead_code)]
    inv: Vec<u64>,
    fac: Vec<u64>,
    finv: Vec<u64>,
    m: u64,
}

impl Comb {
    pub fn new(mut n: usize, m: u64) -> Self {
        n += 1;
        let mut fac = vec![0; n];
        let mut inv = vec![0; n];
        let mut finv = vec![0; n];
        for i in 0..2 {
            fac[i] = 1;
            inv[i] = 1;
            finv[i] = 1;
        }
        for i in 2..n {
            let iu64 = i as u64;
            fac[i] = (fac[i - 1] * iu64) % m;
            inv[i] = m - (inv[m as usize % i] * (m / iu64)) % m;
            finv[i] = (finv[i - 1] * inv[i]) % m;
        }
        Comb { fac, inv, finv, m }
    }
    pub fn com(&self, n: usize, r: usize) -> u64 {
        if n < r {
            return 0;
        }
        self.fac[n] * (self.finv[r] * self.finv[n - r] % self.m) % self.m
    }
    pub fn perm(&self, n: usize, r: usize) -> u64 {
        if n < r {
            return 0;
        }
        self.fac[n] * self.finv[n - r] % self.m
    }
    // TODO: lucas
    // def lucas(self, n, r): # nCr (mod self._mod(prime))
    //     if n < r: return 0
    //     res = 1
    //     while n > 0:
    //         nq, rq = n//self._mod, r//self._mod
    //         nr, rr = n-nq*self._mod, r-rq*self._mod
    //         res *= self.com(nr, rr)
    //         res %= self._mod
    //         n, r = nq, rq
    //     return res
}

fn main() {
    input! {
        //
        s: Chars,
    }
    let n = s.len();
    let mut cnts = vec![0; 26];
    for &si in &s {
        let d = si as u32 - 'a' as u32;
        cnts[d as usize] += 1;
    }
    let mo = 998244353;
    let comb = Comb::new(10000, mo);
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for &c in &cnts {
        let mut new_dp = vec![0; n + 1];
        for v in 0..c + 1 {
            for i in 0..n + 1 {
                if i + v > n {
                    break;
                }
                new_dp[i + v] += dp[i] * comb.com(i + v, v);
                new_dp[i + v] %= mo;
            }
        }
        dp = new_dp;
    }
    let ans: u64 = dp[1..n + 1].iter().sum();
    println!("{}", ans % mo);
}
