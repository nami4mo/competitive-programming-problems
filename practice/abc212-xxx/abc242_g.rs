#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

struct Mo {
    // [0] change vals type (if necessary) !!!!!!
    pair_cnt: usize,
    cl: usize,
    cr: usize,
    vals: Vec<usize>,
    // [1] add info for ans !!!!!!!
    cnts: Vec<usize>,
}

impl Mo {
    // [0] change vals type (if necessary) !!!!!!
    pub fn new(vals: &Vec<usize>) -> Self {
        Mo {
            vals: vals.clone(),
            cl: 0,
            cr: 0,
            // [1] add info for ans !!!!!!!
            cnts: vec![0; vals.len()],
            pair_cnt: 0,
        }
    }

    // [2] impl this !!!!!!!
    // update info with self.vals[i]
    fn add(&mut self, i: usize) {
        let v = self.vals[i];
        self.cnts[v] += 1;
        if self.cnts[v] % 2 == 0 {
            self.pair_cnt += 1;
        }
    }

    // [3] impl this !!!!!!!
    // update info without self.vals[i]
    fn delete(&mut self, i: usize) {
        let v = self.vals[i];
        self.cnts[v] -= 1;
        if self.cnts[v] % 2 == 1 {
            self.pair_cnt -= 1;
        }
    }

    fn process_query(&mut self, l: usize, r: usize) {
        if self.cr < r {
            for i in self.cr..r {
                self.add(i);
            }
        } else if r < self.cr {
            for i in (r..self.cr).rev() {
                self.delete(i);
            }
        }
        if self.cl < l {
            for i in self.cl..l {
                self.delete(i);
            }
        } else if l < self.cl {
            for i in (l..self.cl).rev() {
                self.add(i);
            }
        }
        self.cl = l;
        self.cr = r;
    }

    // [4] (if necessary) change ans vec type !!!!!!!!!!!!
    pub fn process_all(&mut self, queries: Vec<(usize, usize)>) -> Vec<i64> {
        let bucket_size = self.vals.len().sqrt() + 1;
        let mut buckets = vec![vec![]; bucket_size + 1];
        for (qi, &(l, r)) in queries.iter().enumerate() {
            buckets[l / bucket_size].push((l, r, qi));
        }
        for bucket in &mut buckets {
            bucket.sort_by_key(|v| v.1);
        }

        // [5] init ans vec !!!!!!!!!!!!
        let mut ans = vec![-1; queries.len()];

        for bucket in &buckets {
            for &(l, r, qi) in bucket {
                self.process_query(l, r);

                // [6] set ans !!!!!!!!!
                ans[qi] = self.pair_cnt as i64;
            }
        }
        // [7] return ans !!!!!!
        ans
        // vec![]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        cl: [Usize1 ;n],
        q: usize,
        lrs: [(Usize1, usize); q]
    }
    let mut mo = Mo::new(&cl);
    let ans = mo.process_all(lrs);
    for &a in &ans {
        println!("{}", a);
    }
}
