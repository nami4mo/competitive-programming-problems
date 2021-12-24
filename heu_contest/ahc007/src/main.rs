#![allow(unused_imports)]
use core::panic;
use itertools::izip;
use libm::{pow, sqrtf};
use num_integer::sqrt;
use proconio::input;
use proconio::marker::Usize1;
use proconio::source::line::LineSource;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::io::Write;
use std::io::{BufReader, Stdin};
use std::vec;

// const FIRST_EST_SKILL: i64 = 10;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
    id: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum EdgeState {
    Enabled,  // 現在、採用中
    Disabled, // 現在、不採用中
    Deleted,  // 使用しないことに決めた（もう使えない）
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Edge {
    u: usize,
    v: usize,
    id: usize,
    est_d: i64,
    est_d_nocoef: i64,
    real_d: i64,
    st: EdgeState,
}

#[derive(Debug, PartialEq)]
pub struct State {
    n: usize,
    m: usize,
    points: Vec<Point>,
    edges: Vec<Edge>,
    first_mst: Vec<bool>,         // 使う辺がtrue
    gl: Vec<Vec<(usize, usize)>>, // to, ei
}

pub fn calc_p2_dist(p0: &Point, p1: &Point, id: i64) -> (i64, i64) {
    let d2 = (p0.x - p1.x).pow(2) + (p0.y - p1.y).pow(2);
    let d = sqrt(d2);
    let coef = 16 as f64 + (id as f64) / 600.0;
    let coefed_d = d as f64 * coef / 10.0;
    (coefed_d as i64, 2 * d)
}

fn calc_first_mst(state: &mut State) {
    let mut edges2 = state.edges.clone();
    edges2.sort_by_key(|x| x.est_d);

    let mut uf = UnionFind::new(state.n);
    for &e in &edges2 {
        if uf.issame(e.u, e.v) {
            continue;
        }
        uf.unite(e.u, e.v);
        state.first_mst[e.id] = true;
        state.edges[e.id].st = EdgeState::Enabled;
    }
}

impl State {
    pub fn new(stdin: &mut LineSource<BufReader<Stdin>>) -> Self {
        let n = 400;
        let m = 1995;
        let mut points = vec![];
        let mut edges = vec![];
        let mut gl = vec![vec![]; n];
        input! {
            from stdin,
            xyl: [(i64,i64); n],
            uvl: [(usize, usize); m],
        }
        let mut id = 0;
        for &(x, y) in &xyl {
            let p = Point { x, y, id };
            id += 1;
            points.push(p);
        }
        let mut id: usize = 0;
        for &(u, v) in &uvl {
            gl[u].push((v, id));
            gl[v].push((u, id));
            let (est_d, est_d_nocoef) = calc_p2_dist(&points[u], &points[v], id as i64);

            let e = Edge {
                u,
                v,
                id,
                est_d,
                est_d_nocoef,
                real_d: est_d,
                st: EdgeState::Disabled,
            };
            id += 1;
            edges.push(e);
        }
        let mut state = State {
            n,
            m,
            points,
            edges,
            gl,
            first_mst: vec![false; m],
        };
        calc_first_mst(&mut state);
        state
    }
}

fn get_set(state: &mut State, ei: usize, start: usize, used_u: &mut Vec<bool>) {
    assert!(state.edges[ei].st == EdgeState::Enabled);
    let mut q = VecDeque::new();
    q.push_back(start);
    used_u[start] = true;
    while let Some(poped) = q.pop_front() {
        // eprintln!("aaa: {}", poped);
        for &(neib, c_ei) in &state.gl[poped] {
            if c_ei == ei {
                // eprintln!("same! ");
                continue;
            }
            if state.edges[c_ei].st != EdgeState::Enabled {
                continue;
            }
            if used_u[neib] {
                continue;
            }
            used_u[neib] = true;
            q.push_back(neib);
        }
    }
    // let mut cnt = 0;
    // for &mut v in used_u {
    //     if v {
    //         cnt += 1;
    //     }
    // }
    // eprintln!("c {}", cnt);
}

// return: 使う or not
fn try_to_change_edge(state: &mut State, ei: usize, real_d: i64) -> bool {
    let edge = state.edges[ei];
    // eprintln!("c {:?}", edge);
    if edge.st != EdgeState::Enabled {
        state.edges[ei].st = EdgeState::Deleted;
        return false;
    }

    let u = edge.u;
    // let v = edge.v;
    let mut used_u = vec![false; state.n];
    get_set(state, ei, u, &mut used_u);
    let mut best_edge = (real_d, ei);
    // eprintln!("aaa:{}", state.edges.len());
    // eprintln!("--- {}", real_d);
    // let mut cnt = 0;
    for &edge in &state.edges {
        if edge.st != EdgeState::Disabled || edge.id == ei {
            continue;
        }
        // eprintln!("here2");
        // eprintln!("{} {}", used_u[edge.u], used_u[edge.v]);
        if used_u[edge.u] == used_u[edge.v] {
            continue;
        }
        // eprintln!("here");
        // eprintln!("d: {} id: {}", edge.est_d, edge.id);
        if best_edge.0 > edge.est_d {
            best_edge = (edge.est_d, edge.id);
            // cnt += 1;
            break;
        }
    }

    // for &edge in &state.edges {
    //     if edge.st != EdgeState::Disabled || edge.id == ei {
    //         continue;
    //     }
    //     // eprintln!("here2");
    //     // eprintln!("{} {}", used_u[edge.u], used_u[edge.v]);
    //     if used_u[edge.u] == used_u[edge.v] {
    //         continue;
    //     }
    //     // eprintln!("here");
    //     // eprintln!("d: {} id: {}", edge.est_d, edge.id);
    //     if best_edge.0 > edge.est_d {
    //         best_edge = (edge.est_d, edge.id);
    //         cnt += 1;
    //         // break;
    //     }
    // }

    // そのまま使う
    // eprintln!("best!!!! {:?}", best_edge);
    if best_edge.1 == ei {
        true
    } else {
        // eprintln!("update!!!! {} -> {}", ei, best_edge.1);
        state.edges[ei].st = EdgeState::Deleted;
        state.edges[best_edge.1].st = EdgeState::Enabled;
        false
    }
}

fn retry_mst(state: &mut State, ei: usize) {
    let mut edges2 = vec![];
    let mut uf = UnionFind::new(state.n);

    for i in 0..ei {
        let e = state.edges[i];
        if e.st == EdgeState::Enabled {
            uf.unite(e.u, e.v);
        }
    }

    for i in ei..state.m {
        let edge = state.edges[i];
        // if (edge.real_d as f64) / (edge.est_d_nocoef as f64) < 0.51 {
        //     edge.real_d *= 2;
        //     edge.real_d /= 3;
        // }
        edges2.push(edge);
    }
    edges2.sort_by_key(|x| x.real_d);

    for &e in &edges2 {
        if uf.issame(e.u, e.v) {
            state.edges[e.id].st = EdgeState::Disabled;
            continue;
        }

        uf.unite(e.u, e.v);
        state.first_mst[e.id] = true;
        state.edges[e.id].st = EdgeState::Enabled;
    }
}

/* ------------------------------------------------------------------------
    solve!
------------------------------------------------------------------------ */
fn solve() {
    /* ------------ ready ------------ */
    // let mut tm = mytool::TimeManager::new();
    // let mut myrng = mytool::MyRng::new();
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));

    let mut state = State::new(&mut stdin);

    for ei in 0..state.m {
        let real_d = get_stdin(&mut stdin);

        // if state.edges[ei].st == EdgeState::Enabled {
        if false {
            let res = try_to_change_edge(&mut state, ei, real_d);
            if res {
                // eprintln!("aaa");
                state.edges[ei].st = EdgeState::Enabled;
                println!("1");
            } else {
                state.edges[ei].st = EdgeState::Deleted;
                println!("0");
            }
        } else {
            eprintln!("{} -> {}", state.edges[ei].est_d, real_d);
            // state.edges[ei].est_d = real_d;
            state.edges[ei].real_d = real_d;
            retry_mst(&mut state, ei);
            if state.edges[ei].st == EdgeState::Enabled {
                println!("1");
            } else {
                state.edges[ei].st = EdgeState::Deleted;
                println!("0");
            }
        }
    }

    let mut uf = UnionFind::new(state.n);
    for &e in &state.edges {
        if e.st == EdgeState::Enabled {
            uf.unite(e.u, e.v);
        }
    }
    eprintln!("c {}", uf.size(0));

    // for ei in 0..state.m {
    //     let real_d = get_stdin(&mut stdin);
    //     if state.edges[ei].st == EdgeState::Enabled {
    //         println!("1");
    //     } else {
    //         println!("0");
    //     }
    // }

    // let mut cnt = 0;
    // for &e in &state.edges {
    //     if e.st == EdgeState::Enabled {
    //         cnt += 1;
    //     }
    // }
    // eprintln!("c {}", cnt);
}

/* ------------------------------------------------------------------------
    need not change below
------------------------------------------------------------------------ */
fn main() {
    solve();
}

fn get_stdin(stdin: &mut LineSource<BufReader<Stdin>>) -> i64 {
    input! ( from stdin, inval: i64 );
    inval
}

pub mod mytool {
    use rand::prelude::*;
    use std::fmt;
    use std::time::Instant;
    pub struct TimeManager {
        count: u64,
        start: Instant,
        last_time: u64,
    }
    impl fmt::Display for TimeManager {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.count)
        }
    }
    impl TimeManager {
        const COUNT_PER_MEASURE: u64 = 1000;
        pub fn new() -> Self {
            TimeManager {
                count: 0,
                start: Instant::now(),
                last_time: 0,
            }
        }
        pub fn check_time(&mut self, time: u64, force: bool) -> bool {
            self.count += 1;
            if force || self.count > Self::COUNT_PER_MEASURE {
                self.count = 0;
                self.last_time = self.start.elapsed().as_millis() as u64;
            }
            self.last_time < time
        }
        pub fn get_time(&self) -> u64 {
            self.start.elapsed().as_millis() as u64
        }
    }
    pub struct MyRng {
        rng: SmallRng,
    }
    impl MyRng {
        pub fn new() -> Self {
            MyRng {
                rng: SmallRng::seed_from_u64(4445),
            }
        }
        pub fn get_int(&mut self, left: i64, right: i64) -> i64 {
            //! get [left, right] ( not [left, right)  )
            self.rng.gen_range(left, right + 1)
        }
        pub fn get_percent(&mut self) -> f64 {
            self.rng.gen::<f64>()
        }
        pub fn shuffle<T>(&mut self, v: &mut [T]) {
            v.shuffle(&mut self.rng);
        }
        pub fn choose<T: Copy>(&mut self, v: &[T]) -> T {
            *v.choose(&mut self.rng).unwrap()
        }
    }
}

// https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
