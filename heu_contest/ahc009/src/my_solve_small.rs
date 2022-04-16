use core::panic;
use im_rc::HashMap;
use itertools::Itertools;
use mytool::MyRng;
use ordered_float::OrderedFloat;
use proconio::input;
use proconio::marker::Chars;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::vec;

const SIZE: usize = 20; // 盤面のサイズ（縦 | 横）

// 向きを表す Enum
#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
enum Dir {
    L,
    R,
    U,
    D,
}

impl Dir {
    fn from_dyx(dy: i32, dx: i32) -> Self {
        match (dy, dx) {
            (0, -1) => Dir::L,
            (0, 1) => Dir::R,
            (-1, 0) => Dir::U,
            (1, 0) => Dir::D,
            _ => panic!("abs(dy) + abs(dx) != 1"),
        }
    }

    // Dir を (y方向, x方向) の移動の大きさに変換
    fn to_dyx(&self) -> (i32, i32) {
        match self {
            Self::L => (0, -1),
            Self::R => (0, 1),
            Self::U => (-1, 0),
            Self::D => (1, 0),
        }
    }

    // Dir を char に変換（答えの出力に使用）
    fn to_c(&self) -> char {
        match self {
            Self::L => 'L',
            Self::R => 'R',
            Self::U => 'U',
            Self::D => 'D',
        }
    }

    // 反対方向の Dir を返す ( L <-> R,  U <-> D )
    fn rev(&self) -> Self {
        match self {
            Self::L => Self::R,
            Self::R => Self::L,
            Self::U => Self::D,
            Self::D => Self::U,
        }
    }
}

fn make_good_dir(
    sy: usize,
    sx: usize,
    ty: usize,
    tx: usize,
    gl: &Vec<Vec<Vec<(usize, usize)>>>,
) -> (Option<Dir>, usize) {
    if (sy, sx) == (ty, tx) {
        return (None, 0);
    }

    let mut prevs = vec![vec![None; 20]; 20];
    let mut q = VecDeque::new();
    q.push_back((sy, sx));

    'outer: while let Some(v) = q.pop_front() {
        let neibs = &gl[v.0][v.1];
        for &neib in neibs {
            let (y, x) = neib;
            match prevs[y][x] {
                None => {
                    prevs[y][x] = Some(v);
                    q.push_back(neib);
                }
                _ => (),
            }
            if neib == (ty, tx) {
                break 'outer;
            }
        }
    }

    match prevs[ty][tx] {
        None => (None, 0),
        Some(_) => {
            let mut res = vec![(ty, tx)];
            let mut curr = (ty, tx);
            while curr != (sy, sx) {
                let (y, x) = curr;
                curr = prevs[y][x].unwrap();
                res.push(curr);
            }
            res.reverse();

            let (ny, nx) = (res[1].0, res[1].1);
            let dy = ny as i32 - sy as i32;
            let dx = nx as i32 - sx as i32;
            let d = Dir::from_dyx(dy, dx);
            (Some(d), res.len() - 1)
        }
    }
}

struct State {
    pub sy: usize, // スタート地点の y 座標（row）
    pub sx: usize, // スタート地点の x 座標（col）
    pub ty: usize, // ゴール地点の y 座標（row）
    pub tx: usize, // ゴール地点の x 座標（col）

    pub p: f64, // 高橋くんが文字を忘れる確率（入力の p）

    pub commands: Vec<Dir>, // 高橋くんが覚える文字列をここで管理すると楽
    pub score: f64,         // 現在の commands に対する score（評価値）を計算して管理しておくと楽

    // メソッドで参照する
    gl_x: Vec<Vec<bool>>, // 横方向の壁の有無. true -> 通行可能. （入力の h）
    gl_y: Vec<Vec<bool>>, // 縦方向の壁の有無. true -> 通行可能. （入力の v）
    pub good_dir: Vec<Vec<Option<Dir>>>,
    pub gl: Vec<Vec<Vec<(usize, usize)>>>, // gl[y][x] = Vec<(y,x) から移動可能な座標>
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
            hl: [ Chars; 20],
            vl: [ Chars; 19],
        }

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

        const DXYS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

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
                    let ny = (cy as i32 + dy) as usize;
                    let nx = (cx as i32 + dx) as usize;
                    let d = Dir::from_dyx(dy, dx);
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
                    gl[cy][cx].push((ny, nx));
                }
            }
        }

        let mut good_dir = vec![vec![None; 20]; 20];
        let mut dists = vec![vec![0; 20]; 20];

        for y in 0..20 {
            for x in 0..20 {
                let (d, dist) = make_good_dir(y, x, ty, tx, &gl);
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
            sy,
            sx,
            ty,
            tx,
            gl_x,
            gl_y,
            p,
            score: 0.0,
            gl,
            commands: vec![],
            good_dir,
            dists,
        }
    }
    pub fn move_dir(&self, d: Dir, y: usize, x: usize) -> (usize, usize) {
        let (dy, dx) = d.to_dyx();

        let cy = y;
        let cx = x;
        let pos = (y, x);
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
        let ny = (cy as i32 + dy) as usize;
        let nx = (cx as i32 + dx) as usize;
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
        (ny, nx)
    }
}

fn make_first_commands2(state: &State) -> Vec<Dir> {
    let mut pers = vec![vec![0.0; 20]; 20];
    pers[state.sy][state.sx] = 10000.0;
    let mut res = vec![];
    let dirs = vec![Dir::R, Dir::L, Dir::U, Dir::D];

    for turn in 0..200 {
        let good_d = dirs
            .iter()
            .map(|&d| {
                let mut next_pers = vec![vec![0.0; 20]; 20];
                for y in 0..20 {
                    for x in 0..20 {
                        next_pers[y][x] += pers[y][x] * state.p;
                        let (ny, nx) = state.move_dir(d, y, x);
                        next_pers[ny][nx] += pers[y][x] * (1.0 - state.p);
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
                next_pers[y][x] += pers[y][x] * state.p;
                let (ny, nx) = state.move_dir(good_d, y, x);
                next_pers[ny][nx] += pers[y][x];
            }
        }
        next_pers[state.ty][state.tx] = 0.0;
        pers = next_pers;
    }
    res
}

fn simulate2(state: &State) -> f64 {
    const MAX: f64 = 100000.0;
    let mut pers = vec![vec![0.0; 20]; 20];
    pers[state.sy][state.sx] = MAX as f64;
    let mut res = 0.0;

    for turn in 0..200 {
        let d = state.commands[turn];
        let mut next_pers = vec![vec![0.0; 20]; 20];

        for y in 0..20 {
            for x in 0..20 {
                next_pers[y][x] += pers[y][x] * state.p;
                let np = state.move_dir(d, y, x);
                next_pers[np.0][np.1] += pers[y][x] * (1.0 - state.p);
            }
        }

        res += next_pers[state.ty][state.tx] * (400 - turn) as f64;
        next_pers[state.ty][state.tx] = 0.0;

        pers = next_pers;
    }
    res / MAX as f64
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

    state.commands = make_first_commands2(&state);
    state.score = simulate2(&state);

    const TL: u64 = 1970;
    while tm.check_time(TL, true) {
        // opt2_all(&mut state, &mut myrng);
        let per = rng.get_percent();

        if per < 0.9 {
            let rand_i = rng.get_int(0, 199) as usize;
            let curr_d = state.commands[rand_i];

            let mut rand_d = rng.choose(&dirs);
            while rand_d == curr_d {
                rand_d = rng.choose(&dirs);
            }

            let curr_score = state.score;

            state.commands[rand_i] = rand_d;
            let new_score = simulate2(&state);

            if new_score < curr_score {
                state.commands[rand_i] = curr_d;
            } else {
                state.score = new_score;
            }
        } else {
            let rand_i = rng.get_int(10, 190) as usize;
            let rd = rng.get_int(1, 9) as usize;
            let rand_j = rand_i + rd;

            let curr_score = state.score;

            state.commands.swap(rand_i, rand_j);
            let new_score = simulate2(&state);
            if new_score < curr_score {
                // fail
                state.commands.swap(rand_i, rand_j);
            } else {
                state.score = new_score;
            }
        }
    }

    // ---- output ---
    for c in &state.commands {
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
        pub fn get_int(&mut self, left: i32, right: i32) -> i32 {
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
