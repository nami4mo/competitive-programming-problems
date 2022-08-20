#![allow(unused_imports)]
use core::panic;
use itertools::Itertools;
use mytool::{MyRng, TimeManager};
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{borrow, vec};

const DYXS: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

#[cfg(not(feature = "mylocal"))]
const TL: u64 = 2850;
// const TL: u64 = 2700;

#[cfg(feature = "mylocal")]
const TL: u64 = 1900;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// 左/上/右/下 の順に 0/1/2/3
struct Dir(usize);
impl std::ops::Deref for Dir {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.0
    }
}
impl Dir {
    fn rev(self) -> Self {
        Dir((*self + 2) % 4)
    }
    fn dummy() -> Self {
        Dir(999)
    }
    fn iter_all_dirs() -> impl Iterator<Item = Self> {
        (0..4).map(|i| Dir(i))
    }
    fn to_dyx(self) -> (usize, usize) {
        match self {
            Dir(0) => (0, !0),
            Dir(1) => (!0, 0),
            Dir(2) => (0, 1),
            Dir(3) => (1, 0),
            Dir(_) => panic!(),
        }
    }
    fn to_c(self) -> char {
        match self {
            Dir(0) => 'L',
            Dir(1) => 'U',
            Dir(2) => 'R',
            Dir(3) => 'D',
            Dir(_) => panic!(),
        }
    }
    fn from_dyx(dyx: (usize, usize)) -> Self {
        if dyx == (0, !0) {
            Dir(0)
        } else if dyx == (!0, 0) {
            Dir(1)
        } else if dyx == (0, 1) {
            Dir(2)
        } else if dyx == (1, 0) {
            Dir(3)
        } else {
            panic!()
        }
    }
}

fn rev_dyx(dyx: (usize, usize)) -> (usize, usize) {
    if dyx == (0, !0) {
        (0, 1)
    } else if dyx == (!0, 0) {
        (1, 0)
    } else if dyx == (0, 1) {
        (0, !0)
    } else if dyx == (1, 0) {
        (!0, 0)
    } else {
        panic!()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// 左/上/右/下 の順に 0/1/2/3
struct Pattern(usize);
impl std::ops::Deref for Pattern {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.0
    }
}
impl Pattern {
    fn iter_all_patterns() -> impl Iterator<Item = Self> {
        (0..16).map(|i| Pattern(i))
    }
    fn is_matched(lhs: Self, rhs: Self) -> bool {
        Dir::iter_all_dirs().any(|dir| {
            let r_dir = dir.rev();
            (1 << *dir) & *lhs > 0 && (1 << *r_dir) & *rhs > 0
        })
    }
    fn from_single_dir(dir: Dir) -> Self {
        Pattern(1 << *dir)
    }
}

// そのパターンに存在する方向を返す
fn get_dirs_from_pattern(pattern: Pattern) -> Vec<Dir> {
    Dir::iter_all_dirs()
        .filter(|&dir| (1 << *dir) & *pattern > 0)
        .collect::<Vec<_>>()
}
// その方向が存在するパターンを返す
fn get_patterns_from_dir(dir: Dir) -> Vec<Pattern> {
    Pattern::iter_all_patterns()
        .filter(|&p| (1 << *dir) & *p > 0)
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Piece {
    // edges: [bool; 4], // 左/上/右/下 に 辺があるか
    pattern: Pattern, // 1 ~ 15
}

impl Piece {
    fn c_to_pattern(c: char) -> Pattern {
        Pattern(usize::from_str_radix(&c.to_string(), 16).unwrap())
    }
    fn from_char(c: char) -> Self {
        let pattern = Self::c_to_pattern(c);
        Self { pattern }
        // let edges: [bool; 4] = (0..4)
        //     .map(|i| (1 << i) & *pattern > 0)
        //     .collect::<Vec<_>>()
        //     .try_into()
        //     .unwrap();
        // Self { edges, pattern }
    }
    fn from_pattern(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cell {
    Piece(Piece),
    Empty,
}
impl Cell {
    fn piece_from_pattern(p: Pattern) -> Self {
        Self::Piece(Piece::from_pattern(p))
    }
    fn pattern(&self) -> Pattern {
        match self {
            Self::Piece(p) => p.pattern,
            _ => panic!("this cell is empty."),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    n: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(n: usize) -> Self {
        Self {
            n,
            cells: vec![vec![Cell::Empty; n]; n],
        }
    }
    fn is_cell_empty(&self, y: usize, x: usize) -> bool {
        match self.cells[y][x] {
            Cell::Empty => true,
            _ => false,
        }
    }
    fn set_cell(&mut self, y: usize, x: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }
    fn in_area(&self, y: usize, x: usize) -> bool {
        if y == self.n - 1 && x == self.n - 1 {
            return false;
        }
        (0..self.n).contains(&y) && (0..self.n).contains(&x)
    }
    fn swap(&mut self, p1: (usize, usize), p2: (usize, usize)) {
        let (y1, x1) = p1;
        let (y2, x2) = p2;
        let tmp = self.cells[y2][x2].clone();
        self.cells[y2][x2] = self.cells[y1][x1];
        self.cells[y1][x1] = tmp;
    }
    fn get_pattern_num(&self, y: usize, x: usize) -> usize {
        match self.cells[y][x] {
            Cell::Empty => 0,
            Cell::Piece(p) => *p.pattern,
        }
    }

    fn debug_print(&self) {
        for y in 0..self.n {
            for x in 0..self.n {
                let c = match self.cells[y][x] {
                    Cell::Empty => '.',
                    _ => 'o',
                };
                print!("{}", c);
            }
            println!();
        }
    }
    fn print_pattern(&self) {
        for y in 0..self.n {
            for x in 0..self.n {
                let c = match self.cells[y][x] {
                    Cell::Empty => "0".to_string(),
                    Cell::Piece(p) => {
                        let c = format!("{:01x}", *p.pattern);
                        c
                    }
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

struct State {
    n: usize,
    t: usize,                   // 2 * n**3
    orig_cells: Vec<Vec<Cell>>, // 入力そのまま
    board: Board,               //
    orig_board: Board,          // 入力そのまま
    piece_cnts: [usize; 16],    // 各パターンの
    orig_empty_pos: (usize, usize),
}

impl State {
    fn new() -> Self {
        input! {n: usize, t: usize, cells: [Chars; n]}
        let orig_cells = cells
            .into_iter()
            .map(|cs| {
                cs.into_iter()
                    .map(|c| match c {
                        '0' => Cell::Empty,
                        _ => Cell::Piece(Piece::from_char(c)),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut piece_cnts = [0; 16];
        for cell in orig_cells.iter().flatten() {
            if let Cell::Piece(cell) = cell {
                piece_cnts[*cell.pattern] += 1;
            }
        }
        let mut orig_board = Board::new(n);
        let mut orig_empty_pos = (0, 0);
        for y in 0..n {
            for x in 0..n {
                orig_board.set_cell(y, x, orig_cells[y][x]);
                if orig_cells[y][x] == Cell::Empty {
                    orig_empty_pos = (y, x);
                }
            }
        }
        let board = orig_board.clone();
        Self {
            n,
            t,
            orig_cells,
            board,
            orig_board,
            piece_cnts,
            orig_empty_pos,
        }
    }
}

fn solve_by_dfs(state: &State) {
    let center = state.n;
    let mut piece_cnts = state.piece_cnts.clone();
    let mut board = state.board.clone();
    let first_pattern = Pattern::iter_all_patterns()
        .find(|&i| piece_cnts[*i] > 0)
        .unwrap();

    struct DfsState {
        n: usize,
        done: bool,
        ans: Board,
    }
    let mut dfs_state = DfsState {
        n: state.n,
        done: false,
        ans: Board::new(1),
    };

    fn dfs(
        y: usize,
        x: usize,
        board: &mut Board,
        piece_cnts: &mut [usize],
        size: usize,
        dfs_state: &mut DfsState,
        from_dir: Dir,
    ) {
        // println!("{}", size);
        board.debug_print();

        if size > 30 {
            // println!("{}", size);
        }
        if dfs_state.done {
            return;
        }
        // if size == dfs_state.n * dfs_state.n - 1 {
        if size == 20 {
            dfs_state.done = true;
            dfs_state.ans = board.clone();
            return;
        }
        let piece = match board.cells[y][x] {
            Cell::Piece(p) => p,
            Cell::Empty => panic!("must not Empty"),
        };
        let dirs = get_dirs_from_pattern(piece.pattern)
            .into_iter()
            .filter(|&d| d != from_dir)
            .collect::<Vec<_>>();

        // 一つでもNGな方向があればそれ以上は不可
        let check_all_dirs_res = dirs.iter().all(|&d| {
            let (dy, dx) = d.to_dyx();
            let (yy, xx) = (y + dy, x + dx);
            if !board.in_area(yy, xx) {
                return false;
            }
            if !board.is_cell_empty(yy, xx) {
                return false;
            }
            let rev_dir = d.rev();
            let patterns = get_patterns_from_dir(rev_dir);
            patterns.iter().any(|&cp| piece_cnts[*cp] > 0)
        });

        if !check_all_dirs_res {
            return;
        }

        let mut rng = rand::prelude::SmallRng::seed_from_u64(4445);
        let mut dirs = dirs.clone();
        dirs.shuffle(&mut rng);
        // println!("{:?}", dirs);

        for d in dirs {
            let (dy, dx) = d.to_dyx();
            let (yy, xx) = (y + dy, x + dx);
            if !board.in_area(yy, xx) {
                // panic!();
                continue;
            }
            if !board.is_cell_empty(yy, xx) {
                // panic!();
                continue;
            }
            let rev_dir = d.rev();
            let patterns = get_patterns_from_dir(rev_dir);

            let mut patterns = patterns.clone();
            patterns.shuffle(&mut rng);
            // println!("{:?}", patterns);

            for p in patterns {
                if piece_cnts[*p] == 0 {
                    continue;
                }
                let looped_or_out_area = get_dirs_from_pattern(p)
                    .iter()
                    .filter(|&&c_dir| c_dir != rev_dir)
                    .any(|c_dir| {
                        let (dyy, dxx) = c_dir.to_dyx();
                        let (yyy, xxx) = (yy + dyy, xx + dxx);
                        if !board.in_area(yyy, xxx) {
                            return true;
                        }
                        // println!("  {} {} {} {}", yy, dy, xx, dx);
                        match board.cells[yyy][xxx] {
                            Cell::Empty => false,
                            Cell::Piece(neib_piece) => Pattern::is_matched(neib_piece.pattern, p),
                        }
                    });
                if looped_or_out_area {
                    continue;
                }
                piece_cnts[*p] -= 1;
                board.set_cell(yy, xx, Cell::piece_from_pattern(p));
                dfs(yy, xx, board, piece_cnts, size + 1, dfs_state, rev_dir);
                board.set_cell(yy, xx, Cell::Empty);
                piece_cnts[*p] += 1;
            }
        }
    }

    piece_cnts[12] -= 1;
    board.set_cell(0, 0, Cell::piece_from_pattern(Pattern(12)));
    dfs(
        0,
        0,
        &mut board,
        &mut piece_cnts,
        1,
        &mut dfs_state,
        Dir::dummy(),
    );
}

fn check_no_loop(board: &Board, sy: usize, sx: usize) -> bool {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((sy, sx, !0, !0));
    while let Some((cy, cx, py, px)) = q.pop_front() {
        let cell = board.cells[cy][cx];
        let piece = match cell {
            Cell::Piece(p) => p,
            _ => continue,
        };
        let dirs = get_dirs_from_pattern(piece.pattern);
        for d in dirs {
            let (dy, dx) = d.to_dyx();
            let (yy, xx) = (cy + dy, cx + dx);
            if (yy, xx) == (py, px) {
                continue;
            }
            if visited.contains(&(yy, xx)) {
                return false; // loop
            }
            visited.insert((yy, xx));
            q.push_back((yy, xx, cy, cx));
        }
    }
    true
}

fn get_group_size(board: &Board, sy: usize, sx: usize) -> usize {
    // let mut visited = HashSet::new();
    let mut visited = vec![vec![false; board.n]; board.n];
    let mut q = VecDeque::new();
    q.push_back((sy, sx));
    // visited.insert((sy, sx));
    visited[sy][sx] = true;
    while let Some((cy, cx)) = q.pop_front() {
        let cell = board.cells[cy][cx];
        let piece = match cell {
            Cell::Piece(p) => p,
            _ => continue,
        };
        let dirs = get_dirs_from_pattern(piece.pattern);
        for d in dirs {
            let (dy, dx) = d.to_dyx();
            let (yy, xx) = (cy + dy, cx + dx);
            if !board.in_area(yy, xx) {
                continue;
            }
            if visited[yy][xx] {
                // if visited.contains(&(yy, xx)) {
                continue;
            }
            match board.cells[yy][xx] {
                Cell::Empty => (),
                Cell::Piece(piece) => {
                    if Pattern::is_matched(piece.pattern, Pattern::from_single_dir(d)) {
                        // visited.insert((yy, xx));
                        visited[yy][xx] = true;
                        q.push_back((yy, xx));
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for y in 0..board.n {
        for x in 0..board.n {
            if visited[y][x] {
                ans += 1;
            }
        }
    }
    ans
    // visited.len()
}

fn get_group_size_with_empty(board: &Board, sy: usize, sx: usize, empty: (usize, usize)) -> usize {
    // let mut visited = HashSet::new();
    let mut visited = vec![vec![false; board.n]; board.n];
    let mut q = VecDeque::new();
    q.push_back((sy, sx));
    // visited.insert((sy, sx));
    visited[sy][sx] = true;
    while let Some((cy, cx)) = q.pop_front() {
        let cell = board.cells[cy][cx];
        let piece = match cell {
            Cell::Piece(p) => p,
            _ => continue,
        };
        let dirs = get_dirs_from_pattern(piece.pattern);
        for d in dirs {
            let (dy, dx) = d.to_dyx();
            let (yy, xx) = (cy + dy, cx + dx);
            // if !board.in_area(yy, xx) {
            if yy >= board.n || xx >= board.n || (yy, xx) == empty {
                continue;
            }
            if visited[yy][xx] {
                // if visited.contains(&(yy, xx)) {
                continue;
            }
            match board.cells[yy][xx] {
                Cell::Empty => (),
                Cell::Piece(piece) => {
                    if Pattern::is_matched(piece.pattern, Pattern::from_single_dir(d)) {
                        // visited.insert((yy, xx));
                        visited[yy][xx] = true;
                        q.push_back((yy, xx));
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for y in 0..board.n {
        for x in 0..board.n {
            if visited[y][x] {
                ans += 1;
            }
        }
    }
    ans
    // visited.len()
}

fn calc_board_score(board: &Board) -> usize {
    let n = board.n;
    let mut ng_edge_cnt = 0;
    let mut ok_edge_cnt = 0;
    for y in 0..n {
        for x in 0..n {
            if y == n - 1 && x == n - 1 {
                break;
            }
            // println!("-- {} {} --", y, x);
            let p = board.get_pattern_num(y, x);
            let dirs = get_dirs_from_pattern(Pattern(p));
            // println!("{:?}", dirs);
            // println!("p: {:?}", p);
            for dir in dirs {
                // println!("  dir: {:?}", dir);
                let (dy, dx) = dir.to_dyx();
                let (yy, xx) = (y + dy, x + dx);
                // println!("    {} {}", yy, xx);
                if !board.in_area(yy, xx) {
                    ng_edge_cnt += 1;
                    continue;
                }
                let p2 = board.get_pattern_num(yy, xx);

                // println!("    p2: {:?}", p2);

                if Pattern::is_matched(Pattern::from_single_dir(dir), Pattern(p2)) {
                    ok_edge_cnt += 1;
                } else {
                    ng_edge_cnt += 1;
                }
            }
        }
    }
    // dbg!(ok_edge_cnt, ng_edge_cnt);
    ok_edge_cnt
}

fn solve_by_dfs2(state: &State) {
    let mut piece_cnts = state.piece_cnts.clone();
    let mut board = state.board.clone();
    let n = state.n;

    let routes_right = (0..=n - 2).rev().map(|y| (y, n - 1)).collect::<Vec<_>>();
    let routes_top = (0..=n - 1).rev().map(|x| (0, x)).collect::<Vec<_>>();
    let routes_left = (0..=n - 1).map(|y| (y, 0)).collect::<Vec<_>>();
    let routes_bottom = (0..=n - 2).map(|x| (n - 1, x)).collect::<Vec<_>>();
    let routes = [routes_right, routes_top, routes_left, routes_bottom].concat();

    // dbg!(piece_cnts);

    let routes = (0..=n - 1)
        .flat_map(|y| {
            if y == n - 1 {
                (0..=n - 2).map(|x| (y, x)).collect::<Vec<_>>()
            } else {
                (0..=n - 1).map(|x| (y, x)).collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();

    struct DfsState {
        n: usize,
        done: bool,
        ans: Board,
        cnt: usize,
    }
    let mut dfs_state = DfsState {
        n: state.n,
        done: false,
        ans: Board::new(1),
        cnt: 0,
    };
    fn dfs(
        ri: usize,
        routes: &[(usize, usize)],
        piece_cnts: &mut [usize],
        board: &mut Board,
        dfs_state: &mut DfsState,
    ) {
        // dbg!(ri);
        if dfs_state.done {
            // return;
        }
        if ri == routes.len() {
            if get_group_size(board, 0, 0) == dfs_state.n * dfs_state.n - 1 {
                dfs_state.cnt += 1;
                dfs_state.done = true;
                // println!();
                // board.print_pattern();
            }
            return;
        }
        let (y, x) = routes[ri];
        let mut need = vec![];
        let mut ng = vec![];
        for dir in Dir::iter_all_dirs() {
            let (dy, dx) = dir.to_dyx();
            let (yy, xx) = (y + dy, x + dx);
            if !board.in_area(yy, xx) {
                ng.push(dir);
                continue;
            }
            match board.cells[yy][xx] {
                Cell::Empty => (),
                Cell::Piece(piece) => {
                    if Pattern::is_matched(piece.pattern, Pattern::from_single_dir(dir)) {
                        need.push(dir);
                    } else {
                        ng.push(dir);
                    }
                }
            }
        }

        // ここ高速化できる
        let need_v = need.iter().map(|&d| 1 << *d).sum::<usize>();
        let ng_v = ng.iter().map(|&d| 1 << *d).sum::<usize>();

        // これめっちゃ効いてそう
        let mut ps = Pattern::iter_all_patterns().collect_vec();
        if (y + x) % 2 == 1 {
            ps.reverse();
        }
        // ps.sort_by_key(|&p| piece_cnts[*p]);
        let mut rng = rand::prelude::SmallRng::seed_from_u64(4445);
        ps.shuffle(&mut rng);

        // dbg!(y, x, need_v, ng_v);
        // for pattern in Pattern::iter_all_patterns() {
        for pattern in ps {
            if need_v & *pattern != need_v || ng_v & *pattern > 0 {
                continue;
            }
            if piece_cnts[*pattern] == 0 {
                continue;
            }
            // dbg!(pattern);
            piece_cnts[*pattern] -= 1;
            board.set_cell(y, x, Cell::piece_from_pattern(pattern));
            if check_no_loop(board, y, x) {
                dfs(ri + 1, routes, piece_cnts, board, dfs_state);
            }
            board.set_cell(y, x, Cell::Empty);
            piece_cnts[*pattern] += 1;
        }
    }

    // 455949
    // 88c7db
    // let ps = vec![4, 5, 5, 9, 4, 9, 8, 8, 12, 7, 13, 11];
    // let ps = vec![4, 9, 4, 9, 8, 8, 8, 8, 14, 5, 7, 15, 15, 11];
    // println!("{:?}", piece_cnts);
    // let ps = vec![4, 5, 5, 5, 5, 9];
    // let ps = vec![4, 5, 5, 9, 4, 9];
    // let ps = vec![4, 13, 1, 12];
    // let ps = vec![];
    let ps = vec![8, 12, 9, 4, 9, 8];
    // let ps = vec![4, 5, 5, 5, 5, 9, 4, 5, 5, 5, 5, 11];
    for (i, &p) in ps.iter().enumerate() {
        if piece_cnts[p] == 0 {
            panic!("not enough");
        }
        piece_cnts[p] -= 1;
        let y = i / n;
        let x = i % n;
        board.set_cell(y, x, Cell::piece_from_pattern(Pattern(p)));
    }
    dfs(
        ps.len(),
        &routes,
        &mut piece_cnts,
        &mut board,
        &mut dfs_state,
    );
    // dfs(0, &routes, &mut piece_cnts, &mut board, &mut dfs_state);
    dbg!(dfs_state.cnt);
}

fn solve_by_beam(state: &State, rng: &mut MyRng) {
    let width = 20000;
    let n = state.n;

    struct BeamState {
        board: Board,
        empty_pos: (usize, usize),
        score: usize,
        ans: String,
    }
    let first_state = BeamState {
        board: state.orig_board.clone(),
        empty_pos: state.orig_empty_pos.clone(),
        score: 0,
        ans: String::new(),
    };

    const MOD: u64 = 1e18 as u64 + 7;
    let mut pows = vec![];
    let mut c = 1;
    for i in 0..n * n {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &Board| -> u64 {
        let mut num = 0;
        for y in 0..n {
            for x in 0..n {
                let i = y * n + x;
                let p = pows[i] * board.get_pattern_num(y, x) as u64;
                num += p;
                num %= MOD;
            }
        }
        num
    };

    let mut used_hash = HashSet::new();
    let mut dirs_all = Dir::iter_all_dirs().collect::<Vec<_>>();
    let mut beam_states = vec![first_state];
    for _turn in 0..state.t {
        // for _turn in 0..5 {
        let mut new_beam_states = vec![];
        for bs in &beam_states {
            let (cy, cx) = bs.empty_pos;
            rng.shuffle(&mut dirs_all);
            for &dir in &dirs_all {
                // 逆方向に戻るのは NG
                let dir_rev_c = dir.rev().to_c();
                if !bs.ans.is_empty() && bs.ans.chars().last().unwrap() == dir_rev_c {
                    continue;
                }
                let (dy, dx) = dir.to_dyx();
                let (yy, xx) = (cy + dy, cx + dx);
                if !bs.board.in_area(yy, xx) {
                    continue;
                }
                let mut new_board = bs.board.clone();
                new_board.swap((cy, cx), (yy, xx));
                let new_empty_pos = (yy, xx);
                let hs = b_to_hash(&new_board);
                if used_hash.contains(&hs) {
                    continue;
                }
                used_hash.insert(hs);

                let new_ans = bs.ans.clone() + &dir.to_c().to_string();
                // println!("{:?} {:?} {:?}", (cy, cx), dir, new_empty_pos);
                let score = get_group_size(&new_board, 0, 0);
                let score2 = calc_board_score(&new_board);
                if score == n * n - 1 {
                    eprintln!("solve!");
                    return;
                }

                // let score = get_group_size(&new_board, n / 2, n / 2) as u64;
                let new_state = BeamState {
                    board: new_board,
                    empty_pos: new_empty_pos,
                    ans: new_ans,
                    score: score + score2,
                };
                new_beam_states.push(new_state);
            }
        }
        new_beam_states.sort_by_key(|s| Reverse(s.score));
        new_beam_states.truncate(width);
        rng.shuffle(&mut new_beam_states);
        beam_states = new_beam_states;
    }
    beam_states.sort_by_key(|f| f.score);
    for s in beam_states {
        println!("{:#?}", s.score);
        println!("{:#?}", s.ans);
    }
}

fn get_target_board_by_rand(state: &State, rng: &mut MyRng) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    let (cy, cx) = state.orig_empty_pos;

    for x in cx..n - 1 {
        board.swap((cy, x), (cy, x + 1));
    }
    for y in cy..n - 1 {
        board.swap((y, n - 1), (y + 1, n - 1));
    }
    // board.print_pattern();

    let check_wall_ok = |y: usize, x: usize, p: Pattern| -> bool {
        if y == 0 && (1 << 1) & *p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & *p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & *p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & *p > 0 {
            return false;
        }
        true
    };

    // board.swap(state.orig_empty_pos, (n - 1, n - 1));
    let mut cnt = 0;
    loop {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
            continue;
        }
        if (y1, x1) == (y2, x2) {
            continue;
        }
        if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
            continue;
        }

        let score = get_group_size(&board, 0, 0) as u64;
        // let score = calc_board_score(&board) as u64;
        board.swap((y1, x1), (y2, x2));
        cnt = if cnt == 1 { 0 } else { 1 };

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
            || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
        {
            board.swap((y1, x1), (y2, x2));
            cnt = if cnt == 1 { 0 } else { 1 };
        }

        let new_score = get_group_size(&board, 0, 0) as u64;
        // let new_score = calc_board_score(&board) as u64;
        // if new_score < score && rng.get_percent() < 0.999 {
        if new_score < score {
            board.swap((y1, x1), (y2, x2));
            cnt = if cnt == 1 { 0 } else { 1 };
        }

        // let score2 = get_group_size(&board, 0, 0) as u64;

        // dbg!(score);
        // dbg!(score2);
        if score as usize == n * n - 1 {
            // if score as usize == (n * n - 2) * 2 {
            // if cnt == 0 {
            // if cnt >= 0 {
            //     break;
            // } else {
            //     println!(" :( ");
            // }
            let (ans, last_score) = solve_by_target_board(&state, &board);
            if last_score as usize == n * n - 1 {
                break;
            } else {
                eprintln!(" :( ");
                for _ in 0..10000 {
                    let nn = n as i32;
                    let y1 = rng.get_int(0, nn - 1) as usize;
                    let x1 = rng.get_int(0, nn - 1) as usize;
                    let y2 = rng.get_int(0, nn - 1) as usize;
                    let x2 = rng.get_int(0, nn - 1) as usize;
                    if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
                        continue;
                    }
                    board.swap((y1, x1), (y2, x2));
                }
            }
        }
        // board.print_pattern();
    }
    // board.print_pattern();
    board
}

fn get_target_board_by_rand2(state: &State, rng: &mut MyRng) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    let (cy, cx) = state.orig_empty_pos;

    for x in cx..n - 1 {
        board.swap((cy, x), (cy, x + 1));
    }
    for y in cy..n - 1 {
        board.swap((y, n - 1), (y + 1, n - 1));
    }
    // board.print_pattern();

    let check_wall_ok = |y: usize, x: usize, p: Pattern| -> bool {
        if y == 0 && (1 << 1) & *p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & *p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & *p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & *p > 0 {
            return false;
        }
        true
    };

    // board.swap(state.orig_empty_pos, (n - 1, n - 1));
    let mut cnt = 0;
    loop {
        cnt = if cnt == 1 { 0 } else { 1 };
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
            continue;
        }

        // let score = get_group_size(&board, 0, 0) as u64;
        let score = calc_board_score(&board) as u64;
        board.swap((y1, x1), (y2, x2));

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
            || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
        {
            board.swap((y1, x1), (y2, x2));
        }

        // let new_score = get_group_size(&board, 0, 0) as u64;
        let new_score = calc_board_score(&board) as u64;
        if new_score < score {
            board.swap((y1, x1), (y2, x2));
        }

        // let score2 = get_group_size(&board, 0, 0) as u64;

        // dbg!(score);
        // dbg!(score2);
        // if score as usize == n * n - 1 {
        if score as usize == (n * n - 2) * 2 {
            if cnt == 0 {
                break;
            } else {
                // println!(" :( ");
            }
        }
        // board.print_pattern();
    }

    loop {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
            continue;
        }
        if (y1, x1) == (y2, x2) {
            continue;
        }
        if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
            continue;
        }

        cnt = if cnt == 1 { 0 } else { 1 };

        let score = get_group_size(&board, 0, 0) as u64;
        // let score = calc_board_score(&board) as u64;
        board.swap((y1, x1), (y2, x2));

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
            || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
        {
            board.swap((y1, x1), (y2, x2));
            cnt = if cnt == 1 { 0 } else { 1 };
        }

        let new_score = get_group_size(&board, 0, 0) as u64;
        // let new_score = calc_board_score(&board) as u64;
        if new_score < score {
            board.swap((y1, x1), (y2, x2));
            cnt = if cnt == 1 { 0 } else { 1 };
        }

        // let score2 = get_group_size(&board, 0, 0) as u64;

        // dbg!(score);
        // dbg!(score2);
        if score as usize == n * n - 1 {
            // if score as usize == (n * n - 2) * 2 {
            if cnt == 0 {
                break;
            } else {
                // println!(" :( ");
            }
        }
        // board.print_pattern();
    }

    board.print_pattern();
    board
}

fn get_group_len(board: &Board) -> usize {
    let n = board.n;
    let mut vis = vec![vec![false; n]; n];
    let mut ans = 0;
    for y in 0..n {
        for x in 0..n {
            if y == n - 1 && x == n - 1 {
                continue;
            }
            if vis[y][x] {
                continue;
            }

            let mut q = VecDeque::new();
            vis[y][x] = true;
            q.push_back((y, x));
            ans += 1;
            while let Some((cy, cx)) = q.pop_front() {
                let dirs = get_dirs_from_pattern(board.cells[cy][cx].pattern());
                for dir in dirs {
                    let (dy, dx) = dir.to_dyx();
                    let (yy, xx) = (cy + dy, cx + dx);
                    if !board.in_area(yy, xx) {
                        continue;
                    }
                    let pp = board.cells[yy][xx].pattern();
                    if !Pattern::is_matched(Pattern::from_single_dir(dir), pp) {
                        continue;
                    }
                    if vis[yy][xx] {
                        continue;
                    }
                    vis[yy][xx] = true;
                    q.push_back((yy, xx));
                }
            }
        }
    }
    ans
}

fn get_target_board_by_rand3(state: &State, rng: &mut MyRng, tm: &mut TimeManager) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    let (cy, cx) = state.orig_empty_pos;

    for x in cx..n - 1 {
        board.swap((cy, x), (cy, x + 1));
    }
    for y in cy..n - 1 {
        board.swap((y, n - 1), (y + 1, n - 1));
    }
    // board.print_pattern();
    let mut empty_pos = (n - 1, n - 1);
    let ess = vec![
        (n - 1, n - 1),
        (n - 2, n - 1),
        (n - 2, n - 2),
        (n - 1, n - 2),
    ];

    let first_board = board.clone();

    let check_wall_ok = |y: usize, x: usize, p: Pattern| -> bool {
        if y == 0 && (1 << 1) & *p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & *p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & *p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & *p > 0 {
            return false;
        }
        true
    };

    // let mut tmp_ans = Board::new(n);
    let mut tmp_ans = board.clone();
    let mut first = true;
    // board.swap(state.orig_empty_pos, (n - 1, n - 1));
    let mut cnt = 0;
    let mut tt = 0;
    let mut score = 0;
    const RETRY_CNT: usize = 150000;
    while tm.check_time(TL, false) {
        if rng.get_percent() < 1.9 {
            let nn = n as i32;
            let y1 = rng.get_int(0, nn - 1) as usize;
            let x1 = rng.get_int(0, nn - 1) as usize;
            let y2 = rng.get_int(0, nn - 1) as usize;
            let x2 = rng.get_int(0, nn - 1) as usize;
            // assert!(board.cells[empty_pos.0][empty_pos.1] == Cell::Empty);
            if (y1, x1) == empty_pos || (y2, x2) == empty_pos {
                continue;
            }
            if (y1, x1) == (y2, x2) {
                continue;
            }
            if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
                continue;
            }
            // println!("{:?}", empty_pos);

            cnt = if cnt == 1 { 0 } else { 1 };

            // let score_gl = get_group_len(&board);
            // let score_g = get_group_size(&board, 0, 0);
            // let score = calc_board_score(&board)  + score_g;
            // let score = score_g + (n * n - score_gl) * 1;
            // let score = score_g;

            if score == n * n - 2 {
                if first {
                    first = false;
                    tmp_ans = board.clone();
                }
                tt += 1;
            }

            board.swap((y1, x1), (y2, x2));

            // 壁に向かうパターンは採用しない
            if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
                || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
            {
                board.swap((y1, x1), (y2, x2));
                cnt = if cnt == 1 { 0 } else { 1 };
            }

            // let new_score_gl = get_group_len(&board);
            // let new_score_g = get_group_size(&board, 0, 0);
            let new_score_g = get_group_size_with_empty(&board, 0, 0, empty_pos);

            // let new_score = calc_board_score(&board) as u64 + new_score_g;
            // let new_score = new_score_g as u64 + (n * n - new_score_gl) as u64 * 1;
            let new_score = new_score_g;
            if new_score < score {
                board.swap((y1, x1), (y2, x2));
                cnt = if cnt == 1 { 0 } else { 1 };
            } else if new_score > score {
                score = new_score;
            }

            // let score2 = get_group_size(&board, 0, 0) as u64;

            // dbg!(score);
            // dbg!(get_group_len(&board));
            // if score as usize == n * n - 1 {
            // if score as usize == (n * n - 2) * 2 {
            // if score as usize == (n * n - 2) * 2 + n * n - 1 {
            // TOOD: ここのシャッフル閾値ちゃんとやる
            if score == n * n - 1 || tt > RETRY_CNT {
                let (ans, last_score) = solve_by_target_board(&state, &board);
                if tt > RETRY_CNT {
                    tt = 0;
                }

                if last_score as usize == n * n - 1 {
                    break;
                } else {
                    // eprintln!(" :( ");
                    // for _ in 0..1000 {
                    //     let nn = n as i32;
                    //     let y1 = rng.get_int(0, nn - 1) as usize;
                    //     let x1 = rng.get_int(0, nn - 1) as usize;
                    //     let y2 = rng.get_int(0, nn - 1) as usize;
                    //     let x2 = rng.get_int(0, nn - 1) as usize;
                    //     if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
                    //         continue;
                    //     }
                    //     board.swap((y1, x1), (y2, x2));
                    // }
                    // eprintln!("aa ");
                    board = first_board.clone();
                    // let n_empty_pos = rng.choose(&ess);
                    // assert!(
                    //     board.cells[empty_pos.0][empty_pos.1] == Cell::Empty,
                    //     "before {:?} {:?}",
                    //     board.cells,
                    //     empty_pos
                    // );
                    // board.swap(n_empty_pos, (n - 1, n - 1));
                    // assert!(
                    //     board.cells[n_empty_pos.0][n_empty_pos.1] == Cell::Empty,
                    //     "after"
                    // );

                    // score = get_group_size(&board, 0, 0);
                    // empty_pos = n_empty_pos;
                    score = get_group_size_with_empty(&board, 0, 0, empty_pos);
                    // eprintln!("swap")
                }
            }
        } else {
            let nn = n as i32;
            let y1 = rng.get_int(0, nn - 1) as usize;
            let x1 = rng.get_int(0, nn - 1) as usize;
            let y2 = rng.get_int(0, nn - 1) as usize;
            let x2 = rng.get_int(0, nn - 1) as usize;
            let y3 = rng.get_int(0, nn - 1) as usize;
            let x3 = rng.get_int(0, nn - 1) as usize;
            if (y1, x1) == (n - 1, n - 1)
                || (y2, x2) == (n - 1, n - 1)
                || (y3, x3) == (n - 1, n - 1)
            {
                continue;
            }
            // if (y1, x1) == (y2, x2) {
            //     continue;
            // }
            // if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
            //     continue;
            // }

            cnt = if cnt == 1 { 0 } else { 1 };

            // let score_gl = get_group_len(&board);
            // let score_g = get_group_size(&board, 0, 0);
            // let score = calc_board_score(&board) as u64 + score_g;
            // let score = score_g + (n * n - score_gl) as u64 * 1;
            // let score = score_g;

            if score == n * n - 2 {
                tt += 1;
            }

            board.swap((y1, x1), (y2, x2));
            board.swap((y2, x2), (y3, x3));

            // 壁に向かうパターンは採用しない
            if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
                || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
                || !check_wall_ok(y3, x3, board.cells[y3][x3].pattern())
            {
                board.swap((y2, x2), (y3, x3));
                board.swap((y1, x1), (y2, x2));
                cnt = if cnt == 1 { 0 } else { 1 };
            }

            // let new_score_gl = get_group_len(&board);
            let new_score_g = get_group_size(&board, 0, 0);
            // let new_score = calc_board_score(&board) as u64 + new_score_g;
            // let new_score = new_score_g + (n * n - new_score_gl) * 1;
            let new_score = new_score_g;
            if new_score < score {
                board.swap((y2, x2), (y3, x3));
                board.swap((y1, x1), (y2, x2));

                cnt = if cnt == 1 { 0 } else { 1 };
            } else {
                score = new_score;
            }

            // let score2 = get_group_size(&board, 0, 0) as u64;

            // dbg!(get_group_len(&board));
            // if score as usize == n * n - 1 {
            // if score as usize == (n * n - 2) * 2 {
            // if score as usize == (n * n - 2) * 2 + n * n - 1 {
            if score == n * n - 1 || tt > RETRY_CNT {
                // tt = 0;
                if tt > RETRY_CNT {
                    tt = 0;
                }
                let (ans, last_score) = solve_by_target_board(&state, &board);

                if last_score as usize == n * n - 1 {
                    break;
                } else {
                    // println!(" :( ");
                    for _ in 0..10000 {
                        let nn = n as i32;
                        let y1 = rng.get_int(0, nn - 1) as usize;
                        let x1 = rng.get_int(0, nn - 1) as usize;
                        let y2 = rng.get_int(0, nn - 1) as usize;
                        let x2 = rng.get_int(0, nn - 1) as usize;
                        if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
                            continue;
                        }
                        board.swap((y1, x1), (y2, x2));
                    }
                    score = get_group_size(&board, 0, 0);
                }
            }
        }
        // println!();
        // board.print_pattern();
        // dbg!(tt, score);
    }
    dbg!(tt);
    // board.print_pattern();
    if score == n * n - 1 {
        board
    } else {
        tmp_ans
    }
}

fn get_target_board_by_rand4(state: &State, rng: &mut MyRng, tm: &mut TimeManager) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    let (cy, cx) = state.orig_empty_pos;

    for x in cx..n - 1 {
        board.swap((cy, x), (cy, x + 1));
    }
    for y in cy..n - 1 {
        board.swap((y, n - 1), (y + 1, n - 1));
    }
    // board.print_pattern();

    let check_wall_ok = |y: usize, x: usize, p: Pattern| -> bool {
        if y == 0 && (1 << 1) & *p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & *p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & *p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & *p > 0 {
            return false;
        }
        true
    };
    let mut orig_pos = (0..n)
        .map(|i| (0..n).map(|j| (i, j)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let swap_pos = |p1: (usize, usize), p2: (usize, usize), pos: &mut [Vec<(usize, usize)>]| {
        let (y1, x1) = p1;
        let (y2, x2) = p2;
        let tmp = pos[y1][x1];
        pos[y1][x1] = pos[y2][x2];
        pos[y2][x2] = tmp;
    };
    // let thre = n / 2;
    let thre = 5;
    let check_dist_ok = |p1: (usize, usize), p2: (usize, usize)| {
        // let d = p1.0.max(p2.0) - p1.0.min(p2.0) + p1.1.max(p2.1) - p1.1.min(p2.1);
        let d = (p1.0.max(p2.0) - p1.0.min(p2.0)) + (p1.1.max(p2.1) - p1.1.min(p2.1));
        d <= thre
    };

    let start_board = board.clone();
    // let mut tmp_ans = Board::new(n);
    let mut tmp_ans = board.clone();

    let mut first = true;
    // board.swap(state.orig_empty_pos, (n - 1, n - 1));
    let mut cnt = 0;
    let mut tt = 0;
    let mut score = 0;
    const RETRY_CNT: usize = 100000;
    while tm.check_time(TL, false) {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
            continue;
        }
        if (y1, x1) == (y2, x2) {
            continue;
        }
        if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
            continue;
        }
        let op1 = orig_pos[y1][x1];
        let op2 = orig_pos[y2][x2];
        if !check_dist_ok(op2, (y1, x1)) || !check_dist_ok(op1, (y2, x2)) {
            continue;
        }

        cnt = if cnt == 1 { 0 } else { 1 };

        // let score_gl = get_group_len(&board);
        // let score_g = get_group_size(&board, 0, 0);
        // let score = calc_board_score(&board)  + score_g;
        // let score = score_g + (n * n - score_gl) * 1;
        // let score = score_g;

        if score == n * n - 2 {
            if first {
                first = false;
                tmp_ans = board.clone();
            }
            tt += 1;
        }

        board.swap((y1, x1), (y2, x2));
        swap_pos((y1, x1), (y2, x2), &mut orig_pos);

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
            || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
        {
            board.swap((y1, x1), (y2, x2));
            swap_pos((y1, x1), (y2, x2), &mut orig_pos);
            cnt = if cnt == 1 { 0 } else { 1 };
        }

        // let new_score_gl = get_group_len(&board);
        let new_score_g = get_group_size(&board, 0, 0);
        // let new_score = calc_board_score(&board) as u64 + new_score_g;
        // let new_score = new_score_g as u64 + (n * n - new_score_gl) as u64 * 1;
        let new_score = new_score_g;
        if new_score < score {
            board.swap((y1, x1), (y2, x2));
            swap_pos((y1, x1), (y2, x2), &mut orig_pos);
            cnt = if cnt == 1 { 0 } else { 1 };
        } else {
            score = new_score;
        }

        // let score2 = get_group_size(&board, 0, 0) as u64;

        // dbg!(score);
        // dbg!(get_group_len(&board));
        // if score as usize == n * n - 1 {
        // if score as usize == (n * n - 2) * 2 {
        // if score as usize == (n * n - 2) * 2 + n * n - 1 {
        // TOOD: ここのシャッフル閾値ちゃんとやる
        if score == n * n - 1 || tt > RETRY_CNT {
            let (ans, last_score) = solve_by_target_board(&state, &board);
            if tt > RETRY_CNT {
                tt = 0;
            }

            if last_score as usize == n * n - 1 {
                break;
            } else {
                // eprintln!(" :( ");
                // これ純粋に初期状態に戻すでいいのでは
                // for _ in 0..10000 {
                //     let nn = n as i32;
                //     let y1 = rng.get_int(0, nn - 1) as usize;
                //     let x1 = rng.get_int(0, nn - 1) as usize;
                //     let y2 = rng.get_int(0, nn - 1) as usize;
                //     let x2 = rng.get_int(0, nn - 1) as usize;
                //     if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
                //         continue;
                //     }
                //     board.swap((y1, x1), (y2, x2));
                //     swap_pos((y1, x1), (y2, x2), &mut orig_pos);
                // }
                board = start_board.clone();
                score = get_group_size(&board, 0, 0);
            }
        }
    }
    dbg!(tt);
    // board.print_pattern();
    if score == n * n - 1 {
        board
    } else {
        tmp_ans
    }
}

fn get_target_board_by_rand5(state: &State, rng: &mut MyRng, tm: &mut TimeManager) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    let (cy, cx) = state.orig_empty_pos;

    for x in cx..n - 1 {
        board.swap((cy, x), (cy, x + 1));
    }
    for y in cy..n - 1 {
        board.swap((y, n - 1), (y + 1, n - 1));
    }
    // board.print_pattern();

    let check_wall_ok = |y: usize, x: usize, p: Pattern| -> bool {
        if y == 0 && (1 << 1) & *p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & *p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & *p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & *p > 0 {
            return false;
        }
        true
    };
    let mut orig_pos = (0..n)
        .map(|i| (0..n).map(|j| (i, j)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let swap_pos = |p1: (usize, usize), p2: (usize, usize), pos: &mut [Vec<(usize, usize)>]| {
        let (y1, x1) = p1;
        let (y2, x2) = p2;
        let tmp = pos[y1][x1];
        pos[y1][x1] = pos[y2][x2];
        pos[y2][x2] = tmp;
    };
    // const THRE: usize = 2;
    let check_area_ok = |p1: (usize, usize), p2: (usize, usize)| {
        let thre = n / 2;
        p1.0 / thre == p2.0 && p1.1 / thre == p2.1 / thre
        // d <= THRE
    };

    // let mut tmp_ans = Board::new(n);
    let mut tmp_ans = board.clone();

    let mut first = true;
    // board.swap(state.orig_empty_pos, (n - 1, n - 1));
    let mut cnt = 0;
    let mut tt = 0;
    let mut score = 0;
    const RETRY_CNT: usize = 100000;
    while tm.check_time(TL, false) {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
            continue;
        }
        if (y1, x1) == (y2, x2) {
            continue;
        }
        if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
            continue;
        }
        let op1 = orig_pos[y1][x1];
        let op2 = orig_pos[y2][x2];
        if y1 < 3 && x1 < 3 {
            if !check_area_ok(op2, (y1, x1)) {
                continue;
            }
        }
        if y2 < 3 && x2 < 3 {
            if !check_area_ok(op1, (y2, x2)) {
                continue;
            }
        }
        // }

        cnt = if cnt == 1 { 0 } else { 1 };

        // let score_gl = get_group_len(&board);
        // let score_g = get_group_size(&board, 0, 0);
        // let score = calc_board_score(&board)  + score_g;
        // let score = score_g + (n * n - score_gl) * 1;
        // let score = score_g;

        if score == n * n - 2 {
            if first {
                first = false;
                tmp_ans = board.clone();
            }
            tt += 1;
        }

        board.swap((y1, x1), (y2, x2));
        swap_pos((y1, x1), (y2, x2), &mut orig_pos);

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y1][x1].pattern())
            || !check_wall_ok(y2, x2, board.cells[y2][x2].pattern())
        {
            board.swap((y1, x1), (y2, x2));
            swap_pos((y1, x1), (y2, x2), &mut orig_pos);
            cnt = if cnt == 1 { 0 } else { 1 };
        }

        // let new_score_gl = get_group_len(&board);
        let new_score_g = get_group_size(&board, 0, 0);
        // let new_score = calc_board_score(&board) as u64 + new_score_g;
        // let new_score = new_score_g as u64 + (n * n - new_score_gl) as u64 * 1;
        let new_score = new_score_g;
        if new_score < score {
            board.swap((y1, x1), (y2, x2));
            swap_pos((y1, x1), (y2, x2), &mut orig_pos);
            cnt = if cnt == 1 { 0 } else { 1 };
        } else {
            score = new_score;
        }

        // let score2 = get_group_size(&board, 0, 0) as u64;

        // dbg!(score);
        // dbg!(get_group_len(&board));
        // if score as usize == n * n - 1 {
        // if score as usize == (n * n - 2) * 2 {
        // if score as usize == (n * n - 2) * 2 + n * n - 1 {
        // TOOD: ここのシャッフル閾値ちゃんとやる
        if score == n * n - 1 || tt > RETRY_CNT {
            let (ans, last_score) = solve_by_target_board(&state, &board);
            if tt > RETRY_CNT {
                tt = 0;
            }

            if last_score as usize == n * n - 1 {
                break;
            } else {
                // eprintln!(" :( ");
                // これ純粋に初期状態に戻すでいいのでは
                for _ in 0..10000 {
                    let nn = n as i32;
                    let y1 = rng.get_int(0, nn - 1) as usize;
                    let x1 = rng.get_int(0, nn - 1) as usize;
                    let y2 = rng.get_int(0, nn - 1) as usize;
                    let x2 = rng.get_int(0, nn - 1) as usize;
                    if (y1, x1) == (n - 1, n - 1) || (y2, x2) == (n - 1, n - 1) {
                        continue;
                    }
                    // board.swap((y1, x1), (y2, x2));
                    // swap_pos((y1, x1), (y2, x2), &mut orig_pos);
                }
                score = get_group_size(&board, 0, 0);
            }
        }
    }
    dbg!(tt);
    // board.print_pattern();
    if score == n * n - 1 {
        board
    } else {
        tmp_ans
    }
}

// y, x: empty cell
fn get_nearest_pattern_cell(
    sy: usize,
    sx: usize,
    p: Pattern,
    board: &Board,
    free_cells: &[Vec<bool>],
) -> (usize, usize) {
    let n = board.n;
    let mut min_dist = 100000;
    let (mut cy, mut cx) = (!0, !0);
    for y in 0..n {
        for x in 0..n {
            if !free_cells[y][x] {
                continue;
            }
            match board.cells[y][x] {
                Cell::Empty => (),
                Cell::Piece(piece) => {
                    if piece.pattern == p {
                        let dist = sy.max(y) - sy.min(y) + sx.max(x) - sx.min(x);
                        if dist < min_dist {
                            min_dist = dist;
                            // (cy, cx) = (y, x);
                            cy = y;
                            cx = x;
                        }
                    }
                }
            }
        }
    }
    (cy, cx)
}

fn get_routes(
    start: (usize, usize),
    goal: (usize, usize),
    board: &mut Board,
    free_cells: &[Vec<bool>],
    ng_cells: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let n = board.n;
    let mut q = VecDeque::new();
    let mut prevs = vec![vec![None; n]; n];
    q.push_back(start);
    prevs[start.0][start.1] = Some((!0, !0));

    // println!("   get_route {:?} {:?}", start, goal);
    while let Some(pos) = q.pop_front() {
        if pos == goal {
            break;
        }
        let (y, x) = pos;
        for &(dy, dx) in &DYXS {
            let (yy, xx) = (y + dy, x + dx);
            if !board.in_area(yy, xx) && (yy != n - 1 || xx != n - 1) {
                continue;
            }
            if !free_cells[yy][xx] || ng_cells.contains(&(yy, xx)) {
                continue;
            }
            if prevs[yy][xx].is_some() {
                continue;
            }
            prevs[yy][xx] = Some((y, x));
            q.push_back((yy, xx));
        }
    }
    let mut ans = vec![goal];
    let mut c_pos = goal;
    loop {
        // print!("{:?}", c_pos);
        c_pos = prevs[c_pos.0][c_pos.1].unwrap();
        if c_pos == (!0, !0) {
            break;
        }
        ans.push(c_pos);
    }
    // println!();
    ans.reverse();
    ans
}

fn get_dir_char_from_swap(empty: (usize, usize), cell: (usize, usize)) -> char {
    let (ey, ex) = empty;
    let (cy, cx) = cell;
    if ey == cy {
        if ex < cx {
            'R'
        } else {
            'L'
        }
    } else {
        if ey < cy {
            'D'
        } else {
            'U'
        }
    }
}

// コマンドと empty_cell の最終位置
fn carry_cell(
    cell_start: (usize, usize),
    cell_goal: (usize, usize),
    empty_cell: (usize, usize),
    board: &mut Board,
    free_cells: &[Vec<bool>],
) -> (Vec<char>, (usize, usize)) {
    // セルの現在地
    let (mut cy, mut cx) = cell_start;
    let (ty, tx) = cell_goal;
    // empty の現在地
    let (mut ey, mut ex) = empty_cell;

    let mut rng = mytool::MyRng::new();

    // println!("carry_cell {:?} -> {:?} ", cell_start, cell_goal);

    let mut ans = vec![];
    while cy != ty || cx != tx {
        let mut ng_cells = HashSet::new();
        ng_cells.insert((cy, cx));

        // println!("{:?} ", (cy, cx));

        // println!(" cell: {:?} empty: {:?}", (cy, cx), (ey, ex));
        // 方向、つける位置、ルート
        let dir_pos_route = (0..4)
            .filter_map(|di| {
                if di == 0 && tx < cx && free_cells[cy][cx - 1] {
                    Some(((0, !0), (cy, cx - 1)))
                } else if di == 1 && ty < cy && free_cells[cy - 1][cx] {
                    Some(((!0, 0), (cy - 1, cx)))
                } else if di == 2 && cx < tx && free_cells[cy][cx + 1] {
                    Some(((0, 1), (cy, cx + 1)))
                } else if di == 3 && cy < ty && free_cells[cy + 1][cx] {
                    Some(((1, 0), (cy + 1, cx)))
                } else {
                    None
                }
            })
            .map(|v| {
                // println!("  {:?}", v);
                let epos = (ey, ex);
                let routes = get_routes(epos, v.1, board, free_cells, &ng_cells);
                let rr = rng.get_int(0, 100);

                (v.0, v.1, routes, rr)
            })
            .min_by_key(|v| (v.2.len(), v.3))
            .unwrap();
        let route = dir_pos_route.2;
        for i in 0..route.len() - 1 {
            let p1 = route[i];
            let p2 = route[i + 1];
            board.swap(p1, p2);
            ans.push(get_dir_char_from_swap(p1, p2));
        }
        board.swap(dir_pos_route.1, (cy, cx));
        ans.push(get_dir_char_from_swap(dir_pos_route.1, (cy, cx)));

        let dir = dir_pos_route.0;
        // (ey, ex) = (cy, cx);
        // (cy, cx) = (cy + dir.0, cx + dir.1);
        ey = cy;
        ex = cx;
        cy = cy + dir.0;
        cx = cx + dir.1;
    }
    (ans, (ey, ex))
    // let t = (0..4).filter_map(|di|{
    //     if
    // });
}

fn move_empty_cell_by_commands(
    com: String,
    board: &mut Board,
    emp: (usize, usize),
) -> (usize, usize) {
    let (mut ey, mut ex) = emp;
    for c in com.chars() {
        let (dy, dx) = match c {
            'L' => (0, !0),
            'U' => (!0, 0),
            'R' => (0, 1),
            'D' => (1, 0),
            _ => panic!(),
        };
        let (yy, xx) = (ey + dy, ex + dx);
        board.swap((yy, xx), (ey, ex));
        // (ey, ex) = (yy, xx);
        ey = yy;
        ex = xx;
    }
    (ey, ex)
}

fn solve_by_target_board(state: &State, target: &Board) -> (Vec<char>, u64) {
    let n = state.n;
    let mut board = state.orig_board.clone();
    let mut free_cells = vec![vec![true; n]; n];
    let (mut ey, mut ex) = state.orig_empty_pos;
    let dummy = HashSet::new();

    let mut ans = vec![];
    for i in 0..n - 2 {
        // println!("x1");

        for x in i..n - 2 {
            let y = i;
            let t_pattern = target.cells[y][x].pattern();
            let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);
            free_cells[y][x] = false;
        }
        // println!("y: {} -> {}", i, ans.len());

        // println!("x2");

        // last 2
        let y = i;
        let x = n - 2;
        let t_pattern = target.cells[i][n - 1].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
        // println!(" com: {:?}", com);
        // (ey, ex) = (c_ey, c_ex);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y][x] = false;

        // if (ey, ex) == (i, n - 1) {
        //     board.swap((ey, ex), (ey + 1, ex));
        //     ey += 1;
        //     ans.push('D');
        // }
        // 4 のましたにemptyがなければ持っていく
        if (ey, ex) != (i + 1, n - 2) {
            let route = get_routes((ey, ex), (i + 1, n - 2), &mut board, &free_cells, &dummy);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            // (ey, ex) = (i + 1, n - 2);
            ey = i + 1;
            ex = n - 2;
        }

        let s = ans.iter().collect::<String>();
        // println!("{}", s);

        let y = i + 1;
        let x = n - 2;
        let t_pattern = target.cells[i][n - 2].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);

        if (ty, tx) != (i, n - 1) {
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);
            free_cells[y][x] = false;
            let mut route = get_routes((ey, ex), (i, n - 1), &mut board, &free_cells, &dummy);
            route.extend(vec![(i, n - 2), (i + 1, n - 2)]);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            free_cells[i + 1][n - 2] = true;
            // (ey, ex) = (i + 1, n - 2);
            ey = i + 1;
            ex = n - 2;
        } else {
            if ex == n - 1 {
                board.swap((ey, ex), (ey, ex - 1));
                ex -= 1;
                ans.push('L');
            }
            let (c_ey, c_ex) = move_empty_cell_by_commands(
                "URDLDRULURDLURDDLUURD".to_string(),
                &mut board,
                (ey, ex),
            );
            ey = c_ey;
            ex = c_ex;
            ans.extend("URDLDRULURDLURDDLUURD".chars());
            // URDLDRULURDLURDDLUURD
            // RULDRDLUURDLU (miss?)
        }
        free_cells[i][n - 1] = false;

        // break;
        // println!("y2: {} -> {}", i, ans.len());

        // println!("y1");

        for y in i + 1..n - 2 {
            let x = i;
            let t_pattern = target.cells[y][x].pattern();
            let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);
            free_cells[y][x] = false;
        }

        // println!("x: {} -> {}", i, ans.len());

        // println!("y2");
        // println!("eee {:?}", (ey, ex));
        // last 2
        let y = n - 2;
        let x = i;
        let t_pattern = target.cells[n - 1][i].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
        // (ey, ex) = (c_ey, c_ex);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y][x] = false;

        // if (ey, ex) == (n - 1, i) {
        //     board.swap((ey, ex), (ey, ex + 1));
        //     ex += 1;
        //     ans.push('R');
        // }
        if (ey, ex) != (n - 2, i + 1) {
            let route = get_routes((ey, ex), (n - 2, i + 1), &mut board, &free_cells, &dummy);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            // (ey, ex) = (n - 2, i + 1);
            ey = n - 2;
            ex = i + 1;
        }

        let y = n - 2;
        let x = i + 1;
        let t_pattern = target.cells[n - 2][i].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);

        if (ty, tx) != (n - 1, i) {
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);

            free_cells[y][x] = false;
            let mut route = get_routes((ey, ex), (n - 1, i), &mut board, &free_cells, &dummy);
            route.extend(vec![(n - 2, i), (n - 2, i + 1)]);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            free_cells[n - 2][i + 1] = true;
            free_cells[n - 1][i] = false;
            // (ey, ex) = (n - 2, i + 1);
            ey = n - 2;
            ex = i + 1;
        } else {
            if ey == n - 1 {
                board.swap((ey, ex), (ey - 1, ex));
                ey -= 1;
                ans.push('U');
            }
            let (c_ey, c_ex) = move_empty_cell_by_commands(
                "LDRURDLULDRULDRRULLDR".to_string(),
                &mut board,
                (ey, ex),
            );
            ey = c_ey;
            ex = c_ex;
            ans.extend("LDRURDLULDRULDRRULLDR".chars());
            // free_cells[n - 2][i + 1] = true;
            free_cells[n - 1][i] = false;
        }

        // println!("x2: {} -> {}", i, ans.len());
    }

    // TODO: 最後に4個を回す
    for _ in 0..20 {
        if (ey, ex) == (n - 2, n - 2) {
            board.swap((ey, ex), (ey + 1, ex));
            ey += 1;
            ans.push('D');
        } else if (ey, ex) == (n - 1, n - 2) {
            board.swap((ey, ex), (ey, ex + 1));
            ex += 1;
            ans.push('R');
        } else if (ey, ex) == (n - 1, n - 1) {
            board.swap((ey, ex), (ey - 1, ex));
            ey -= 1;
            ans.push('U');
        } else if (ey, ex) == (n - 2, n - 1) {
            board.swap((ey, ex), (ey, ex - 1));
            ex -= 1;
            ans.push('L');
        }
        if get_group_size(&board, 0, 0) == n * n - 1 {
            break;
        }
    }

    (ans, get_group_size(&board, 0, 0) as u64)
}

// target は余分な下の段はなし
fn solve_puzzle_ng(
    pn: usize,
    initial_board: &[Vec<usize>],
    target: &[Vec<usize>],
) -> Vec<(usize, usize)> {
    // assert_eq!(initial_board.len(), target.len());
    // assert_eq!(initial_board[0].len(), target[0].len());

    println!("{:?}", initial_board);
    println!("{:?}", target);
    let width = 2000;
    #[derive(Clone)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<usize>>,
        score: usize,
        pos_0: (usize, usize),
    }
    let mut pos_0 = (0, 0);
    for y in 0..pn + 1 {
        for x in 0..pn {
            if initial_board[y][x] == 0 {
                pos_0 = (y, x);
                break;
            }
        }
    }
    let first_state = BeamState {
        ans: vec![],
        board: initial_board.to_vec(),
        score: 999,
        pos_0,
    };
    let swap = |y1: usize, x1: usize, y2: usize, x2: usize, board: &mut [Vec<usize>]| {
        let tmp = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = tmp;
    };
    let in_area = |y: usize, x: usize| -> bool { (0..pn + 1).contains(&y) && (0..pn).contains(&x) };
    let calc_score = |board: &[Vec<usize>]| -> usize {
        let mut ans = 0;
        for y in 0..pn {
            for x in 0..pn {
                let val = target[y][x];
                let mut min_d = 10000;
                for yy in 0..pn + 1 {
                    for xx in 0..pn {
                        if board[yy][xx] == val {
                            let d = y.max(yy) - y.min(yy) + x.max(xx) - x.min(xx);
                            min_d = min_d.min(d);
                        }
                    }
                }
                ans += min_d;
            }
        }
        ans
    };
    let mut states = vec![first_state];
    for _ in 0..100 {
        let mut new_states = vec![];
        for state in &states {
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }
                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);
                new_state.score = calc_score(&new_state.board);
                new_state.ans.push((dy, dx));
                new_state.pos_0 = (yy, xx);
                // dbg!(new_state.score);

                if new_state.score == 0 {
                    // dbg!(new_state.board);
                    return new_state.ans;
                }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        new_states.truncate(width);
        states = new_states;
    }
    vec![]
}

fn solve_puzzle(
    ny: usize,
    nx: usize,
    initial_board: &[Vec<usize>],
    target: &[Vec<usize>],
    tm: &mut TimeManager,
) -> Vec<(usize, usize)> {
    // println!("{:?}", initial_board);
    // println!("{:?}", target);
    let width = 3000;
    #[derive(Clone)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<usize>>,
        score: usize,
        pos_0: (usize, usize),
    }
    let mut pos_0 = (0, 0);
    for y in 0..ny {
        for x in 0..nx {
            if initial_board[y][x] == 0 {
                pos_0 = (y, x);
                break;
            }
        }
    }
    let first_state = BeamState {
        ans: vec![],
        board: initial_board.to_vec(),
        score: 999,
        pos_0,
    };
    let swap = |y1: usize, x1: usize, y2: usize, x2: usize, board: &mut [Vec<usize>]| {
        let tmp = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = tmp;
    };
    let in_area = |y: usize, x: usize| -> bool { (0..ny).contains(&y) && (0..nx).contains(&x) };
    let calc_score = |board: &[Vec<usize>]| -> usize {
        let mut used = vec![vec![false; nx]; ny];
        let mut ans = 0;
        for y in 0..ny {
            for x in 0..nx {
                let val = target[y][x];
                let mut min_d = 10000;
                let (mut min_y, mut min_x) = (0, 0);
                for yy in 0..ny {
                    for xx in 0..nx {
                        if board[yy][xx] == val && !used[yy][xx] {
                            let d = y.max(yy) - y.min(yy) + x.max(xx) - x.min(xx);
                            if d < min_d {
                                min_d = d;
                                min_y = yy;
                                min_x = xx;
                            }
                        }
                    }
                }
                ans += min_d;
                used[min_y][min_x] = true;
            }
        }
        ans
    };

    const MOD: u64 = 1e9 as u64 + 7;
    let mut pows = vec![];
    let mut c = 1;
    for i in 0..ny * nx {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &[Vec<usize>]| -> u64 {
        let mut num = 0;
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                let p = pows[i] * board[y][x] as u64;
                num += p;
                num %= MOD;
            }
        }
        num
    };
    let mut used_hash = HashSet::new();
    let mut states = vec![first_state];
    for _ in 0..1000 {
        if !tm.check_time(TL, true) {
            return vec![];
        }
        let mut new_states = vec![];
        for state in &states {
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }
                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);

                let hs = b_to_hash(&new_state.board);
                if used_hash.contains(&hs) {
                    continue;
                }
                used_hash.insert(hs);

                new_state.score = calc_score(&new_state.board);
                new_state.ans.push((dy, dx));
                new_state.pos_0 = (yy, xx);
                // dbg!(new_state.score);

                if new_state.score == 0 {
                    // dbg!(new_state.board);
                    return new_state.ans;
                }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        new_states.truncate(width);
        states = new_states;
    }
    vec![]
}

fn solve_puzzle2(
    ny: usize,
    nx: usize,
    initial_board: &[Vec<usize>],
    target: &[Vec<usize>],
    tm: &mut TimeManager,
) -> Vec<(usize, usize)> {
    // println!("{:?}", initial_board);
    // println!("{:?}", target);
    let width = 1500;
    #[derive(Clone)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<usize>>,
        score: usize,
        pos_0: (usize, usize),
    }
    let mut pos_0 = (0, 0);
    for y in 0..ny {
        for x in 0..nx {
            if initial_board[y][x] == 0 {
                pos_0 = (y, x);
                break;
            }
        }
    }
    let first_state = BeamState {
        ans: vec![],
        board: initial_board.to_vec(),
        score: 999,
        pos_0,
    };

    let (mut last_ey, mut last_ex) = (ny - 1, nx - 1);
    for y in 0..ny {
        for x in 0..nx {
            if target[y][x] == 0 {
                last_ey = y;
                last_ex = x;
            }
        }
    }
    let last_state = BeamState {
        ans: vec![],
        board: target.to_vec(),
        score: 999,
        pos_0: (last_ey, last_ex),
    };

    let swap = |y1: usize, x1: usize, y2: usize, x2: usize, board: &mut [Vec<usize>]| {
        let tmp = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = tmp;
    };
    let in_area = |y: usize, x: usize| -> bool { (0..ny).contains(&y) && (0..nx).contains(&x) };
    let calc_score = |board: &[Vec<usize>], this_target: &[Vec<usize>]| -> usize {
        let mut used = vec![vec![false; nx]; ny];
        let mut ans = 0;
        for y in 0..ny {
            for x in 0..nx {
                let val = this_target[y][x];
                if val == 0 {
                    continue;
                }
                let mut min_d = 10000;
                let (mut min_y, mut min_x) = (0, 0);
                for yy in 0..ny {
                    for xx in 0..nx {
                        if board[yy][xx] == val && !used[yy][xx] {
                            let d = y.max(yy) - y.min(yy) + x.max(xx) - x.min(xx);
                            if d < min_d {
                                min_d = d;
                                min_y = yy;
                                min_x = xx;
                            }
                        }
                    }
                }
                ans += min_d;
                used[min_y][min_x] = true;
            }
        }
        ans
    };

    // const MOD: u64 = 1e9 as u64 + 9;
    const MOD: u64 = 1e18 as u64;
    let mut pows = vec![];
    let mut c = 1;
    for i in 0..ny * nx {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &[Vec<usize>]| -> u64 {
        let mut num = 0;
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                let p = pows[i] * board[y][x] as u64;
                num += p;
                num %= MOD;
            }
        }
        num
    };
    let mut used_hash = HashMap::<u64, Vec<(usize, usize)>>::new();
    let mut used_hash_rev = HashMap::<u64, Vec<(usize, usize)>>::new();

    let mut states = vec![first_state];
    let mut states_rev = vec![last_state];
    for _ in 0..1000 {
        // 前から
        if !tm.check_time(TL, false) {
            eprintln!("tttt: {}", tm.get_time());
            return vec![];
        }
        let mut new_states = vec![];
        for state in &states {
            if !tm.check_time(TL, true) {
                eprintln!("tttt: {}", tm.get_time());

                return vec![];
            }
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }
                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);

                let hs = b_to_hash(&new_state.board);
                if used_hash.contains_key(&hs) {
                    continue;
                }

                new_state.score = calc_score(&new_state.board, target);
                new_state.ans.push((dy, dx));
                used_hash.insert(hs, new_state.ans.clone());
                if used_hash_rev.contains_key(&hs) {
                    let mut rev_ans = used_hash_rev[&hs].clone();
                    rev_ans.reverse();
                    let rev_ans = rev_ans.into_iter().map(|d| rev_dyx(d)).collect::<Vec<_>>();
                    // println!("{:?}", new_state.ans);
                    // println!("{:?}", rev_ans);
                    let ans = [new_state.ans, rev_ans].concat();
                    return ans;
                }

                new_state.pos_0 = (yy, xx);
                // dbg!(new_state.score);

                // if new_state.score == 0 {
                //     // dbg!(new_state.board);
                //     return new_state.ans;
                // }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        new_states.truncate(width);
        states = new_states;

        // 後ろから
        let mut new_states = vec![];
        for state in &states_rev {
            if !tm.check_time(TL, true) {
                eprintln!("tttt: {}", tm.get_time());

                return vec![];
            }
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }
                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);

                let hs = b_to_hash(&new_state.board);
                if used_hash_rev.contains_key(&hs) {
                    continue;
                }

                new_state.score = calc_score(&new_state.board, initial_board);
                new_state.ans.push((dy, dx));
                used_hash_rev.insert(hs, new_state.ans.clone());

                if used_hash.contains_key(&hs) {
                    let ans = used_hash[&hs].clone();
                    // ans.reverse();
                    let mut rev_ans = new_state.ans.clone();
                    rev_ans.reverse();
                    let rev_ans = rev_ans.into_iter().map(|d| rev_dyx(d)).collect::<Vec<_>>();
                    // println!("{:?} {:?}", ans, rev_ans);
                    let ans = [ans, rev_ans].concat();
                    // eprintln!("{}", ans.len());
                    return ans;
                }
                new_state.pos_0 = (yy, xx);
                // dbg!(new_state.score);

                // if new_state.score == 0 {
                //     // dbg!(new_state.board);
                //     return new_state.ans;
                // }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        new_states.truncate(width);
        states_rev = new_states;
    }
    vec![]
}

fn solve_puzzle3(
    ny: usize,
    nx: usize,
    initial_board: &[Vec<usize>],
    target: &[Vec<usize>],
    tm: &mut TimeManager,
) -> Vec<(usize, usize)> {
    // println!("{:?}", initial_board);
    // println!("{:?}", target);
    let width = 500;
    #[derive(Clone)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<u8>>,
        score: usize,
        pos_0: (usize, usize),
        hs: u128,
    }
    let mut pos_0 = (0, 0);
    for y in 0..ny {
        for x in 0..nx {
            if initial_board[y][x] == 0 {
                pos_0 = (y, x);
                break;
            }
        }
    }

    let (mut last_ey, mut last_ex) = (ny - 1, nx - 1);
    for y in 0..ny {
        for x in 0..nx {
            if target[y][x] == 0 {
                last_ey = y;
                last_ex = x;
            }
        }
    }

    let swap = |y1: usize, x1: usize, y2: usize, x2: usize, board: &mut [Vec<u8>]| {
        let tmp = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = tmp;
    };
    let in_area = |y: usize, x: usize| -> bool { (0..ny).contains(&y) && (0..nx).contains(&x) };
    let calc_score = |board: &[Vec<u8>], this_target: &[Vec<u8>]| -> usize {
        let mut used = vec![vec![false; nx]; ny];
        let mut ans = 0;
        for y in 0..ny {
            for x in 0..nx {
                let val = this_target[y][x];
                if val == 0 {
                    continue;
                }
                let mut min_d = 10000;
                let (mut min_y, mut min_x) = (0, 0);
                for yy in 0..ny {
                    for xx in 0..nx {
                        if board[yy][xx] == val && !used[yy][xx] {
                            let d = y.max(yy) - y.min(yy) + x.max(xx) - x.min(xx);
                            if d < min_d {
                                min_d = d;
                                min_y = yy;
                                min_x = xx;
                            }
                        }
                    }
                }
                ans += min_d;
                used[min_y][min_x] = true;
            }
        }
        ans
    };
    let calc_score_one = |board: &[Vec<u8>], this_target: &[Vec<u8>], num: u8| -> usize {
        let mut poss = vec![];
        let mut t_poss = vec![];
        for yy in 0..ny {
            for xx in 0..nx {
                if board[yy][xx] == num {
                    poss.push((yy, xx));
                }
                if this_target[yy][xx] == num {
                    t_poss.push((yy, xx));
                }
            }
        }
        let mut used = vec![false; t_poss.len()];
        let mut res = 0;
        for (py, px) in poss {
            let mut min_d = 10000;
            let mut mi = 0;
            for (i, &(ty, tx)) in t_poss.iter().enumerate() {
                if used[i] {
                    continue;
                }
                let d = py.max(ty) - py.min(ty) + px.max(tx) - px.min(tx);
                if d < min_d {
                    min_d = d;
                    mi = i;
                }
            }
            used[mi] = true;
            res += min_d;
        }
        res
    };
    let mut initial_board_u8 = vec![vec![0; nx]; ny];
    let mut target_u8 = vec![vec![0; nx]; ny];
    for y in 0..ny {
        for x in 0..nx {
            initial_board_u8[y][x] = initial_board[y][x] as u8;
            target_u8[y][x] = target[y][x] as u8;
        }
    }
    // const MOD: u64 = 1e9 as u64 + 9;
    // const MOD: u64 = 1e18 as u64;
    const MOD: u128 = 1e35 as u128;
    let mut pows = vec![];
    let mut c = 1;
    for i in 0..ny * nx {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &[Vec<u8>]| -> u128 {
        let mut num = 0;
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                let p = pows[i] * board[y][x] as u128;
                num += p;
                num %= MOD;
            }
        }
        num
    };

    let start_hs = b_to_hash(&initial_board_u8);
    let goal_hs = b_to_hash(&target_u8);
    let first_state = BeamState {
        ans: vec![],
        board: initial_board_u8.to_vec(),
        score: calc_score(&initial_board_u8, &target_u8),
        pos_0,
        hs: start_hs,
    };
    let last_state = BeamState {
        ans: vec![],
        board: target_u8.to_vec(),
        score: calc_score(&target_u8, &initial_board_u8),
        pos_0: (last_ey, last_ex),
        hs: goal_hs,
    };

    // let mut used_hash = HashMap::<u64, Vec<(usize, usize)>>::new();
    // let mut used_hash_rev = HashMap::<u64, Vec<(usize, usize)>>::new();

    // 移動と、前のhash
    let mut used_hash = HashMap::<u128, ((usize, usize), u128)>::new();
    let mut used_hash_rev = HashMap::<u128, ((usize, usize), u128)>::new();

    let mut states = vec![first_state];
    let mut states_rev = vec![last_state];
    for loop_cnt in 0..1000 {
        // 前から
        if !tm.check_time(TL, false) {
            return vec![];
        }
        let mut new_states = vec![];
        while let Some(state) = states.pop() {
            // for state in states {
            if !tm.check_time(TL, true) {
                return vec![];
            }
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }
                let num = state.board[yy][xx];
                let curr_score_one = calc_score_one(&state.board, &target_u8, num);

                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);
                let hs = b_to_hash(&new_state.board);
                if used_hash.contains_key(&hs) {
                    continue;
                }
                new_state.hs = hs;

                // new_state.score = calc_score(&new_state.board, target);
                let new_score_one = calc_score_one(&new_state.board, &target_u8, num);
                if new_score_one > curr_score_one {
                    new_state.score += new_score_one - curr_score_one;
                } else {
                    new_state.score -= curr_score_one - new_score_one;
                }

                // new_state.ans.push((dy, dx));
                // used_hash.insert(hs, new_state.ans.clone());
                used_hash.insert(hs, ((dy, dx), state.hs));

                if used_hash_rev.contains_key(&hs) {
                    // eprintln!("aaa {}", loop_cnt);
                    // eprintln!("start: {}", start_hs);
                    let mut ans = vec![];
                    let mut curr = hs;
                    while curr != start_hs {
                        let ((dy, dx), p_hs) = used_hash[&curr];
                        ans.push((dy, dx));
                        curr = p_hs;
                        // println!("{}", curr);
                    }
                    ans.reverse();

                    let mut curr = hs;
                    while curr != goal_hs {
                        let ((dy, dx), p_hs) = used_hash_rev[&curr];
                        ans.push(rev_dyx((dy, dx)));
                        curr = p_hs;
                    }
                    return ans;
                    // let mut rev_ans = used_hash_rev[&hs].clone();
                    // rev_ans.reverse();
                    // let rev_ans = rev_ans.into_iter().map(|d| rev_dyx(d)).collect::<Vec<_>>();
                    // println!("{:?}", new_state.ans);
                    // println!("{:?}", rev_ans);

                    // return [new_state.ans, rev_ans].concat();
                }

                new_state.pos_0 = (yy, xx);
                // dbg!(new_state.score);

                // if new_state.score == 0 {
                //     // dbg!(new_state.board);
                //     return new_state.ans;
                // }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        while new_states.len() > width {
            new_states.pop();
        }
        // new_states.truncate(width);
        // states.clear();
        while let Some(s) = new_states.pop() {
            states.push(s);
        }
        // states = new_states;

        // 後ろから
        let mut new_states = vec![];
        // for state in states_rev {
        while let Some(state) = states_rev.pop() {
            if !tm.check_time(TL, true) {
                return vec![];
            }
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }

                let num = state.board[yy][xx];
                let curr_score_one = calc_score_one(&state.board, &initial_board_u8, num);

                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);

                let hs = b_to_hash(&new_state.board);
                if used_hash_rev.contains_key(&hs) {
                    continue;
                }
                new_state.hs = hs;

                // new_state.score = calc_score(&new_state.board, initial_board);
                let new_score_one = calc_score_one(&new_state.board, &initial_board_u8, num);
                if new_score_one > curr_score_one {
                    new_state.score += new_score_one - curr_score_one;
                } else {
                    new_state.score -= curr_score_one - new_score_one;
                }

                // new_state.ans.push((dy, dx));
                // used_hash_rev.insert(hs, new_state.ans.clone());
                used_hash_rev.insert(hs, ((dy, dx), state.hs));

                if used_hash.contains_key(&hs) {
                    // let ans = used_hash[&hs].clone();
                    // // ans.reverse();
                    // let mut rev_ans = new_state.ans.clone();
                    // rev_ans.reverse();
                    // let rev_ans = rev_ans.into_iter().map(|d| rev_dyx(d)).collect::<Vec<_>>();
                    // // println!("{:?} {:?}", ans, rev_ans);
                    // return [ans, rev_ans].concat();
                    // return vec![(0, 1)];
                    // eprintln!("{:#?}", used_hash);
                    // eprintln!("aaa {}", loop_cnt);
                    // eprintln!("start: {}", start_hs);
                    let mut ans = vec![];
                    let mut curr = hs;
                    while curr != start_hs {
                        let ((dy, dx), p_hs) = used_hash[&curr];
                        ans.push((dy, dx));
                        curr = p_hs;
                        // println!("{}", curr);
                    }
                    ans.reverse();

                    let mut curr = hs;
                    while curr != goal_hs {
                        let ((dy, dx), p_hs) = used_hash_rev[&curr];
                        ans.push(rev_dyx((dy, dx)));
                        curr = p_hs;
                    }
                    return ans;
                }
                new_state.pos_0 = (yy, xx);

                // if new_state.score == 0 {
                //     // dbg!(new_state.board);
                //     return new_state.ans;
                // }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        new_states.truncate(width);
        // states_rev = new_states;
        while let Some(s) = new_states.pop() {
            states_rev.push(s);
        }
    }
    vec![]
}

fn solve_puzzle_partial(
    ny: usize,
    nx: usize,
    sy: usize,
    sx: usize,
    initial_board: &[Vec<usize>],
    target: &[Vec<usize>],
    tm: &mut TimeManager,
) -> Vec<(usize, usize)> {
    eprintln!("{:?}", initial_board);
    eprintln!("{:?}", target);
    let nny = initial_board.len();
    let nnx = initial_board[0].len();

    let width = 5000;
    #[derive(Clone, Debug)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<usize>>,
        score: usize,
        pos_0: (usize, usize),
    }
    let mut pos_0 = (0, 0);
    for y in 0..nny {
        for x in 0..nnx {
            if initial_board[y][x] == 0 {
                pos_0 = (y, x);
                break;
            }
        }
    }
    let first_state = BeamState {
        ans: vec![],
        board: initial_board.to_vec(),
        score: 999,
        pos_0,
    };
    let swap = |y1: usize, x1: usize, y2: usize, x2: usize, board: &mut [Vec<usize>]| {
        let tmp = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = tmp;
    };
    let in_area = |y: usize, x: usize| -> bool { (0..nny).contains(&y) && (0..nnx).contains(&x) };
    let calc_score = |board: &[Vec<usize>]| -> usize {
        let mut used = vec![vec![false; nnx]; nny];
        let mut ans = 0;
        for y in sy..sy + ny {
            for x in sx..sx + nx {
                let val = target[y][x];
                let mut min_d = 10000;
                let (mut min_y, mut min_x) = (0, 0);
                for yy in 0..nny {
                    for xx in 0..nnx {
                        if board[yy][xx] == val && !used[yy][xx] {
                            let d = y.max(yy) - y.min(yy) + x.max(xx) - x.min(xx);
                            if d < min_d {
                                min_d = d;
                                min_y = yy;
                                min_x = xx;
                            }
                        }
                    }
                }
                ans += min_d;
                used[min_y][min_x] = true;
            }
        }
        ans
    };

    const MOD: u64 = 1e9 as u64 + 7;
    let mut pows = vec![];
    let mut c = 1;
    for i in 0..nny * nnx {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &[Vec<usize>]| -> u64 {
        let mut num = 0;
        for y in 0..nny {
            for x in 0..nnx {
                let i = y * nnx + x;
                let p = pows[i] * board[y][x] as u64;
                num += p;
                num %= MOD;
            }
        }
        num
    };
    let mut used_hash = HashSet::new();

    let mut states = vec![first_state];
    // dbg!(&states);
    for _ in 0..1000 {
        // println!("aaa");
        if !tm.check_time(TL, true) {
            dbg!("timeup!");
            return vec![];
        }
        let mut new_states = vec![];
        for state in &states {
            let (y, x) = state.pos_0;
            for &(dy, dx) in &DYXS {
                let (yy, xx) = (y + dy, x + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if !state.ans.is_empty() {
                    let (py, px) = *state.ans.last().unwrap();
                    if py + dy == 0 && px + dx == 0 {
                        continue;
                    }
                }
                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);

                let hs = b_to_hash(&new_state.board);
                if used_hash.contains(&hs) {
                    // continue;
                }
                used_hash.insert(hs);

                new_state.score = calc_score(&new_state.board);
                new_state.ans.push((dy, dx));
                new_state.pos_0 = (yy, xx);
                // dbg!(yy, xx);
                // dbg!(new_state.score);

                if new_state.score == 0 {
                    // dbg!(new_state.board);
                    return new_state.ans;
                }

                new_states.push(new_state);
            }
        }
        new_states.sort_by_key(|s| s.score);
        new_states.truncate(width);
        states = new_states;
    }
    vec![]
}

// fn solve_by_target_board_puzzle(state: &State, target: &Board) -> Vec<Dir> {
//     let board = state.orig_board.clone();
//     let n = board.n;

//     let mut first_puzzele_board = vec![];
//     for y in 0..n {
//         let mut row = vec![];
//         for x in 0..n {
//             let p = match board.cells[y][x] {
//                 Cell::Empty => 0,
//                 Cell::Piece(p) => *p.pattern,
//             };
//             row.push(p);
//         }
//         first_puzzele_board.push(row);
//     }
//     let mut target_puzzele_board = vec![];
//     for y in 0..n {
//         let mut row = vec![];
//         for x in 0..n {
//             let p = match target.cells[y][x] {
//                 Cell::Empty => 0,
//                 Cell::Piece(p) => *p.pattern,
//             };
//             row.push(p);
//         }
//         target_puzzele_board.push(row);
//     }

//     let mut ans = vec![];
//     let dirs = solve_puzzle(n, &first_puzzele_board, &target_puzzele_board);
//     println!("{:?}", dirs);
//     for d in dirs {
//         ans.push(Dir::from_dyx(d));
//     }
//     ans
// }

fn solve_puzzle_astar(
    ny: usize,
    nx: usize,
    initial_board: &[Vec<usize>],
    target: &[Vec<usize>],
    tm: &mut TimeManager,
) -> Vec<(usize, usize)> {
    #[derive(Clone)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<usize>>,
        score: usize,
        pos_0: (usize, usize),
    }
    let mut pos_0 = (0, 0);
    for y in 0..ny {
        for x in 0..nx {
            if initial_board[y][x] == 0 {
                pos_0 = (y, x);
                break;
            }
        }
    }
    let first_state = BeamState {
        ans: vec![],
        board: initial_board.to_vec(),
        score: 999,
        pos_0,
    };
    let swap = |y1: usize, x1: usize, y2: usize, x2: usize, board: &mut [Vec<usize>]| {
        let tmp = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = tmp;
    };
    let in_area = |y: usize, x: usize| -> bool { (0..ny).contains(&y) && (0..nx).contains(&x) };
    let calc_score = |board: &[Vec<usize>]| -> usize {
        let mut used = vec![vec![false; nx]; ny];
        let mut ans = 0;
        for y in 0..ny {
            for x in 0..nx {
                let val = target[y][x];
                let mut min_d = 10000;
                let (mut min_y, mut min_x) = (0, 0);
                for yy in 0..ny {
                    for xx in 0..nx {
                        if board[yy][xx] == val && !used[yy][xx] {
                            let d = y.max(yy) - y.min(yy) + x.max(xx) - x.min(xx);
                            if d < min_d {
                                min_d = d;
                                min_y = yy;
                                min_x = xx;
                            }
                        }
                    }
                }
                ans += min_d;
                used[min_y][min_x] = true;
            }
        }
        ans
    };

    const MOD: u64 = 1e18 as u64 + 7;
    let mut pows = vec![];
    let mut c = 1;
    for i in 0..ny * nx {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &[Vec<usize>]| -> u64 {
        let mut num = 0;
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                let p = pows[i] * board[y][x] as u64;
                num += p;
                num %= MOD;
            }
        }
        num
    };
    let mut used_hash = HashSet::new();
    // let mut states = vec![first_state];

    let mut cnt = 0;
    let mut states_set = BTreeMap::<(usize, usize), BeamState>::new();
    states_set.insert((999, cnt), first_state);
    cnt += 1;

    for i in 0..1000000 {
        // println!("{}", i);
        if !tm.check_time(TL, true) {
            return vec![];
        }
        let key = match states_set.iter().next() {
            None => break,
            Some(s) => s.0.clone(),
        };

        println!(
            "{:?} {:?}",
            states_set.iter().next().unwrap().0,
            states_set.iter().last().unwrap().0
        );

        let state = states_set.remove(&key).unwrap();
        println!("{:?} {}", key, state.ans.len());
        let (y, x) = state.pos_0;
        for &(dy, dx) in &DYXS {
            let (yy, xx) = (y + dy, x + dx);
            if !in_area(yy, xx) {
                continue;
            }
            if !state.ans.is_empty() {
                let (py, px) = *state.ans.last().unwrap();
                if py + dy == 0 && px + dx == 0 {
                    continue;
                }
            }
            let mut new_state = state.clone();
            swap(y, x, yy, xx, &mut new_state.board);

            let hs = b_to_hash(&new_state.board);
            if used_hash.contains(&hs) {
                continue;
            }
            used_hash.insert(hs);

            new_state.score = calc_score(&new_state.board);
            new_state.ans.push((dy, dx));
            new_state.pos_0 = (yy, xx);
            // dbg!(new_state.score);

            if new_state.score == 0 {
                // dbg!(new_state.board);
                return new_state.ans;
            }
            states_set.insert((new_state.score, cnt), new_state);
            if states_set.len() > 100 {
                let l = *states_set.iter().last().unwrap().0;
                states_set.remove(&l);
            }
            cnt += 1;
        }
    }
    vec![]
}

fn solve_by_target_board_2(
    state: &State,
    target: &Board,
    rem_y: usize,
    rem_x: usize,
    tm: &mut TimeManager,
) -> (Vec<char>, u64) {
    assert!(rem_y <= rem_x);
    let n = state.n;
    let mut board = state.orig_board.clone();
    let mut free_cells = vec![vec![true; n]; n];
    let (mut ey, mut ex) = state.orig_empty_pos;
    let dummy = HashSet::new();

    let mut ans = vec![];
    for i in 0..n - rem_y {
        // println!("x1");

        for x in i..n - 2 {
            let y = i;
            let t_pattern = target.cells[y][x].pattern();
            let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);
            free_cells[y][x] = false;
        }
        // eprintln!("y: {} -> {}", i, ans.len());

        // println!("x2");

        // last 2
        let y = i;
        let x = n - 2;
        let t_pattern = target.cells[i][n - 1].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
        // println!(" com: {:?}", com);
        // (ey, ex) = (c_ey, c_ex);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y][x] = false;

        // if (ey, ex) == (i, n - 1) {
        //     board.swap((ey, ex), (ey + 1, ex));
        //     ey += 1;
        //     ans.push('D');
        // }
        // 4 のましたにemptyがなければ持っていく
        if (ey, ex) != (i + 1, n - 2) {
            let route = get_routes((ey, ex), (i + 1, n - 2), &mut board, &free_cells, &dummy);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            // (ey, ex) = (i + 1, n - 2);
            ey = i + 1;
            ex = n - 2;
        }

        // let s = ans.iter().collect::<String>();
        // println!("{}", s);

        let y = i + 1;
        let x = n - 2;
        let t_pattern = target.cells[i][n - 2].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);

        if (ty, tx) != (i, n - 1) {
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);
            free_cells[y][x] = false;
            let mut route = get_routes((ey, ex), (i, n - 1), &mut board, &free_cells, &dummy);
            route.extend(vec![(i, n - 2), (i + 1, n - 2)]);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            free_cells[i + 1][n - 2] = true;
            // (ey, ex) = (i + 1, n - 2);
            ey = i + 1;
            ex = n - 2;
        } else {
            if ex == n - 1 {
                board.swap((ey, ex), (ey, ex - 1));
                ex -= 1;
                ans.push('L');
            }
            let (c_ey, c_ex) = move_empty_cell_by_commands(
                "URDLDRULURDLURDDLUURD".to_string(),
                &mut board,
                (ey, ex),
            );
            ey = c_ey;
            ex = c_ex;
            ans.extend("URDLDRULURDLURDDLUURD".chars());
            // URDLDRULURDLURDDLUURD
            // RULDRDLUURDLU (miss?)
        }
        free_cells[i][n - 1] = false;

        // break;
        // println!("y2: {} -> {}", i, ans.len());

        // println!("y1");

        if i == n - rem_x {
            // println!("{:?}", free_cells);
            break;
        }

        for y in i + 1..n - 2 {
            let x = i;
            let t_pattern = target.cells[y][x].pattern();
            let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);
            free_cells[y][x] = false;
        }

        // println!("x: {} -> {}", i, ans.len());

        // println!("y2");
        // println!("eee {:?}", (ey, ex));
        // last 2
        let y = n - 2;
        let x = i;
        let t_pattern = target.cells[n - 1][i].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
        // (ey, ex) = (c_ey, c_ex);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y][x] = false;

        // if (ey, ex) == (n - 1, i) {
        //     board.swap((ey, ex), (ey, ex + 1));
        //     ex += 1;
        //     ans.push('R');
        // }
        if (ey, ex) != (n - 2, i + 1) {
            let route = get_routes((ey, ex), (n - 2, i + 1), &mut board, &free_cells, &dummy);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            // (ey, ex) = (n - 2, i + 1);
            ey = n - 2;
            ex = i + 1;
        }

        let y = n - 2;
        let x = i + 1;
        let t_pattern = target.cells[n - 2][i].pattern();
        let (ty, tx) = get_nearest_pattern_cell(ey, ex, t_pattern, &board, &free_cells);

        if (ty, tx) != (n - 1, i) {
            let (com, (c_ey, c_ex)) =
                carry_cell((ty, tx), (y, x), (ey, ex), &mut board, &free_cells);
            // (ey, ex) = (c_ey, c_ex);
            ey = c_ey;
            ex = c_ex;
            ans.extend(com);

            free_cells[y][x] = false;
            let mut route = get_routes((ey, ex), (n - 1, i), &mut board, &free_cells, &dummy);
            route.extend(vec![(n - 2, i), (n - 2, i + 1)]);
            for ri in 0..route.len() - 1 {
                let p1 = route[ri];
                let p2 = route[ri + 1];
                board.swap(p1, p2);
                ans.push(get_dir_char_from_swap(p1, p2));
            }
            free_cells[n - 2][i + 1] = true;
            free_cells[n - 1][i] = false;
            // (ey, ex) = (n - 2, i + 1);
            ey = n - 2;
            ex = i + 1;
        } else {
            if ey == n - 1 {
                board.swap((ey, ex), (ey - 1, ex));
                ey -= 1;
                ans.push('U');
            }
            let (c_ey, c_ex) = move_empty_cell_by_commands(
                "LDRURDLULDRULDRRULLDR".to_string(),
                &mut board,
                (ey, ex),
            );
            ey = c_ey;
            ex = c_ex;
            ans.extend("LDRURDLULDRULDRRULLDR".chars());
            // free_cells[n - 2][i + 1] = true;
            free_cells[n - 1][i] = false;
        }

        // println!("x2: {} -> {}", i, ans.len());
    }

    // 4x4 が残る
    // 4x3 が残る

    let mut first_puzzele_board = vec![];
    for y in n - rem_y..n {
        let mut row = vec![];
        // for x in n - 3..n {
        for x in n - rem_x..n {
            let p = match board.cells[y][x] {
                Cell::Empty => 0,
                Cell::Piece(p) => *p.pattern,
            };
            row.push(p);
        }
        first_puzzele_board.push(row);
    }
    let mut target_puzzele_board = vec![];
    for y in n - rem_y..n {
        let mut row = vec![];
        // for x in n - 3..n {
        for x in n - rem_x..n {
            let p = match target.cells[y][x] {
                Cell::Empty => 0,
                Cell::Piece(p) => *p.pattern,
            };
            row.push(p);
        }
        target_puzzele_board.push(row);
    }
    // println!("aaaa");
    // let dirs = solve_puzzle(
    // let dirs = solve_puzzle2(
    let dirs = solve_puzzle3(
        rem_y,
        rem_x,
        &first_puzzele_board,
        &target_puzzele_board,
        tm,
    );
    // println!("{:?}", dirs);
    if dirs.is_empty() {
        return (vec![], 0);
    }

    for d in dirs {
        ans.push(Dir::from_dyx(d).to_c());
    }
    (ans, get_group_size(&board, 0, 0) as u64)
}

fn solve_by_partial_puzzle(
    state: &State,
    target: &Board,
    tm: &mut TimeManager,
) -> (Vec<char>, bool) {
    let n = target.n;
    let mut board = state.orig_board.clone();
    let (mut ey, mut ex) = state.orig_empty_pos;

    let mut ans = vec![];
    for i in 0..n - 4 {
        let y = i;
        // let xlen = n - i;
        let xlen = n;

        let mut c_target = vec![vec![0; xlen]];
        for x in 0..xlen {
            // c_target[0][x] = target.get_pattern_num(y, x + i);
            c_target[0][x] = target.get_pattern_num(y, x);
        }
        let mut inib = vec![vec![0; xlen]; n - i];
        for yy in 0..n - i {
            for xx in 0..xlen {
                // inib[yy][xx] = board.get_pattern_num(yy + i, xx + i);
                inib[yy][xx] = board.get_pattern_num(yy + i, xx);
            }
        }
        let dirs = solve_puzzle_partial(1, xlen, 0, 0, &inib, &c_target, tm);
        eprintln!("d {:?}", dirs.len());
        for (dy, dx) in dirs {
            let (yy, xx) = (ey + dy, ex + dx);
            board.swap((yy, xx), (ey, ex));
            ey = yy;
            ex = xx;
            ans.push(Dir::from_dyx((dy, dx)).to_c());
        }

        // let x = i;
        // let ylen = n - i - 1;
        // let mut c_target = vec![vec![0]; ylen];
        // for y in 0..n - i - 1 {
        //     c_target[y][0] = target.get_pattern_num(y + i + 1, x);
        // }
        // let mut inib = vec![vec![0; n - i]; n - i - 1];
        // for yy in 0..n - i - 1 {
        //     for xx in 0..n - i {
        //         inib[yy][xx] = board.get_pattern_num(yy + i + 1, xx + i);
        //     }
        // }
        // let dirs = solve_puzzle_partial(n - i - 1, 1, 0, 0, &inib, &c_target, tm);
        // println!("d {:?}", dirs);

        // for (dy, dx) in dirs {
        //     let (yy, xx) = (ey + dy, ex + dx);
        //     board.swap((yy, xx), (ey, ex));
        //     ey = yy;
        //     ex = xx;
        //     ans.push(Dir::from_dyx((dy, dx)).to_c());
        // }
    }
    (ans, true)
}

/* ------------------------------------------------------------------------
    solve!
------------------------------------------------------------------------ */
fn solve() {
    // let mut t = vec![vec![9, 2, 5], vec![1, 4, 6], vec![1, 2, 8]];
    // let mut v = vec![vec![6, 1, 9], vec![2, 8, 1], vec![5, 4, 2], vec![9, 3, 0]];
    // let ans = solve_puzzle(3, &v, &t);
    // dbg!(ans.len());

    /* ------------ ready ------------ */
    let mut tm = mytool::TimeManager::new();
    let mut rng = mytool::MyRng::new();
    // get_matched_piece_patterns();
    let mut state = State::new();
    // println!("{:?}", state.piece_cnts);
    // solve_by_dfs(&state);
    // solve_by_dfs2(&state);
    // solve_by_beam(&state, &mut rng);
    // let t = tm.get_time();

    // let target_board = get_target_board_by_rand5(&state, &mut rng, &mut tm);
    // let target_board = get_target_board_by_rand4(&state, &mut rng, &mut tm);
    // let t2 = tm.get_time();
    // dbg!(t2);
    // target_board.print_pattern();
    // let ans = solve_by_target_board_puzzle(&state, &target_board);
    // let ans = ans.iter().map(|d| d.to_c()).collect::<Vec<_>>();
    // let s = ans.iter().collect::<String>();
    // println!("{}", s);

    // return;

    // let target_board = get_target_board_by_rand4(&state, &mut rng, &mut tm);
    // target_board.print_pattern();

    // let n = state.n;
    // let mut tar = vec![vec![0; n]];
    // for x in 0..n {
    //     tar[0][x] = match target_board.cells[0][x] {
    //         Cell::Empty => 0,
    //         Cell::Piece(p) => *p.pattern,
    //     };
    // }

    // let mut initb = vec![vec![0; n]; n];
    // for y in 0..n {
    //     for x in 0..n {
    //         initb[y][x] = match state.orig_board.cells[y][x] {
    //             Cell::Empty => 0,
    //             Cell::Piece(p) => *p.pattern,
    //         };
    //     }
    // }
    // let t = tm.get_time();
    // let ans = solve_puzzle_partial(1, 6, 0, 0, &initb, &tar, &mut tm);
    // let t2 = tm.get_time();
    // dbg!(t2 - t);
    // let mut ddd = vec![];
    // for d in ans {
    //     ddd.push(Dir::from_dyx(d).to_c());
    // }
    // let s = ddd.iter().collect::<String>();
    // println!("{:?}", s);

    // let target_board = get_target_board_by_rand2(&state, &mut rng);
    // let target_board = get_target_board_by_rand(&state, &mut rng);
    let n = state.n;

    // ほとんどtreeが見つからない
    // let target_board = get_target_board_by_rand4(&state, &mut rng, &mut tm);

    let target_board = get_target_board_by_rand3(&state, &mut rng, &mut tm);
    // target_board.print_pattern();

    // let (ans, _) = solve_by_partial_puzzle(&state, &target_board, &mut tm);
    // let s = ans.iter().collect::<String>();
    // println!("{}", s);
    // dbg!(TL);

    let t = tm.get_time();
    let (ans1, score) = solve_by_target_board(&state, &target_board);
    // let (ans2, _) = solve_by_target_board_2(&state, &target_board, 3, 4, &mut tm);
    // let (ans2, _) = solve_by_target_board_2(&state, &target_board, 4, 4, &mut tm);
    // let (ans3, _) = solve_by_target_board_2(&state, &target_board, 4, 4, &mut tm);
    // let (ans3, _) = solve_by_target_board_2(&state, &target_board, 4, 5, &mut tm);
    let (ans2, _) = solve_by_target_board_2(&state, &target_board, 5, 5, &mut tm);

    // width 下げて、これあげたら、めっちゃいけそう？
    // let (ans3, _) = solve_by_target_board_2(&state, &target_board, 5, 6, &mut tm);
    let (ans3, _) = solve_by_target_board_2(&state, &target_board, 6, 6, &mut tm);
    let t2 = tm.get_time();
    // let ans3 = ans2.clone();
    dbg!(t2);
    dbg!(ans1.len());
    dbg!(ans2.len());
    dbg!(ans3.len());

    let t2 = tm.get_time();
    dbg!(t2);

    let g = get_group_size(&target_board, 0, 0);
    if g != n * n - 1 {
        dbg!("no tree....");
    }

    if ans2.is_empty() && ans3.is_empty() {
        let s = ans1.iter().collect::<String>();
        println!("{}", s);
        if g != n * n - 1 {
            eprintln!("Score = {}", 500000 * (n * n - 2) / (n * n - 1));
        } else {
            let coef = 2.0 - s.len() as f64 / state.t as f64;
            eprintln!("Score = {}", (500000.0 * coef) as u64);
        }
        return;
    }

    if ans3.is_empty() {
        let s = ans2.iter().collect::<String>();
        println!("{}", s);
        if g != n * n - 1 {
            eprintln!("Score = {}", 500000 * (n * n - 2) / (n * n - 1));
        } else {
            let coef = 2.0 - s.len() as f64 / state.t as f64;
            eprintln!("Score = {}", (500000.0 * coef) as u64);
        }
        return;
    }

    let s = if ans3.len() < ans2.len() {
        ans3.iter().collect::<String>()
    } else {
        ans2.iter().collect::<String>()
    };
    let t2 = tm.get_time();
    dbg!(t2);
    println!("{}", s);

    if g != n * n - 1 {
        eprintln!("Score = {}", 500000 * (n * n - 2) / (n * n - 1));
    } else {
        let coef = 2.0 - s.len() as f64 / state.t as f64;
        eprintln!("Score = {}", (500000.0 * coef) as u64);
    }

    // let target_board = get_target_board_by_rand3(&state, &mut rng);
    // // let target_board = get_target_board_by_rand2(&state, &mut rng);
    // // let target_board = get_target_board_by_rand(&state, &mut rng);
    // target_board.print_pattern();
    // let t = tm.get_time();
    // let (ans, score) = solve_by_target_board(&state, &target_board);
    // let s = ans.iter().collect::<String>();
    // let t2 = tm.get_time();
    // // println!("{}", t2 - t);
    // println!("{}", t2);
    // println!("{}", s);

    // let mut b = &mut state.orig_board.clone();
    // let n = state.n;
    // let mut free_cells = vec![vec![true; n]; n];

    // let cs = carry_cell((3, 5), (0, 1), (1, 1), &mut b, &free_cells);
    // b.print_pattern();
    // let s = cs.iter().collect::<String>();
    // println!("{}", s);

    // const TL: u64 = 2970;
    // while tm.check_time(TL, true) {}

    // ---- output ---

    // eprintln!("Score = {}", state.score);
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
