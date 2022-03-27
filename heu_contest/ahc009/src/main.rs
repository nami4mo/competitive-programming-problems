use core::panic;
use im_rc::HashMap;
use itertools::Itertools;
use mytool::MyRng;
use num::abs;
use num_traits::Pow;
use ordered_float::OrderedFloat;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use proconio::source::line::LineSource;
use rand::prelude::SliceRandom;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::io::Write;
use std::io::{BufReader, Stdin};
use std::vec;

type Input = LineSource<BufReader<Stdin>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/* ------------------------- State -------------------------*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquareState {
    Free,
    Blocked,
    Targeted, // someone will block the square
}

#[derive(Debug, Clone)]
pub struct Square {
    pub pets: HashSet<usize>,   // id of pets
    pub humans: HashSet<usize>, // id of humans
    pub state: SquareState,
}

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum Dir {
    L,
    R,
    U,
    D,
}

impl Dir {
    fn dyx(&self) -> (i64, i64) {
        match self {
            Self::L => (0, -1),
            Self::R => (0, 1),
            Self::U => (-1, 0),
            Self::D => (1, 0),
        }
    }

    fn to_c(&self) -> char {
        match self {
            Self::L => 'L',
            Self::R => 'R',
            Self::U => 'U',
            Self::D => 'D',
        }
    }

    fn rev(&self) -> Self {
        match self {
            Self::L => Self::R,
            Self::R => Self::L,
            Self::U => Self::D,
            Self::D => Self::U,
        }
    }
}

fn dyx_to_dir(dy: i64, dx: i64) -> Dir {
    match (dy, dx) {
        (-1, 0) => Dir::U,
        (1, 0) => Dir::D,
        (0, -1) => Dir::L,
        (0, 1) => Dir::R,
        _ => panic!(),
    }
}

pub fn make_good_dir(start: Point, goal: Point, gl: &Vec<Vec<Vec<Point>>>) -> (Option<Dir>, usize) {
    if start == goal {
        return (None, 0);
    }

    let mut prevs = vec![vec![None; 20]; 20];
    let mut q = VecDeque::new();
    q.push_back(start);

    'outer: while let Some(v) = q.pop_front() {
        let neibs = &gl[v.y][v.x];
        for &neib in neibs {
            match prevs[neib.y][neib.x] {
                None => {
                    prevs[neib.y][neib.x] = Some(v);
                    q.push_back(neib);
                }
                _ => (),
            }
            if neib == goal {
                break 'outer;
            }
        }
    }

    match prevs[goal.y][goal.x] {
        None => (None, 0),
        Some(_) => {
            let mut res = vec![goal];
            let mut curr = goal;
            while curr != start {
                curr = prevs[curr.y][curr.x].unwrap();
                res.push(curr);
            }
            res.reverse();

            let (ny, nx) = (res[1].y, res[1].x);
            let dy = ny as i64 - start.y as i64;
            let dx = nx as i64 - start.x as i64;
            let d = dyx_to_dir(dy, dx);
            (Some(d), res.len() - 1)
        }
    }
}

pub struct State {
    pub start: Point,
    pub goal: Point,
    pub pos: Point,
    pub com: Vec<Dir>,
    pub p: f64,
    pub score: f64,
    pub gl_x: Vec<Vec<bool>>,
    pub gl_y: Vec<Vec<bool>>,
    pub good_dir: Vec<Vec<Option<Dir>>>,
    pub gl: Vec<Vec<Vec<Point>>>,
    pub dists: Vec<Vec<usize>>,
}

impl State {
    pub fn new() -> Self {
        input! {
            sy: usize,
            sx: usize,
            ty: usize,
            tx: usize,
            p: f64,
            // hl: [ [usize; 19]; 20],
            // vl: [ [usize; 20]; 19],
            hl: [ Chars; 20],
            vl: [ Chars; 19],
        }
        let start = Point { x: sx, y: sy };
        let goal = Point { x: tx, y: ty };

        let mut gl_x = vec![vec![true; 19]; 20];
        for y in 0..20 {
            for x in 0..19 {
                gl_x[y][x] = hl[y][x] == '0';
            }
        }

        let mut gl_y = vec![vec![true; 20]; 19];
        for y in 0..19 {
            for x in 0..20 {
                gl_y[y][x] = vl[y][x] == '0';
            }
        }

        let dirs = vec![Dir::R, Dir::L, Dir::U, Dir::D];

        const DXYS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        let mut gl = vec![vec![vec![]; 20]; 20];
        for cy in 0..20 {
            for cx in 0..20 {
                let mut rng = rand::thread_rng();
                let mut dd = DXYS.clone();
                dd.shuffle(&mut rng);
                for &(dy, dx) in &dd {
                    if cy == 0 && dy == -1 {
                        continue;
                    }
                    if cy == 19 && dy == 1 {
                        continue;
                    }
                    if cx == 0 && dx == -1 {
                        continue;
                    }
                    if cx == 19 && dx == 1 {
                        continue;
                    }
                    let ny = (cy as i64 + dy) as usize;
                    let nx = (cx as i64 + dx) as usize;
                    let d = dyx_to_dir(dy, dx);
                    if d == Dir::L && !gl_x[cy][nx] {
                        continue;
                    }
                    if d == Dir::R && !gl_x[cy][cx] {
                        continue;
                    }
                    if d == Dir::U && !gl_y[ny][cx] {
                        continue;
                    }
                    if d == Dir::D && !gl_y[cy][cx] {
                        continue;
                    }
                    gl[cy][cx].push(Point { y: ny, x: nx });
                }
            }
        }

        let mut good_dir = vec![vec![None; 20]; 20];
        let mut dists = vec![vec![0; 20]; 20];

        for y in 0..20 {
            for x in 0..20 {
                let (d, dist) = make_good_dir(Point { y, x }, goal, &gl);
                good_dir[y][x] = d;
                dists[y][x] = dist;
                // match d {
                //     None => (),
                //     Some(d) => good_dir[y][x] = d,
                // }
            }
        }

        // dbg!(&gl_x);
        Self {
            start,
            goal,
            pos: start,
            gl_x,
            gl_y,
            p,
            score: 200.0,
            gl,
            com: vec![],
            good_dir,
            dists,
        }
    }

    pub fn move_dir(&self, d: Dir, pos: Point) -> Point {
        let (dy, dx) = d.dyx();

        let cy = pos.y;
        let cx = pos.x;
        if cy == 0 && dy == -1 {
            return pos;
        }
        if cy == 19 && dy == 1 {
            return pos;
        }
        if cx == 0 && dx == -1 {
            return pos;
        }
        if cx == 19 && dx == 1 {
            return pos;
        }
        let ny = (cy as i64 + dy) as usize;
        let nx = (cx as i64 + dx) as usize;
        if d == Dir::L && !self.gl_x[cy][nx] {
            return pos;
        }
        if d == Dir::R && !self.gl_x[cy][cx] {
            return pos;
        }
        if d == Dir::U && !self.gl_y[ny][cx] {
            return pos;
        }
        if d == Dir::D && !self.gl_y[cy][cx] {
            return pos;
        }
        // self.pos = Point { y: ny, x: nx };
        Point { y: ny, x: nx }
    }
}

pub fn make_first_com(state: &State) -> Vec<Dir> {
    let mut pers = vec![vec![0.0; 20]; 20];
    pers[state.start.y][state.start.x] = 100.0;
    let mut res = vec![];
    for turn in 0..200 {
        let mut mp = HashMap::new();
        mp.insert(Dir::L, 0.0);
        mp.insert(Dir::R, 0.0);
        mp.insert(Dir::D, 0.0);
        mp.insert(Dir::U, 0.0);
        for y in 0..20 {
            for x in 0..20 {
                if state.good_dir[y][x].is_none() {
                    continue;
                }
                let d = state.good_dir[y][x].unwrap();
                mp[&d] += pers[y][x];
            }
        }
        let mut vl = mp.into_iter().map(|(k, v)| (k, v)).collect_vec();
        vl.sort_by_key(|&(_, v)| ordered_float::OrderedFloat(20000.0 - v));
        let mut good_d = vl[0].0;
        if turn != 0 {
            if good_d.rev() == *res.last().unwrap() {
                good_d = vl[1].0;
            }
        }

        res.push(good_d);

        let mut next_pers = vec![vec![0.0; 20]; 20];
        for y in 0..20 {
            for x in 0..20 {
                let p = Point { y, x };
                next_pers[y][x] += pers[y][x] * state.p;
                let np = state.move_dir(good_d, p);
                next_pers[np.y][np.x] += pers[y][x] * (1.0 - state.p);
            }
        }
        pers = next_pers;
    }
    res
}

pub fn make_first_com2(state: &State) -> Vec<Dir> {
    let mut pers = vec![vec![0.0; 20]; 20];
    pers[state.start.y][state.start.x] = 10000.0;
    let mut res = vec![];
    let dirs = vec![Dir::R, Dir::L, Dir::U, Dir::D];

    for turn in 0..200 {
        let good_d = dirs
            .iter()
            .map(|&d| {
                let mut next_pers = vec![vec![0.0; 20]; 20];
                for y in 0..20 {
                    for x in 0..20 {
                        let p = Point { y, x };
                        next_pers[y][x] += pers[y][x] * state.p;
                        let np = state.move_dir(d, p);
                        next_pers[np.y][np.x] += pers[y][x] * (1.0 - state.p);
                    }
                }
                let mut dsum = 0.0;
                for y in 0..20 {
                    for x in 0..20 {
                        dsum += next_pers[y][x] * state.dists[y][x] as f64;
                    }
                }
                (d, OrderedFloat(dsum))
            })
            .min_by_key(|&(_d, v)| v)
            .unwrap()
            .0;

        res.push(good_d);
        let mut next_pers = vec![vec![0.0; 20]; 20];
        for y in 0..20 {
            for x in 0..20 {
                let p = Point { y, x };
                next_pers[y][x] += pers[y][x] * state.p;
                let np = state.move_dir(good_d, p);
                next_pers[np.y][np.x] += pers[y][x];
            }
        }
        next_pers[state.goal.y][state.goal.x] = 0.0;
        pers = next_pers;
    }
    res
}

fn simulate2(state: &State, rng: &mut MyRng) -> f64 {
    const MAX: f64 = 100000.0;
    let mut pers = vec![vec![0.0; 20]; 20];
    pers[state.start.y][state.start.x] = MAX as f64;
    let mut res = 0.0;

    for turn in 0..200 {
        let d = state.com[turn];
        let mut next_pers = vec![vec![0.0; 20]; 20];

        for y in 0..20 {
            for x in 0..20 {
                let p = Point { y, x };
                next_pers[y][x] += pers[y][x] * state.p;
                let np = state.move_dir(d, p);
                next_pers[np.y][np.x] += pers[y][x] * (1.0 - state.p);
            }
        }

        res += next_pers[state.goal.y][state.goal.x] * (400 - turn) as f64;
        next_pers[state.goal.y][state.goal.x] = 0.0;

        pers = next_pers;
    }
    res / MAX as f64
}

pub fn make_first_com3(state: &State, rng: &mut MyRng) -> Vec<Dir> {
    let mut pers = vec![vec![0.0; 20]; 20];
    pers[state.start.y][state.start.x] = 1000000.0;
    let mut res = vec![];
    let dirs = vec![Dir::R, Dir::L, Dir::U, Dir::D];

    for turn in 0..200 {
        let dir_scores = dirs
            .iter()
            .map(|&d| {
                let mut next_pers = vec![vec![0.0; 20]; 20];
                for y in 0..20 {
                    for x in 0..20 {
                        let p = Point { y, x };
                        next_pers[y][x] += pers[y][x] * state.p;
                        let np = state.move_dir(d, p);
                        next_pers[np.y][np.x] += pers[y][x] * (1.0 - state.p);
                    }
                }
                let mut dsum = 0.0;
                for y in 0..20 {
                    for x in 0..20 {
                        dsum += next_pers[y][x] * state.dists[y][x] as f64;
                    }
                }
                (d, OrderedFloat(dsum))
            })
            .sorted_by_key(|&(_d, v)| v)
            .collect_vec();
        // .min_by_key(|&(_d, v)| v)
        // .unwrap()

        let good_d = if rng.get_percent() < 0.95 {
            dir_scores[0].0
        } else {
            dir_scores[1].0
        };

        res.push(good_d);
        let mut next_pers = vec![vec![0.0; 20]; 20];
        for y in 0..20 {
            for x in 0..20 {
                let p = Point { y, x };
                next_pers[y][x] += pers[y][x] * state.p;
                let np = state.move_dir(good_d, p);
                next_pers[np.y][np.x] += pers[y][x];
            }
        }
        next_pers[state.goal.y][state.goal.x] = 0.0;
        pers = next_pers;
    }
    res
}

/* ------------------------------------------------------------------------
    solve!
------------------------------------------------------------------------ */
fn solve() {
    /* ------------ ready ------------ */
    let mut tm = mytool::TimeManager::new();
    let mut rng = mytool::MyRng::new();
    let mut state = State::new();

    let dirs = vec![Dir::R, Dir::L, Dir::U, Dir::D];

    // let com = init_com(&mut rng);
    // let com = make_first_com(&state);
    let c2 = make_first_com2(&state);
    state.com = c2.clone();

    let score = simulate2(&state, &mut rng);
    state.score = score;

    let c1 = make_first_com(&state);
    state.com = c1;
    let score2 = simulate2(&state, &mut rng);
    if score2 < score {
        state.com = c2;
    } else {
        state.score = score2;
    }

    // for _ in 0..10 {
    //     let prev_c = state.com.clone();
    //     let c3 = make_first_com3(&state, &mut rng);
    //     state.com = c3;
    //     let score = simulate2(&state, &mut rng);
    //     if score < state.score {
    //         state.com = prev_c;
    //     }
    // }
    // println!("{}", state.score);

    // println!("{:?}", state.good_dir);

    // const TL: u64 = 1920;

    const TL: u64 = 3970;
    while tm.check_time(TL, true) {
        // opt2_all(&mut state, &mut myrng);
        let per = rng.get_percent();

        if per < 0.9 {
            let rand_i = rng.get_int(0, 199) as usize;
            let curr_d = state.com[rand_i];

            let mut rand_d = rng.choose(&dirs);
            while rand_d == curr_d {
                rand_d = rng.choose(&dirs);
            }

            let curr_score = state.score;

            state.com[rand_i] = rand_d;
            let new_score = simulate2(&state, &mut rng);

            if new_score < curr_score {
                state.com[rand_i] = curr_d;
            } else {
                state.score = new_score;
            }
        } else {
            // let rand_i = rng.get_int(0, 199) as usize;
            // let rand_j = rng.get_int(0, 199) as usize;

            let rand_i = rng.get_int(10, 190) as usize;
            let rd = rng.get_int(1, 9) as usize;
            let rand_j = rand_i + rd;

            let curr_score = state.score;

            state.com.swap(rand_i, rand_j);
            let new_score = simulate2(&state, &mut rng);

            // let kaizen = new_score - curr_score;
            // let yaki_p = yaki_prob(tm.get_time() as usize, TL as usize, 5.0, 0.2, kaizen as i64);

            // println!("{}", curr_score - new_score);
            if new_score < curr_score {
                // fail
                state.com.swap(rand_i, rand_j);
            } else {
                state.score = new_score;
            }
        }
    }

    // ---- output ---
    for c in &state.com {
        print!("{}", c.to_c());
    }
    println!();
    // ---- output ---

    eprintln!("Score = {}", state.score);
}

/* ------------------------------------------------------------------------
    need not change below
------------------------------------------------------------------------ */
fn main() {
    solve();
}

/* ----------------- IO -------------------- */
// pub fn get_pets_moves(stdin: &mut LineSource<BufReader<Stdin>>, n: usize) -> Vec<Vec<char>> {
//     input! {from stdin, moves: [Chars; n]}
//     moves
// }

pub fn output(cs: &Vec<char>) {
    for &c in cs {
        print!("{}", c);
    }
    println!();
    std::io::stdout().flush().unwrap();
}

fn yaki_prob(time_passed: usize, time_limit: usize, ther0: f64, ther1: f64, kaizen: i64) -> f64 {
    // --- how to use ---
    // if yaki_prob(
    //     i,
    //     UPDATE_SKILL_LOOP_CNT,
    //     (state.members[member_id].task_results.len() as f64) * 5.0,
    //     0.1,
    //     kaizen,
    // ) > myrng.get_percent()
    let time01 = (time_passed as f64) / (time_limit as f64);
    let mut tt = ther0.pow(1.0 - time01) * ther1.pow(time01);
    tt *= kaizen as f64;
    tt.exp()
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
