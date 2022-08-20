#![allow(unused_imports)]
use core::panic;
use itertools::Itertools;
use mytool::{MyRng, TimeManager};
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{borrow, vec};

const DYXS: [(usize, usize); 4] = [(0, !0), (!0, 0), (0, 1), (1, 0)];
// const DYXS: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

#[cfg(not(feature = "mylocal"))]
const TL: u64 = 2920;
// const TL: u64 = 2700;

#[cfg(feature = "mylocal")]
const TL: u64 = 1700;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// 左/上/右/下 の順に 0/1/2/3
struct Dir(usize);
impl std::ops::Deref for Dir {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.0
    }
}
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    fn pattern2(&self) -> Pattern {
        match self {
            Self::Piece(p) => p.pattern,
            Self::Empty => Pattern(0),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    n: usize,
    cells: Vec<Vec<Cell>>,
}

#[allow(dead_code)]
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
        // if y == self.n - 1 && x == self.n - 1 {
        //     return false;
        // }
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
    t: usize,          // 2 * n**3
    orig_board: Board, // 入力そのまま
    #[allow(dead_code)]
    piece_cnts: [usize; 16], // 各パターンの
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
        Self {
            n,
            t,
            orig_board,
            piece_cnts,
            orig_empty_pos,
        }
    }
}

// connected_size ?
fn get_connected_size(board: &Board, sy: usize, sx: usize) -> usize {
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

#[allow(dead_code)]
fn get_connected_size_with_empty(
    board: &Board,
    sy: usize,
    sx: usize,
    empty: (usize, usize),
) -> usize {
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

fn get_connected_size2(board: &Board, sy: usize, sx: usize) -> usize {
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
            if yy >= board.n || xx >= board.n {
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

#[allow(dead_code)]
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
                    if board.cells[yy][xx] == Cell::Empty {
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

fn shuffle_board(board: &mut Board, rng: &mut MyRng, cnt: usize) {
    for _ in 0..cnt {
        let nn = board.n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        // assert!(board.cells[empty_pos.0][empty_pos.1] == Cell::Empty);
        // if (y1, x1) == empty_pos || (y2, x2) == empty_pos {
        //     continue;
        // }
        // if (y1, x1) == (y2, x2) {
        //     continue;
        // }
        // if board.cells[y1][x1].pattern() == board.cells[y2][x2].pattern() {
        //     continue;
        // }
        board.swap((y1, x1), (y2, x2));
    }
}

#[allow(dead_code)]
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
    let empty_pos = (n - 1, n - 1);
    // let ess = vec![
    //     (n - 1, n - 1),
    //     (n - 2, n - 1),
    //     (n - 2, n - 2),
    //     (n - 1, n - 2),
    // ];

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
            // let score_g = get_connected_size(&board, 0, 0);
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
            // let new_score_g = get_connected_size(&board, 0, 0);
            let new_score_g = get_connected_size_with_empty(&board, 0, 0, empty_pos);

            // let new_score = calc_board_score(&board) as u64 + new_score_g;
            // let new_score = new_score_g as u64 + (n * n - new_score_gl) as u64 * 1;
            let new_score = new_score_g;
            if new_score < score {
                board.swap((y1, x1), (y2, x2));
                cnt = if cnt == 1 { 0 } else { 1 };
            } else if new_score > score {
                score = new_score;
            }

            // let score2 = get_connected_size(&board, 0, 0) as u64;

            // dbg!(score);
            // dbg!(get_group_len(&board));
            // if score as usize == n * n - 1 {
            // if score as usize == (n * n - 2) * 2 {
            // if score as usize == (n * n - 2) * 2 + n * n - 1 {
            // TOOD: ここのシャッフル閾値ちゃんとやる
            if score == n * n - 1 || tt > RETRY_CNT {
                let (_, last_score) = solve_by_target_board(&state, &board);
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

                    // score = get_connected_size(&board, 0, 0);
                    // empty_pos = n_empty_pos;
                    score = get_connected_size_with_empty(&board, 0, 0, empty_pos);
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
            // let score_g = get_connected_size(&board, 0, 0);
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
            let new_score_g = get_connected_size(&board, 0, 0);
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

            // let score2 = get_connected_size(&board, 0, 0) as u64;

            // dbg!(get_group_len(&board));
            // if score as usize == n * n - 1 {
            // if score as usize == (n * n - 2) * 2 {
            // if score as usize == (n * n - 2) * 2 + n * n - 1 {
            if score == n * n - 1 || tt > RETRY_CNT {
                // tt = 0;
                if tt > RETRY_CNT {
                    tt = 0;
                }
                let (_, last_score) = solve_by_target_board(&state, &board);

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
                    score = get_connected_size(&board, 0, 0);
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

fn get_target_board_by_rand_empty4(state: &State, rng: &mut MyRng, tm: &mut TimeManager) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    // let first_board = board.clone();

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
    let mut emps_ok = vec![vec![false; n]; n];
    emps_ok[0][0] = true;
    emps_ok[0][n - 1] = true;
    emps_ok[n - 1][0] = true;
    emps_ok[n - 1][n - 1] = true;

    let mut tmp_ans = board.clone();
    let mut tt = 0;
    let mut score = 0;
    // const RETRY_CNT: usize = 150000; //  6 が失敗しやすい？
    const RETRY_CNT: usize = 150000; //  6 が失敗しやすい？
                                     // const RETRY_CNT: usize = 200000;

    let retry_cnt = if n == 6 {
        RETRY_CNT / 5
    } else if n == 10 {
        // RETRY_CNT * 3
        // RETRY_CNT * 2
        RETRY_CNT * 5 / 2
    } else if n == 9 {
        RETRY_CNT * 2 // ちょうどいい？
    } else {
        RETRY_CNT
    };
    // let retry_cnt = RETRY_CNT;

    let e = get_empty_pos(&board);
    board.swap(e, (n - 1, n - 1));

    let mut tmp_ans_score = 0;
    while tm.check_time(TL, false) {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if !board.in_area(y2, x2) {
            continue;
        }

        if (y1, x1) == (y2, x2) {
            continue;
        }
        // if score == n * n - 2 {
        // if (score >= n * n - 4 && n == 6) || score >= n * n - 3 {
        //     tt += 1;
        // }
        // if score >= n * n - 2 && n == 10 {
        //     tt += 1;
        // }
        tt += 1;

        if board.get_pattern_num(y1, x1) == 0 && y2 != 0 && y2 != n - 1 && x2 != 0 && x2 != n - 1 {
            continue;
        }
        if board.get_pattern_num(y2, x2) == 0 && y1 != 0 && y1 != n - 1 && x1 != 0 && x1 != n - 1 {
            continue;
        }
        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y2][x2].pattern2())
            || !check_wall_ok(y2, x2, board.cells[y1][x1].pattern2())
        {
            continue;
        }

        // if board.get_pattern_num(y1, x1) == 0 && !emps_ok[y2][x2] {
        //     continue;
        // }
        // if board.get_pattern_num(y2, x2) == 0 && !emps_ok[y1][x1] {
        //     continue;
        // }

        board.swap((y1, x1), (y2, x2));

        // let new_score_0 = n * n - 1 - get_group_len(&board); // 0 ~ n*n-2
        // let new_score_g = get_connected_size_with_empty(&board, 0, 0, empty_pos);
        let new_score_g = get_connected_size2(&board, 0, 0);
        let new_score = new_score_g;

        if new_score == n * n - 2 && score == n * n - 3 {
            // eprintln!("{}", tt);
        }

        if new_score < score {
            board.swap((y1, x1), (y2, x2));
        } else if new_score > score {
            score = new_score;
            // tmp_ans = board.clone();
        }
        // println!("{} {}", tt, score);

        // TOOD: ここのシャッフル閾値ちゃんとやる
        // if score ==  || tt > RETRY_CNT {
        if new_score_g == n * n - 1 || tt > retry_cnt {
            // let (_, last_score) = solve_by_target_board(&state, &board);
            let (_ans, last_score) = solve_by_target_board_free_empty(&state, &board, 2, 2);
            // println!("{}", last_score);
            if last_score > tmp_ans_score {
                tmp_ans = board.clone();
                tmp_ans_score = last_score;
            }
            // println!("{:?}", ans.iter().collect::<String>());
            // println!("{} ", last_score);

            if tt > retry_cnt {
                tt = 0;
                // eprintln!("reest");
            }

            if last_score as usize == n * n - 1 {
                return board.clone();
                // break;
            } else {
                // board = first_board.clone();
                shuffle_board(&mut board, rng, 1000);
                // let e = get_empty_pos(&board);
                // board.swap(e, (n - 1, n - 1));
                score = get_connected_size2(&board, 0, 0);
            }
        }
        // if rng.get_percent() < 0.00001 {
        //     shuffle_board(&mut board, rng, 1);
        // }
    }
    tmp_ans
    // dbg!(tt);
    // if score == n * n - 1 {
    //     board
    // } else {
    //     tmp_ans
    // }
}

fn calc_board_score(board: &Board) -> usize {
    let n = board.n;
    // let mut ng_edge_cnt = 0;
    let mut ok_edge_cnt = 0;
    for y in 0..n {
        for x in 0..n {
            // if y == n - 1 && x == n - 1 {
            //     break;
            // }
            let p = board.get_pattern_num(y, x);
            let dirs = get_dirs_from_pattern(Pattern(p));
            for dir in dirs {
                let (dy, dx) = dir.to_dyx();
                let (yy, xx) = (y + dy, x + dx);
                if !board.in_area(yy, xx) {
                    // ng_edge_cnt += 1;
                    continue;
                }
                let p2 = board.get_pattern_num(yy, xx);
                if Pattern::is_matched(Pattern::from_single_dir(dir), Pattern(p2)) {
                    ok_edge_cnt += 1;
                } else {
                    // ng_edge_cnt += 1;
                }
            }
        }
    }
    // assert!(ok_edge_cnt % 2 == 0);
    ok_edge_cnt / 2
}

// 交換後
fn calc_board_score_diff(board: &mut Board, p1: (usize, usize), p2: (usize, usize)) -> i32 {
    let mut res = 0;
    for (y, x) in vec![p1, p2] {
        let p = board.get_pattern_num(y, x);
        let dirs = get_dirs_from_pattern(Pattern(p));
        for dir in dirs {
            let (dy, dx) = dir.to_dyx();
            let (yy, xx) = (y + dy, x + dx);
            if !board.in_area(yy, xx) {
                // ng_edge_cnt += 1;
                continue;
            }
            let pat2 = board.get_pattern_num(yy, xx);
            if Pattern::is_matched(Pattern::from_single_dir(dir), Pattern(pat2)) {
                // ok_edge_cnt += 1;
                // 隣り合うセルの swap じに、重複カウントを避ける
                if (y, x) == p1 && (yy, xx) == p2 {
                    continue;
                }
                res += 1;
            } else {
                // ng_edge_cnt += 1;
            }
        }
    }
    board.swap(p1, p2);
    for (y, x) in vec![p1, p2] {
        let p = board.get_pattern_num(y, x);
        let dirs = get_dirs_from_pattern(Pattern(p));
        for dir in dirs {
            let (dy, dx) = dir.to_dyx();
            let (yy, xx) = (y + dy, x + dx);
            if !board.in_area(yy, xx) {
                // ng_edge_cnt += 1;
                continue;
            }
            let pat2 = board.get_pattern_num(yy, xx);
            if Pattern::is_matched(Pattern::from_single_dir(dir), Pattern(pat2)) {
                // ok_edge_cnt += 1;
                // 隣り合うセルの swap じに、重複カウントを避ける
                if (y, x) == p1 && (yy, xx) == p2 {
                    continue;
                }
                res -= 1;
            } else {
                // ng_edge_cnt += 1;
            }
        }
    }
    board.swap(p1, p2);
    res
}

fn get_target_board_by_rand_empty_two_score(
    state: &State,
    rng: &mut MyRng,
    tm: &mut TimeManager,
) -> Board {
    let mut board = state.orig_board.clone();
    let n = state.n;
    // let first_board = board.clone();

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
    // let mut emps_ok = vec![vec![false; n]; n];
    // emps_ok[0][0] = true;
    // emps_ok[0][n - 1] = true;
    // emps_ok[n - 1][0] = true;
    // emps_ok[n - 1][n - 1] = true;

    let mut tmp_ans = board.clone();
    let mut tt = 0;
    // const RETRY_CNT: usize = 150000; //  6 が失敗しやすい？
    const RETRY_CNT: usize = 150000; //  6 が失敗しやすい？
                                     // const RETRY_CNT: usize = 200000;

    let retry_cnt = if n == 6 {
        RETRY_CNT / 5
    } else if n == 10 {
        RETRY_CNT * 2
    } else if n == 9 {
        RETRY_CNT * 3 / 2
    } else {
        RETRY_CNT
    };
    // let retry_cnt = RETRY_CNT;

    let e = get_empty_pos(&board);
    board.swap(e, (n - 1, n - 1));

    let mut score_pre = calc_board_score(&board);
    let mut score = 0;
    let mut tmp_ans_score = 0;
    let mut first = true;
    while tm.check_time(TL, false) {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if !board.in_area(y2, x2) {
            continue;
        }

        if (y1, x1) == (y2, x2) {
            continue;
        }

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board.cells[y2][x2].pattern2())
            || !check_wall_ok(y2, x2, board.cells[y1][x1].pattern2())
        {
            continue;
        }
        // if score == n * n - 2 {
        // if (score >= n * n - 4 && n == 6) || score >= n * n - 3 {
        //     tt += 1;
        // }
        // if score >= n * n - 2 && n == 10 {
        //     tt += 1;
        // }

        if board.get_pattern_num(y1, x1) == 0 && y2 != 0 && y2 != n - 1 && x2 != 0 && x2 != n - 1 {
            continue;
        }
        if board.get_pattern_num(y2, x2) == 0 && y1 != 0 && y1 != n - 1 && x1 != 0 && x1 != n - 1 {
            continue;
        }
        // if board.get_pattern_num(y1, x1) == 0 && !emps_ok[y2][x2] {
        //     continue;
        // }
        // if board.get_pattern_num(y2, x2) == 0 && !emps_ok[y1][x1] {
        //     continue;
        // }
        board.swap((y1, x1), (y2, x2));

        // 軽い方のスコア計算
        // これが満点になるまではこれをやる
        // これ引っかかるのはなぜ...
        // score の diff 計算がバグってるっぽいが
        // いい感じのタイミングでこれを超えるのでもう放置でいいか...
        // 直った
        // assert!(score_pre <= n * n - 2);
        if score_pre >= n * n - 4 && first {
            let (_ans, last_score) = solve_by_target_board_free_empty(&state, &board, 2, 2);
            if last_score > tmp_ans_score {
                tmp_ans = board.clone();
                tmp_ans_score = last_score;
            }
            first = false;
        }
        if score_pre < n * n - 2 {
            let score_diff = calc_board_score_diff(&mut board, (y1, x1), (y2, x2));
            if score_diff >= 0 {
                score_pre += score_diff as usize;
                // let score_gt = calc_board_score(&board);
                // if score_gt != score_pre {
                //     println!("gt: {}, pre: {}", score_gt, score_pre);
                //     board.print_pattern();
                // }
            } else {
                board.swap((y1, x1), (y2, x2));
            }
            // println!("scorepre: {}", score_pre);
            // board.print_pattern();
        } else {
            tt += 1;
            // board.print_pattern();
            // eprintln!();

            let new_score_g = get_connected_size2(&board, 0, 0);
            let new_score = new_score_g;

            if new_score < score {
                board.swap((y1, x1), (y2, x2));
            } else if new_score > score {
                score = new_score;
            }

            if new_score_g == n * n - 1 || tt > retry_cnt {
                let (_ans, last_score) = solve_by_target_board_free_empty(&state, &board, 2, 2);
                if last_score > tmp_ans_score {
                    tmp_ans = board.clone();
                    tmp_ans_score = last_score;
                }

                if tt > retry_cnt {
                    tt = 0;
                }

                if last_score as usize == n * n - 1 {
                    return board.clone();
                } else {
                    // eprintln!("reset");
                    shuffle_board(&mut board, rng, 1000);
                    score = get_connected_size2(&board, 0, 0);
                    score_pre = calc_board_score(&board);
                }
            }
        }
    }
    tmp_ans
}

fn get_target_board_by_rand_empty_two_score_2(
    state: &State,
    rng: &mut MyRng,
    tm: &mut TimeManager,
) -> Board {
    let board = state.orig_board.clone();
    let n = state.n;
    const RETRY_CNT: usize = 150000;
    let retry_cnt = if n == 6 {
        // RETRY_CNT / 5
        RETRY_CNT
    } else if n == 10 {
        RETRY_CNT * 2
    } else if n == 9 {
        RETRY_CNT * 3 / 2
    } else {
        RETRY_CNT
    };

    let check_wall_ok = |y: usize, x: usize, p: u8| -> bool {
        if y == 0 && (1 << 1) & p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & p > 0 {
            return false;
        }
        true
    };

    let mut board_u8 = vec![vec![!0; n]; n];
    for y in 0..n {
        for x in 0..n {
            board_u8[y][x] = board.get_pattern_num(y, x) as u8;
        }
    }
    let swap_board = |board_u8: &mut [Vec<u8>], p1: (usize, usize), p2: (usize, usize)| {
        let (y1, x1) = p1;
        let (y2, x2) = p2;
        let tmp = board_u8[y2][x2];
        board_u8[y2][x2] = board_u8[y1][x1];
        board_u8[y1][x1] = tmp;
    };

    let mut pattern_dirs = vec![vec![]; 16];
    for i in 0..16 {
        for di in 0..4 {
            if (1 << di) & i > 0 {
                pattern_dirs[i].push(di);
            }
        }
    }
    let mut pattern_from_ok_dir = [[false; 4]; 16];
    for i in 0..16 {
        for di in 0..4 {
            if (1 << di) & i > 0 {
                pattern_from_ok_dir[i][(di + 2) % 4] = true;
            }
        }
    }
    let in_area = |y: usize, x: usize| (0..n).contains(&y) && (0..n).contains(&x);
    let calc_board_score_u8 = |board_u8: &[Vec<u8>]| -> usize {
        let mut ok_edge_cnt = 0;
        for y in 0..n {
            for x in 0..n {
                let p = board_u8[y][x];
                let dirs = &pattern_dirs[p as usize];
                for &dir in dirs {
                    let (dy, dx) = DYXS[dir];
                    let (yy, xx) = (y + dy, x + dx);
                    if !in_area(yy, xx) {
                        continue;
                    }
                    let p2 = board_u8[yy][xx];
                    if pattern_from_ok_dir[p2 as usize][dir] {
                        ok_edge_cnt += 1;
                    }
                }
            }
        }
        // assert!(ok_edge_cnt % 2 == 0);
        ok_edge_cnt / 2
    };
    let calc_board_score_diff_u8 =
        |board_u8: &mut [Vec<u8>], p1: (usize, usize), p2: (usize, usize)| -> i32 {
            let mut res = 0;
            for (y, x) in vec![p1, p2] {
                let p = board_u8[y][x];
                let dirs = &pattern_dirs[p as usize];
                for &dir in dirs {
                    let (dy, dx) = DYXS[dir];
                    let (yy, xx) = (y + dy, x + dx);
                    if !in_area(yy, xx) {
                        continue;
                    }
                    let pat2 = board_u8[yy][xx];
                    if pattern_from_ok_dir[pat2 as usize][dir] {
                        // 隣り合うセルの swap じに、重複カウントを避ける
                        if (y, x) == p1 && (yy, xx) == p2 {
                            continue;
                        }
                        res += 1;
                    }
                }
            }
            swap_board(board_u8, p1, p2);
            for (y, x) in vec![p1, p2] {
                let p = board_u8[y][x];
                let dirs = &pattern_dirs[p as usize];
                for &dir in dirs {
                    let (dy, dx) = DYXS[dir];
                    let (yy, xx) = (y + dy, x + dx);
                    if !in_area(yy, xx) {
                        continue;
                    }
                    let pat2 = board_u8[yy][xx];
                    if pattern_from_ok_dir[pat2 as usize][dir] {
                        // 隣り合うセルの swap じに、重複カウントを避ける
                        if (y, x) == p1 && (yy, xx) == p2 {
                            continue;
                        }
                        res -= 1;
                    }
                }
            }
            swap_board(board_u8, p1, p2);
            res
        };

    let get_connected_size_u8 = |board_u8: &[Vec<u8>], sy: usize, sx: usize| -> usize {
        let mut visited = vec![vec![false; n]; n];
        let mut q = VecDeque::new();
        q.push_back((sy, sx));
        visited[sy][sx] = true;
        while let Some((cy, cx)) = q.pop_front() {
            let pat = board_u8[cy][cx];
            let dirs = &pattern_dirs[pat as usize];
            for &di in dirs {
                let (dy, dx) = DYXS[di];
                let (yy, xx) = (cy + dy, cx + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if visited[yy][xx] {
                    continue;
                }
                let pat2 = board_u8[yy][xx];
                if pattern_from_ok_dir[pat2 as usize][di] {
                    visited[yy][xx] = true;
                    q.push_back((yy, xx));
                }
            }
        }
        let mut ans = 0;
        for y in 0..n {
            for x in 0..n {
                if visited[y][x] {
                    ans += 1;
                }
            }
        }
        ans
    };
    let to_board = |board_u8: &[Vec<u8>]| -> Board {
        let mut new_b = Board::new(n);
        for y in 0..n {
            for x in 0..n {
                let p = board_u8[y][x] as usize;
                if p == 0 {
                    new_b.set_cell(y, x, Cell::Empty);
                } else {
                    new_b.set_cell(
                        y,
                        x,
                        Cell::piece_from_pattern(Pattern(board_u8[y][x] as usize)),
                    );
                }
            }
        }
        new_b
    };
    let shuffle_board_u8 = |board_u8: &mut [Vec<u8>], rng: &mut MyRng, cnt: usize| {
        for _ in 0..cnt {
            let nn = board.n as i32;
            let y1 = rng.get_int(0, nn - 1) as usize;
            let x1 = rng.get_int(0, nn - 1) as usize;
            let y2 = rng.get_int(0, nn - 1) as usize;
            let x2 = rng.get_int(0, nn - 1) as usize;
            swap_board(board_u8, (y1, x1), (y2, x2));
        }
    };

    let mut tmp_ans = board_u8.clone();
    let mut tt = 0;

    let e = get_empty_pos(&board);
    swap_board(&mut board_u8, e, (n - 1, n - 1));

    let mut score_pre = calc_board_score_u8(&board_u8);
    let mut score = 0;
    let mut tmp_ans_score = 0;
    let mut first = true;
    while tm.check_time(TL, false) {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if !in_area(y2, x2) {
            continue;
        }

        if (y1, x1) == (y2, x2) {
            continue;
        }

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board_u8[y2][x2]) || !check_wall_ok(y2, x2, board_u8[y1][x1]) {
            continue;
        }

        // if board_u8[y1][x1] == 0 && y2 != 0 && y2 != n - 1 && x2 != 0 && x2 != n - 1 {
        //     continue;
        // }
        // if board_u8[y2][x2] == 0 && y1 != 0 && y1 != n - 1 && x1 != 0 && x1 != n - 1 {
        //     continue;
        // }

        swap_board(&mut board_u8, (y1, x1), (y2, x2));

        // assert!(score_pre <= n * n - 2);
        if score_pre >= n * n - 4 && first {
            let bd = to_board(&board_u8);
            let (_ans, last_score) = solve_by_target_board_free_empty(&state, &bd, 2, 2);
            if last_score > tmp_ans_score {
                tmp_ans = board_u8.clone();
                tmp_ans_score = last_score;
            }
            first = false;
        }
        if score_pre < n * n - 2 {
            let score_diff = calc_board_score_diff_u8(&mut board_u8, (y1, x1), (y2, x2));
            if score_diff >= 0 {
                score_pre += score_diff as usize;
            } else {
                swap_board(&mut board_u8, (y1, x1), (y2, x2));
            }
        } else {
            tt += 1;
            let new_score_g = get_connected_size_u8(&board_u8, n / 2, n / 2);
            let new_score = new_score_g;

            if new_score < score {
                swap_board(&mut board_u8, (y1, x1), (y2, x2));
            } else if new_score > score {
                score = new_score;
            }

            if new_score_g == n * n - 1 || tt > retry_cnt {
                let bd = to_board(&board_u8);
                let (_ans, last_score) = solve_by_target_board_free_empty(&state, &bd, 2, 2);
                if last_score > tmp_ans_score {
                    tmp_ans = board_u8.clone();
                    tmp_ans_score = last_score;
                }

                if tt > retry_cnt {
                    tt = 0;
                }

                if last_score as usize == n * n - 1 {
                    return to_board(&board_u8);
                } else {
                    // shuffle_board(&mut board, rng, 1000);
                    shuffle_board_u8(&mut board_u8, rng, 1000);
                    score = get_connected_size_u8(&board_u8, 0, 0);
                    score_pre = calc_board_score_u8(&board_u8);
                }
            }
        }
    }
    to_board(&tmp_ans)
}

fn get_target_board_by_rand_empty_two_score_for_6(
    state: &State,
    rng: &mut MyRng,
    tm: &mut TimeManager,
) -> Board {
    let board = state.orig_board.clone();
    let n = state.n;
    const RETRY_CNT: usize = 50000;
    let retry_cnt = RETRY_CNT;

    let check_wall_ok = |y: usize, x: usize, p: u8| -> bool {
        if y == 0 && (1 << 1) & p > 0 {
            return false;
        }
        if y == n - 1 && (1 << 3) & p > 0 {
            return false;
        }
        if x == 0 && (1 << 0) & p > 0 {
            return false;
        }
        if x == n - 1 && (1 << 2) & p > 0 {
            return false;
        }
        true
    };

    let mut board_u8 = vec![vec![!0; n]; n];
    for y in 0..n {
        for x in 0..n {
            board_u8[y][x] = board.get_pattern_num(y, x) as u8;
        }
    }
    let swap_board = |board_u8: &mut [Vec<u8>], p1: (usize, usize), p2: (usize, usize)| {
        let (y1, x1) = p1;
        let (y2, x2) = p2;
        let tmp = board_u8[y2][x2];
        board_u8[y2][x2] = board_u8[y1][x1];
        board_u8[y1][x1] = tmp;
    };

    let mut pattern_dirs = vec![vec![]; 16];
    for i in 0..16 {
        for di in 0..4 {
            if (1 << di) & i > 0 {
                pattern_dirs[i].push(di);
            }
        }
    }
    let mut pattern_from_ok_dir = [[false; 4]; 16];
    for i in 0..16 {
        for di in 0..4 {
            if (1 << di) & i > 0 {
                pattern_from_ok_dir[i][(di + 2) % 4] = true;
            }
        }
    }
    let in_area = |y: usize, x: usize| (0..n).contains(&y) && (0..n).contains(&x);

    let get_connected_size_u8 = |board_u8: &[Vec<u8>], sy: usize, sx: usize| -> usize {
        let mut visited = vec![vec![false; n]; n];
        let mut q = VecDeque::new();
        q.push_back((sy, sx));
        visited[sy][sx] = true;
        while let Some((cy, cx)) = q.pop_front() {
            let pat = board_u8[cy][cx];
            let dirs = &pattern_dirs[pat as usize];
            for &di in dirs {
                let (dy, dx) = DYXS[di];
                let (yy, xx) = (cy + dy, cx + dx);
                if !in_area(yy, xx) {
                    continue;
                }
                if visited[yy][xx] {
                    continue;
                }
                let pat2 = board_u8[yy][xx];
                if pattern_from_ok_dir[pat2 as usize][di] {
                    visited[yy][xx] = true;
                    q.push_back((yy, xx));
                }
            }
        }
        let mut ans = 0;
        for y in 0..n {
            for x in 0..n {
                if visited[y][x] {
                    ans += 1;
                }
            }
        }
        ans
    };
    let to_board = |board_u8: &[Vec<u8>]| -> Board {
        let mut new_b = Board::new(n);
        for y in 0..n {
            for x in 0..n {
                let p = board_u8[y][x] as usize;
                if p == 0 {
                    new_b.set_cell(y, x, Cell::Empty);
                } else {
                    new_b.set_cell(
                        y,
                        x,
                        Cell::piece_from_pattern(Pattern(board_u8[y][x] as usize)),
                    );
                }
            }
        }
        new_b
    };
    let shuffle_board_u8 = |board_u8: &mut [Vec<u8>], rng: &mut MyRng, cnt: usize| {
        for _ in 0..cnt {
            let nn = board.n as i32;
            let y1 = rng.get_int(0, nn - 1) as usize;
            let x1 = rng.get_int(0, nn - 1) as usize;
            let y2 = rng.get_int(0, nn - 1) as usize;
            let x2 = rng.get_int(0, nn - 1) as usize;
            swap_board(board_u8, (y1, x1), (y2, x2));
        }
    };

    let mut tmp_ans = board_u8.clone();
    let mut tt = 0;

    let e = get_empty_pos(&board);
    swap_board(&mut board_u8, e, (n - 1, n - 1));

    let mut score = 0;
    let mut tmp_ans_score = 0;
    while tm.check_time(TL, false) {
        let nn = n as i32;
        let y1 = rng.get_int(0, nn - 1) as usize;
        let x1 = rng.get_int(0, nn - 1) as usize;
        let y2 = rng.get_int(0, nn - 1) as usize;
        let x2 = rng.get_int(0, nn - 1) as usize;
        if !in_area(y2, x2) {
            continue;
        }

        if (y1, x1) == (y2, x2) {
            continue;
        }

        // 壁に向かうパターンは採用しない
        if !check_wall_ok(y1, x1, board_u8[y2][x2]) || !check_wall_ok(y2, x2, board_u8[y1][x1]) {
            continue;
        }

        if board_u8[y1][x1] == 0 && y2 != 0 && y2 != n - 1 && x2 != 0 && x2 != n - 1 {
            continue;
        }
        if board_u8[y2][x2] == 0 && y1 != 0 && y1 != n - 1 && x1 != 0 && x1 != n - 1 {
            continue;
        }

        swap_board(&mut board_u8, (y1, x1), (y2, x2));

        tt += 1;
        let new_score_g = get_connected_size_u8(&board_u8, 0, 0);
        let new_score = new_score_g;

        if new_score < score {
            swap_board(&mut board_u8, (y1, x1), (y2, x2));
        } else if new_score > score {
            score = new_score;
        }

        if new_score_g == n * n - 1 || tt > retry_cnt {
            let bd = to_board(&board_u8);
            let (_ans, last_score) = solve_by_target_board_free_empty(&state, &bd, 2, 2);
            if last_score > tmp_ans_score {
                tmp_ans = board_u8.clone();
                tmp_ans_score = last_score;
            }

            if tt > retry_cnt {
                tt = 0;
            }

            if last_score as usize == n * n - 1 {
                return to_board(&board_u8);
            } else {
                // shuffle_board(&mut board, rng, 1000);
                shuffle_board_u8(&mut board_u8, rng, 1000);
                score = get_connected_size_u8(&board_u8, 0, 0);
            }
        }
    }
    to_board(&tmp_ans)
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
    let mut min_dist = (100000, 1000000);
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
                        // let dist = sy.max(y) - sy.min(y) + sx.max(x) - sx.min(x);
                        let dist1 = (sy.max(y) - sy.min(y)).max(sx.max(x) - sx.min(x));
                        let dist2 = (sy.max(y) - sy.min(y)) + sx.max(x) - sx.min(x);
                        let dy = sy.max(y) - sy.min(y);
                        let dx = sx.max(x) - sx.min(x);
                        let d = dy * dy + dx * dx;
                        if (d, dist2) < min_dist {
                            min_dist = (dist1, dist2);
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
            let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);
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
        let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);
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

        let y = i + 1;
        let x = n - 2;
        let t_pattern = target.cells[i][n - 2].pattern();
        let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);

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
            let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);
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
        let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);
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
        let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);

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
        if get_connected_size(&board, 0, 0) == n * n - 1 {
            break;
        }
    }

    (ans, get_connected_size(&board, 0, 0) as u64)
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

    let width = 800;
    // let width = 1000;
    // let width = 500;
    // let width = ny * nx * 50;
    // let width = 15000 / (ny * nx);
    // let width = 1500;

    #[derive(Clone)]
    struct BeamState {
        ans: Vec<(usize, usize)>,
        board: Vec<Vec<u8>>,
        score: usize,
        pos_0: (usize, usize),
        hs: u64,
        val_scores: [usize; 16],
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
                            let dy = y.max(yy) - y.min(yy);
                            let dx = x.max(xx) - x.min(xx);
                            let d = dy + dx;
                            let d = dy * dy + dx * dx;

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
                let dy = py.max(ty) - py.min(ty);
                let dx = px.max(tx) - px.min(tx);
                let d = dy + dx;
                let d = dy * dy + dx * dx;
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
    const MOD: u64 = 1e18 as u64;
    let mut pows = vec![];
    let mut c = 1;
    for _ in 0..ny * nx {
        pows.push(c);
        c = (c * 16) % MOD;
    }

    let b_to_hash = |board: &[Vec<u8>]| -> u64 {
        let mut num = 0;
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                let p = pows[i] * board[y][x] as u64;
                num += p;
                // num %= MOD;
            }
        }
        num
    };

    let update_hash =
        |board: &[Vec<u8>], prev_hs: u64, p1: (usize, usize), p2: (usize, usize)| -> u64 {
            let (y1, x1) = p1;
            let (y2, x2) = p2;
            let i1 = y1 * nx + x1;
            let i2 = y2 * nx + x2;
            let plus = pows[i1] * board[y1][x1] as u64 + pows[i2] * board[y2][x2] as u64;
            let minus = pows[i1] * board[y2][x2] as u64 + pows[i2] * board[y1][x1] as u64;
            prev_hs + plus - minus
        };

    let start_hs = b_to_hash(&initial_board_u8);
    let goal_hs = b_to_hash(&target_u8);

    let mut val_scores = [0; 16];
    for num in 0..16 {
        let score_one = calc_score_one(&initial_board_u8, &target_u8, num);
        val_scores[num as usize] = score_one;
    }
    let mut val_scores_t = [0; 16];
    for num in 0..16 {
        let score_one = calc_score_one(&target_u8, &initial_board_u8, num);
        val_scores_t[num as usize] = score_one;
    }

    let first_state = BeamState {
        ans: vec![],
        board: initial_board_u8.to_vec(),
        score: calc_score(&initial_board_u8, &target_u8),
        pos_0,
        hs: start_hs,
        val_scores,
    };
    let last_state = BeamState {
        ans: vec![],
        board: target_u8.to_vec(),
        score: calc_score(&target_u8, &initial_board_u8),
        pos_0: (last_ey, last_ex),
        hs: goal_hs,
        val_scores: val_scores_t,
    };

    // let mut used_hash = HashMap::<u64, Vec<(usize, usize)>>::new();
    // let mut used_hash_rev = HashMap::<u64, Vec<(usize, usize)>>::new();

    // 移動と、前のhash
    let mut used_hash = HashMap::<u64, ((usize, usize), u64)>::new();
    let mut used_hash_rev = HashMap::<u64, ((usize, usize), u64)>::new();

    let mut states = vec![first_state];
    let mut states_rev = vec![last_state];
    for _loop_cnt in 0..1000 {
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
                // let curr_score_one = calc_score_one(&state.board, &target_u8, num);
                let curr_score_one = state.val_scores[num as usize];

                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);
                // let hs = b_to_hash(&new_state.board);
                let hs = update_hash(&new_state.board, state.hs, (y, x), (yy, xx));

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
                new_state.val_scores[num as usize] = new_score_one;

                // new_state.ans.push((dy, dx));
                // used_hash.insert(hs, new_state.ans.clone());
                used_hash.insert(hs, ((dy, dx), state.hs));

                if used_hash_rev.contains_key(&hs) {
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
                // let curr_score_one = calc_score_one(&state.board, &initial_board_u8, num);
                let curr_score_one = state.val_scores[num as usize];

                let mut new_state = state.clone();
                swap(y, x, yy, xx, &mut new_state.board);

                // let hs = b_to_hash(&new_state.board);
                let hs = update_hash(&new_state.board, state.hs, (y, x), (yy, xx));
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
                new_state.val_scores[num as usize] = new_score_one;

                // new_state.ans.push((dy, dx));
                // used_hash_rev.insert(hs, new_state.ans.clone());
                used_hash_rev.insert(hs, ((dy, dx), state.hs));

                if used_hash.contains_key(&hs) {
                    let mut ans = vec![];
                    let mut curr = hs;
                    while curr != start_hs {
                        let ((dy, dx), p_hs) = used_hash[&curr];
                        ans.push((dy, dx));
                        curr = p_hs;
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

fn get_empty_pos(board: &Board) -> (usize, usize) {
    for y in 0..board.n {
        for x in 0..board.n {
            if board.cells[y][x] == Cell::Empty {
                return (y, x);
            }
        }
    }
    unreachable!()
}

fn modify_commands_lr(com: &String) -> String {
    com.chars()
        .map(|c| match c {
            'L' => 'R',
            'R' => 'L',
            _ => c,
        })
        .collect::<String>()
}
fn modify_commands_ud(com: &String) -> String {
    com.chars()
        .map(|c| match c {
            'U' => 'D',
            'D' => 'U',
            _ => c,
        })
        .collect::<String>()
}

fn solve_row(
    y: usize,
    x0: usize,
    x1: usize,
    rev: bool,  // おく方向
    rev2: bool, // どちら側から置くか
    board: &mut Board,
    target: &Board,
    free_cells: &mut [Vec<bool>],
) -> (Vec<char>, (usize, usize)) {
    // assert!(x0 < x1);

    let dpy = if rev2 { !0 } else { 1 };
    let (mut ey, mut ex) = get_empty_pos(board);
    let xs = {
        let mut xs = (x0..=x1).collect::<Vec<_>>();
        if rev {
            xs.reverse();
        }
        xs
    };
    // assert!(xs.len() >= 2);
    let mut ans = vec![];
    let dummy = HashSet::new();

    // last 2 以外
    for i in 0..xs.len() - 2 {
        let x = xs[i];
        let t_pattern = target.cells[y][x].pattern();
        let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x), (ey, ex), board, &free_cells);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y][x] = false;
    }
    let x1 = xs[xs.len() - 2];
    let x2 = xs[xs.len() - 1];

    // x2 を x1 の位置に
    let t_pattern = target.cells[y][x2].pattern();
    let (ty, tx) = get_nearest_pattern_cell(y, x1, t_pattern, &board, &free_cells);
    let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x1), (ey, ex), board, &free_cells);
    ey = c_ey;
    ex = c_ex;
    ans.extend(com);
    free_cells[y][x1] = false;

    // x1 の真下に empty が来るように
    if (ey, ex) != (y + dpy, x1) {
        let route = get_routes((ey, ex), (y + dpy, x1), board, &free_cells, &dummy);
        for ri in 0..route.len() - 1 {
            let p1 = route[ri];
            let p2 = route[ri + 1];
            board.swap(p1, p2);
            ans.push(get_dir_char_from_swap(p1, p2));
        }
        ey = y + dpy;
        ex = x1;
    }

    // x1 を x2 の真下に
    let t_pattern = target.cells[y][x1].pattern();
    let (ty, tx) = get_nearest_pattern_cell(y + dpy, x1, t_pattern, &board, &free_cells);
    if (ty, tx) != (y, x2) {
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y + dpy, x1), (ey, ex), board, &free_cells);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y + dpy][x1] = false;
        let mut route = get_routes((ey, ex), (y, x2), board, &free_cells, &dummy);
        route.extend(vec![(y, x1), (y + dpy, x1)]);
        for ri in 0..route.len() - 1 {
            let p1 = route[ri];
            let p2 = route[ri + 1];
            board.swap(p1, p2);
            ans.push(get_dir_char_from_swap(p1, p2));
        }
        free_cells[y + dpy][x1] = true;
        ey = y + dpy;
        ex = x1;
    } else {
        if ex == x2 {
            board.swap((ey, x2), (ey, x1));
            ex = x1;
            if x1 < x2 {
                ans.push('L');
            } else {
                ans.push('R');
            }
        }
        let coms = "URDLDRULURDLURDDLUURD".to_string();
        let coms = if rev { modify_commands_lr(&coms) } else { coms };
        let coms = if rev2 {
            modify_commands_ud(&coms)
        } else {
            coms
        };
        let (c_ey, c_ex) = move_empty_cell_by_commands(coms.clone(), board, (ey, ex));
        ey = c_ey;
        ex = c_ex;
        ans.extend(coms.chars());
    }
    free_cells[y][x2] = false;
    (ans, (ey, ex))
}

fn solve_col(
    x: usize,
    y0: usize,
    y1: usize,
    rev: bool,
    rev2: bool,
    board: &mut Board,
    target: &Board,
    free_cells: &mut [Vec<bool>],
) -> (Vec<char>, (usize, usize)) {
    // assert!(y0 < y1);
    let dpx = if rev2 { !0 } else { 1 };

    let (mut ey, mut ex) = get_empty_pos(board);
    let ys = {
        let mut ys = (y0..=y1).collect::<Vec<_>>();
        if rev {
            ys.reverse();
        }
        ys
    };
    // assert!(ys.len() >= 2);
    let mut ans = vec![];
    let dummy = HashSet::new();

    // last 2 以外
    for i in 0..ys.len() - 2 {
        let y = ys[i];
        let t_pattern = target.cells[y][x].pattern();
        let (ty, tx) = get_nearest_pattern_cell(y, x, t_pattern, &board, &free_cells);
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y, x), (ey, ex), board, &free_cells);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y][x] = false;
    }
    let y1 = ys[ys.len() - 2];
    let y2 = ys[ys.len() - 1];

    // y2 を y1 の位置に
    let t_pattern = target.cells[y2][x].pattern();
    let (ty, tx) = get_nearest_pattern_cell(y1, x, t_pattern, &board, &free_cells);
    let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y1, x), (ey, ex), board, &free_cells);
    ey = c_ey;
    ex = c_ex;
    ans.extend(com);
    free_cells[y1][x] = false;

    // y1 の真右に empty が来るように
    if (ey, ex) != (y1, x + dpx) {
        let route = get_routes((ey, ex), (y1, x + dpx), board, &free_cells, &dummy);
        for ri in 0..route.len() - 1 {
            let p1 = route[ri];
            let p2 = route[ri + 1];
            board.swap(p1, p2);
            ans.push(get_dir_char_from_swap(p1, p2));
        }
        ey = y1;
        ex = x + dpx;
    }

    // x1 を x2 の真右に
    let t_pattern = target.cells[y1][x].pattern();
    let (ty, tx) = get_nearest_pattern_cell(y1, x + dpx, t_pattern, &board, &free_cells);
    if (ty, tx) != (y2, x) {
        let (com, (c_ey, c_ex)) = carry_cell((ty, tx), (y1, x + dpx), (ey, ex), board, &free_cells);
        ey = c_ey;
        ex = c_ex;
        ans.extend(com);
        free_cells[y1][x + dpx] = false;
        let mut route = get_routes((ey, ex), (y2, x), board, &free_cells, &dummy);
        route.extend(vec![(y1, x), (y1, x + dpx)]);
        for ri in 0..route.len() - 1 {
            let p1 = route[ri];
            let p2 = route[ri + 1];
            board.swap(p1, p2);
            ans.push(get_dir_char_from_swap(p1, p2));
        }
        free_cells[y1][x + dpx] = true;
        ey = y1;
        ex = x + dpx;
    } else {
        if ey == y2 {
            board.swap((y2, ex), (y1, ex));
            ey = y1;
            if y1 < y2 {
                ans.push('U');
            } else {
                ans.push('D');
            }
        }
        let coms = "LDRURDLULDRULDRRULLDR".to_string();
        let coms = if rev { modify_commands_ud(&coms) } else { coms };
        let coms = if rev2 {
            modify_commands_lr(&coms)
        } else {
            coms
        };
        let (c_ey, c_ex) = move_empty_cell_by_commands(coms.clone(), board, (ey, ex));
        ey = c_ey;
        ex = c_ex;
        ans.extend(coms.chars());
    }
    free_cells[y2][x] = false;
    (ans, (ey, ex))
}

#[allow(dead_code)]
fn solve_by_target_board_3(
    state: &State,
    tg: &Board,
    rem_y: usize,
    rem_x: usize,
    tm: &mut TimeManager,
) -> (Vec<char>, u64) {
    // assert!(rem_y <= rem_x);
    let n = state.n;
    let mut bd = state.orig_board.clone();
    let mut fcs = vec![vec![true; n]; n];

    let mut ans = vec![];

    for i in 0..n - rem_y {
        if i % 2 == 0 {
            let (com, (_, _)) = solve_row(i, i, n - 1, true, false, &mut bd, tg, &mut fcs);
            ans.extend(com);
            if i == n - rem_x {
                break;
            }
            let (com, (_eyy, _exx)) =
                solve_col(i, i + 1, n - 1, false, false, &mut bd, tg, &mut fcs);
            ans.extend(com);
        } else {
            if i == n - rem_x {
                let (com, (_eyy, _exx)) =
                    solve_row(i, i, n - 1, false, false, &mut bd, tg, &mut fcs);
                ans.extend(com);
                break;
            } else {
                let (com, (_eyy, _exx)) =
                    solve_col(i, i, n - 1, true, false, &mut bd, tg, &mut fcs);
                ans.extend(com);

                let (com, (_eyy, _exx)) =
                    solve_row(i, i + 1, n - 1, false, false, &mut bd, tg, &mut fcs);
                ans.extend(com);
            }
        }
    }

    let mut first_puzzele_board = vec![];
    for y in n - rem_y..n {
        let mut row = vec![];
        // for x in n - 3..n {
        for x in n - rem_x..n {
            let p = match bd.cells[y][x] {
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
            let p = match tg.cells[y][x] {
                Cell::Empty => 0,
                Cell::Piece(p) => *p.pattern,
            };
            row.push(p);
        }
        target_puzzele_board.push(row);
    }
    let dirs = solve_puzzle3(
        rem_y,
        rem_x,
        &first_puzzele_board,
        &target_puzzele_board,
        tm,
    );
    if dirs.is_empty() {
        return (vec![], 0);
    }
    for d in dirs {
        ans.push(Dir::from_dyx(d).to_c());
    }
    (ans, get_connected_size(&bd, 0, 0) as u64)
}

fn solve_by_target_board_free_empty(
    state: &State,
    tg: &Board,
    _rem_y: usize,
    _rem_x: usize,
) -> (Vec<char>, u64) {
    // assert!(rem_y <= rem_x);
    let n = state.n;
    let mut bd = state.orig_board.clone();
    let mut fcs = vec![vec![true; n]; n];

    let mut ans = vec![];
    #[derive(Clone, Copy, Debug)]
    enum Pos {
        Up,
        Down,
        Left,
        Right,
    }
    impl Pos {
        fn rev(&self) -> Self {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
            }
        }
    }
    // これの位置は target の empty や 最初の empty に合わせて決める
    // 今は仮置き
    let (mut p_ey, mut p_ex) = (0, 0);
    let mut pos_ud = Pos::Up;
    let mut pos_lr = Pos::Left;

    let (t_ey, t_ex) = get_empty_pos(tg);

    let get_done_col_cnt = |fcs: &[Vec<bool>]| -> usize {
        for y in 0..n {
            let x0 = (0..n).filter(|&x| fcs[y][x]).min();
            let x1 = (0..n).filter(|&x| fcs[y][x]).max();
            if x0.is_some() {
                return n - (x1.unwrap() - x0.unwrap() + 1);
            }
        }
        unreachable!()
    };
    let get_done_row_cnt = |fcs: &[Vec<bool>]| -> usize {
        for x in 0..n {
            let y0 = (0..n).filter(|&y| fcs[y][x]).min();
            let y1 = (0..n).filter(|&y| fcs[y][x]).max();
            if y0.is_some() {
                return n - (y1.unwrap() - y0.unwrap() + 1);
            }
        }
        unreachable!()
    };

    let mut done_row = vec![false; n];
    let mut done_col = vec![false; n];
    for _ in 0..100 {
        let done_row_cnt = get_done_row_cnt(&fcs);
        let done_col_cnt = get_done_col_cnt(&fcs);

        if n - done_col_cnt == 2 && n - done_row_cnt == 2 {
            break;
        }

        let empty_same = (p_ey, p_ex) == (t_ey, t_ex);
        let empty_all_diff = p_ey != t_ey && p_ex != t_ex;
        let x_same = p_ex == t_ex;
        let y_same = p_ey == t_ey;
        let row_done = n - done_row_cnt <= 2;
        let col_done = n - done_col_cnt <= 2;
        let prior_row = done_row_cnt <= done_col_cnt;

        // eprintln!("-------");
        // eprintln!(" {} {}", done_row_cnt, done_col_cnt);
        // eprintln!("{:?} {:?} {:?}", (p_ey, p_ex), pos_ud, pos_lr);

        // とりあえず横か縦に動かす
        // - 完全に empty が被っているとき
        // - x が被っているので row をしたいが、 row は終わっている時
        // - y が被っているので col をしたいが、 col は終わっている時
        if empty_same || (x_same && row_done) || (y_same && col_done) {
            // row を潰したいので まず縦に移動
            if prior_row {
                // eprintln!("  move col");
                let y0 = (0..n).filter(|&y| fcs[y][p_ex]).min().unwrap();
                let y1 = (0..n).filter(|&y| fcs[y][p_ex]).max().unwrap();
                pos_ud = pos_ud.rev();
                p_ey = match pos_ud {
                    Pos::Up => y0,
                    Pos::Down => y1,
                    _ => unreachable!(),
                };
            } else {
                // eprintln!("  move row");
                let x0 = (0..n).filter(|&x| fcs[p_ey][x]).min().unwrap();
                let x1 = (0..n).filter(|&x| fcs[p_ey][x]).max().unwrap();
                pos_lr = pos_lr.rev();
                p_ex = match pos_lr {
                    Pos::Left => x0,
                    Pos::Right => x1,
                    _ => unreachable!(),
                };
            }
        }
        // - x が被っていて row がまだ終わっていないので row をする
        else if x_same || (empty_all_diff && prior_row) {
            // eprintln!("  row");
            let y = p_ey;
            let x0 = (0..n).filter(|&x| fcs[y][x]).min().unwrap();
            let x1 = (0..n).filter(|&x| fcs[y][x]).max().unwrap();
            let rev = match pos_lr {
                Pos::Left => false,
                Pos::Right => true,
                _ => unreachable!(),
            };
            let rev2 = match pos_ud {
                Pos::Up => false,
                Pos::Down => true,
                _ => unreachable!(),
            };
            let (com, (_, _)) = solve_row(y, x0, x1, rev, rev2, &mut bd, tg, &mut fcs);
            ans.extend(com);
            pos_lr = pos_lr.rev();
            p_ex = if rev { x0 } else { x1 };
            p_ey = if rev2 { p_ey - 1 } else { p_ey + 1 };
            done_row[y] = true;
        } else {
            // eprintln!("  col");
            let x = p_ex;
            let y0 = (0..n).filter(|&y| fcs[y][x]).min().unwrap();
            let y1 = (0..n).filter(|&y| fcs[y][x]).max().unwrap();
            let rev = match pos_ud {
                Pos::Up => false,
                Pos::Down => true,
                _ => unreachable!(),
            };
            let rev2 = match pos_lr {
                Pos::Left => false,
                Pos::Right => true,
                _ => unreachable!(),
            };
            let (com, (_, _)) = solve_col(x, y0, y1, rev, rev2, &mut bd, tg, &mut fcs);
            ans.extend(com);
            pos_ud = pos_ud.rev();
            p_ey = if rev { y0 } else { y1 };
            p_ex = if rev2 { p_ex - 1 } else { p_ex + 1 };
            done_col[x] = true;
        }
        // break;
    }
    // eprintln!("ng!");

    let (mut ey, mut ex) = get_empty_pos(&bd);
    // eprintln!("{:?}", fcs);
    // eprintln!("{:?}", (ey, ex));
    let mut di = 0;
    for _ in 0..20 {
        // eprintln!("{:?}", (ey, ex));

        let mut ok = true;
        for y in 0..n {
            for x in 0..n {
                if bd.get_pattern_num(y, x) != tg.get_pattern_num(y, x) {
                    ok = false;
                }
            }
        }
        if ok {
            // println!("{}", get_connected_size(&bd, 0, 0));
            break;
        }
        // for &(dy, dx) in &DYXS {
        let (dy, dx) = DYXS[di];
        di += 1;
        di %= 4;
        let (yy, xx) = (ey + dy, ex + dx);
        if !bd.in_area(yy, xx) {
            continue;
        }
        if fcs[yy][xx] {
            bd.swap((yy, xx), (ey, ex));
            ey = yy;
            ex = xx;
            ans.push(Dir::from_dyx((dy, dx)).to_c());
        }
        // }
    }

    return (ans, get_connected_size(&bd, 0, 0) as u64);
}

fn solve_by_target_board_free_empty_2(
    state: &State,
    tg: &Board,
    rem_y: usize,
    rem_x: usize,
    tm: &mut TimeManager,
) -> (Vec<char>, u64) {
    // assert!(rem_y <= rem_x);
    let n = state.n;
    let mut bd = state.orig_board.clone();
    let mut fcs = vec![vec![true; n]; n];

    let mut ans = vec![];
    #[derive(Clone, Copy, Debug)]
    enum Pos {
        Up,
        Down,
        Left,
        Right,
    }
    impl Pos {
        fn rev(&self) -> Self {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
            }
        }
    }
    let (t_ey, t_ex) = get_empty_pos(tg);

    // これの位置は target の empty や 最初の empty に合わせて決める
    // 今は仮置き
    // let (mut p_ey, mut p_ex) = (0, 0);
    // let mut pos_ud = Pos::Up;
    // let mut pos_lr = Pos::Left;
    let (mut p_ey, mut pos_ud) = if t_ey < n / 2 {
        (0, Pos::Up)
    } else {
        (n - 1, Pos::Down)
    };
    let (mut p_ex, mut pos_lr) = if t_ex < n / 2 {
        (0, Pos::Left)
    } else {
        (n - 1, Pos::Right)
    };

    // eprintln!("ttt: {:?}", (t_ey, t_ex));

    let get_done_col_cnt = |fcs: &[Vec<bool>]| -> usize {
        for y in 0..n {
            let x0 = (0..n).filter(|&x| fcs[y][x]).min();
            let x1 = (0..n).filter(|&x| fcs[y][x]).max();
            if x0.is_some() {
                return n - (x1.unwrap() - x0.unwrap() + 1);
            }
        }
        unreachable!()
    };
    let get_done_row_cnt = |fcs: &[Vec<bool>]| -> usize {
        for x in 0..n {
            let y0 = (0..n).filter(|&y| fcs[y][x]).min();
            let y1 = (0..n).filter(|&y| fcs[y][x]).max();
            if y0.is_some() {
                return n - (y1.unwrap() - y0.unwrap() + 1);
            }
        }
        unreachable!()
    };

    let mut done_row = vec![false; n];
    let mut done_col = vec![false; n];
    for _ in 0..20 {
        let done_row_cnt = get_done_row_cnt(&fcs);
        let done_col_cnt = get_done_col_cnt(&fcs);

        let todo_row_cnt = (n - done_row_cnt) - rem_y;
        let todo_col_cnt = (n - done_col_cnt) - rem_x;

        if todo_row_cnt == 0 && todo_col_cnt == 0 {
            break;
        }

        let empty_same = (p_ey, p_ex) == (t_ey, t_ex);
        let empty_all_diff = p_ey != t_ey && p_ex != t_ex;
        let x_same = p_ex == t_ex;
        let y_same = p_ey == t_ey;
        let row_done = todo_row_cnt == 0;
        let col_done = todo_col_cnt == 0;
        let prior_row = todo_row_cnt >= todo_col_cnt;

        // eprintln!("-------");
        // eprintln!(" done_row: {}, done_col: {}", done_row_cnt, done_col_cnt);
        // eprintln!(" todo_row: {}, todo_col: {}", todo_row_cnt, todo_col_cnt);
        // eprintln!("{:?} {:?} {:?}", (p_ey, p_ex), pos_ud, pos_lr);

        // とりあえず横か縦に動かす
        // - 完全に empty が被っているとき
        // - x が被っているので row をしたいが、 row は終わっている時
        // - y が被っているので col をしたいが、 col は終わっている時
        if empty_same || (x_same && row_done) || (y_same && col_done) {
            // row を潰したいので まず縦に移動
            if prior_row {
                // eprintln!("  move col");
                let y0 = (0..n).filter(|&y| fcs[y][p_ex]).min().unwrap();
                let y1 = (0..n).filter(|&y| fcs[y][p_ex]).max().unwrap();
                pos_ud = pos_ud.rev();
                p_ey = match pos_ud {
                    Pos::Up => y0,
                    Pos::Down => y1,
                    _ => unreachable!(),
                };
            } else {
                // eprintln!("  move row");
                let x0 = (0..n).filter(|&x| fcs[p_ey][x]).min().unwrap();
                let x1 = (0..n).filter(|&x| fcs[p_ey][x]).max().unwrap();
                pos_lr = pos_lr.rev();
                p_ex = match pos_lr {
                    Pos::Left => x0,
                    Pos::Right => x1,
                    _ => unreachable!(),
                };
            }
        }
        // - x が被っていて row がまだ終わっていないので row をする
        else if x_same || (empty_all_diff && prior_row) {
            // eprintln!("  row {}", p_ey);
            let y = p_ey;
            let x0 = (0..n).filter(|&x| fcs[y][x]).min().unwrap();
            let x1 = (0..n).filter(|&x| fcs[y][x]).max().unwrap();
            let rev = match pos_lr {
                Pos::Left => false,
                Pos::Right => true,
                _ => unreachable!(),
            };
            let rev2 = match pos_ud {
                Pos::Up => false,
                Pos::Down => true,
                _ => unreachable!(),
            };
            let (com, (_, _)) = solve_row(y, x0, x1, rev, rev2, &mut bd, tg, &mut fcs);
            ans.extend(com);
            pos_lr = pos_lr.rev();
            p_ex = if rev { x0 } else { x1 };
            p_ey = if rev2 { p_ey - 1 } else { p_ey + 1 };
            done_row[y] = true;
        } else {
            // eprintln!("  col {}", p_ex);
            let x = p_ex;
            let y0 = (0..n).filter(|&y| fcs[y][x]).min().unwrap();
            let y1 = (0..n).filter(|&y| fcs[y][x]).max().unwrap();
            let rev = match pos_ud {
                Pos::Up => false,
                Pos::Down => true,
                _ => unreachable!(),
            };
            let rev2 = match pos_lr {
                Pos::Left => false,
                Pos::Right => true,
                _ => unreachable!(),
            };
            let (com, (_, _)) = solve_col(x, y0, y1, rev, rev2, &mut bd, tg, &mut fcs);
            ans.extend(com);
            pos_ud = pos_ud.rev();
            p_ey = if rev { y0 } else { y1 };
            p_ex = if rev2 { p_ex - 1 } else { p_ex + 1 };
            done_col[x] = true;
        }
        // break;
    }

    let (mut sy, mut sx) = (0, 0);
    'outer: for y in 0..n {
        for x in 0..n {
            if fcs[y][x] {
                sy = y;
                sx = x;
                break 'outer;
            }
        }
    }

    let mut first_puzzele_board = vec![vec![!0; rem_x]; rem_y];
    let mut target_puzzele_board = vec![vec![!0; rem_x]; rem_y];
    for y in sy..sy + rem_y {
        for x in sx..sx + rem_x {
            first_puzzele_board[y - sy][x - sx] = bd.get_pattern_num(y, x);
            target_puzzele_board[y - sy][x - sx] = tg.get_pattern_num(y, x);
        }
    }

    let dirs = solve_puzzle3(
        rem_y,
        rem_x,
        &first_puzzele_board,
        &target_puzzele_board,
        tm,
    );
    if dirs.is_empty() {
        return (vec![], 0);
    }
    for d in dirs {
        ans.push(Dir::from_dyx(d).to_c());
    }
    (ans, get_connected_size(&bd, 0, 0) as u64)
}

// 最初の貪欲パートの開始位置を複数試す
fn solve_by_target_board_free_empty_3(
    state: &State,
    tg: &Board,
    rem_y: usize,
    rem_x: usize,
    tm: &mut TimeManager,
) -> (Vec<char>, u64) {
    // assert!(rem_y <= rem_x);
    let n = state.n;
    let mut bd = state.orig_board.clone();
    let mut fcs = vec![vec![true; n]; n];

    let mut ans = vec![];
    #[derive(Clone, Copy, Debug)]
    enum Pos {
        Up,
        Down,
        Left,
        Right,
    }
    impl Pos {
        fn rev(&self) -> Self {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
            }
        }
    }
    let (t_ey, t_ex) = get_empty_pos(tg);

    let starts = vec![
        ((0, 0), (Pos::Up, Pos::Left)),
        ((n - 1, 0), (Pos::Down, Pos::Left)),
        ((0, n - 1), (Pos::Up, Pos::Right)),
        ((n - 1, n - 1), (Pos::Down, Pos::Right)),
    ];

    // eprintln!("ttt: {:?}", (t_ey, t_ex));

    let get_done_col_cnt = |fcs: &[Vec<bool>]| -> usize {
        for y in 0..n {
            let x0 = (0..n).filter(|&x| fcs[y][x]).min();
            let x1 = (0..n).filter(|&x| fcs[y][x]).max();
            if x0.is_some() {
                return n - (x1.unwrap() - x0.unwrap() + 1);
            }
        }
        unreachable!()
    };
    let get_done_row_cnt = |fcs: &[Vec<bool>]| -> usize {
        for x in 0..n {
            let y0 = (0..n).filter(|&y| fcs[y][x]).min();
            let y1 = (0..n).filter(|&y| fcs[y][x]).max();
            if y0.is_some() {
                return n - (y1.unwrap() - y0.unwrap() + 1);
            }
        }
        unreachable!()
    };

    let mut best_len = 1000000;
    let mut best_ans = vec![];
    let mut best_board = bd.clone();
    let mut best_fcs = fcs.clone();

    for &((mut p_ey, mut p_ex), (mut pos_ud, mut pos_lr)) in &starts {
        // println!("{} {} {:?} {:?}", p_ey, p_ex, pos_ud, pos_lr);
        let mut c_bd = state.orig_board.clone();
        let mut c_fcs = vec![vec![true; n]; n];
        let mut done_row = vec![false; n];
        let mut done_col = vec![false; n];
        let mut c_ans = vec![];
        for _ in 0..50 {
            let done_row_cnt = get_done_row_cnt(&c_fcs);
            let done_col_cnt = get_done_col_cnt(&c_fcs);

            let todo_row_cnt = (n - done_row_cnt) - rem_y;
            let todo_col_cnt = (n - done_col_cnt) - rem_x;

            if todo_row_cnt == 0 && todo_col_cnt == 0 {
                if best_len > c_ans.len() {
                    best_len = c_ans.len();
                    best_ans = c_ans;
                    best_board = c_bd.clone();
                    best_fcs = c_fcs.clone();
                }
                break;
            }

            let empty_same = (p_ey, p_ex) == (t_ey, t_ex);
            let empty_all_diff = p_ey != t_ey && p_ex != t_ex;
            let x_same = p_ex == t_ex;
            let y_same = p_ey == t_ey;
            let row_done = todo_row_cnt == 0;
            let col_done = todo_col_cnt == 0;
            let prior_row = todo_row_cnt >= todo_col_cnt;

            // eprintln!("-------");
            // eprintln!(" done_row: {}, done_col: {}", done_row_cnt, done_col_cnt);
            // eprintln!(" todo_row: {}, todo_col: {}", todo_row_cnt, todo_col_cnt);
            // eprintln!("{:?} {:?} {:?}", (p_ey, p_ex), pos_ud, pos_lr);

            // とりあえず横か縦に動かす
            // - 完全に empty が被っているとき
            // - x が被っているので row をしたいが、 row は終わっている時
            // - y が被っているので col をしたいが、 col は終わっている時
            if empty_same || (x_same && row_done) || (y_same && col_done) {
                // row を潰したいので まず縦に移動
                if prior_row {
                    // eprintln!("  move col");
                    let y0 = (0..n).filter(|&y| c_fcs[y][p_ex]).min().unwrap();
                    let y1 = (0..n).filter(|&y| c_fcs[y][p_ex]).max().unwrap();
                    pos_ud = pos_ud.rev();
                    p_ey = match pos_ud {
                        Pos::Up => y0,
                        Pos::Down => y1,
                        _ => unreachable!(),
                    };
                } else {
                    // eprintln!("  move row");
                    let x0 = (0..n).filter(|&x| c_fcs[p_ey][x]).min().unwrap();
                    let x1 = (0..n).filter(|&x| c_fcs[p_ey][x]).max().unwrap();
                    pos_lr = pos_lr.rev();
                    p_ex = match pos_lr {
                        Pos::Left => x0,
                        Pos::Right => x1,
                        _ => unreachable!(),
                    };
                }
            }
            // - x が被っていて row がまだ終わっていないので row をする
            else if x_same || (empty_all_diff && prior_row) {
                // eprintln!("  row {}", p_ey);
                let y = p_ey;
                let x0 = (0..n).filter(|&x| c_fcs[y][x]).min().unwrap();
                let x1 = (0..n).filter(|&x| c_fcs[y][x]).max().unwrap();
                let rev = match pos_lr {
                    Pos::Left => false,
                    Pos::Right => true,
                    _ => unreachable!(),
                };
                let rev2 = match pos_ud {
                    Pos::Up => false,
                    Pos::Down => true,
                    _ => unreachable!(),
                };
                let (com, (_, _)) = solve_row(y, x0, x1, rev, rev2, &mut c_bd, tg, &mut c_fcs);
                c_ans.extend(com);
                pos_lr = pos_lr.rev();
                p_ex = if rev { x0 } else { x1 };
                p_ey = if rev2 { p_ey - 1 } else { p_ey + 1 };
                done_row[y] = true;
            } else {
                // eprintln!("  col {}", p_ex);
                let x = p_ex;
                let y0 = (0..n).filter(|&y| c_fcs[y][x]).min().unwrap();
                let y1 = (0..n).filter(|&y| c_fcs[y][x]).max().unwrap();
                let rev = match pos_ud {
                    Pos::Up => false,
                    Pos::Down => true,
                    _ => unreachable!(),
                };
                let rev2 = match pos_lr {
                    Pos::Left => false,
                    Pos::Right => true,
                    _ => unreachable!(),
                };
                let (com, (_, _)) = solve_col(x, y0, y1, rev, rev2, &mut c_bd, tg, &mut c_fcs);
                c_ans.extend(com);
                pos_ud = pos_ud.rev();
                p_ey = if rev { y0 } else { y1 };
                p_ex = if rev2 { p_ex - 1 } else { p_ex + 1 };
                done_col[x] = true;
            }
            // break;
        }
    }
    // eprintln!("aa: {:?}", ans);
    // eprintln!("len: {:?}", best_len);
    ans.extend(best_ans);
    bd = best_board;
    fcs = best_fcs;

    let (mut sy, mut sx) = (0, 0);
    'outer: for y in 0..n {
        for x in 0..n {
            if fcs[y][x] {
                sy = y;
                sx = x;
                break 'outer;
            }
        }
    }

    let mut first_puzzele_board = vec![vec![!0; rem_x]; rem_y];
    let mut target_puzzele_board = vec![vec![!0; rem_x]; rem_y];
    for y in sy..sy + rem_y {
        for x in sx..sx + rem_x {
            first_puzzele_board[y - sy][x - sx] = bd.get_pattern_num(y, x);
            target_puzzele_board[y - sy][x - sx] = tg.get_pattern_num(y, x);
        }
    }

    let dirs = solve_puzzle3(
        rem_y,
        rem_x,
        &first_puzzele_board,
        &target_puzzele_board,
        tm,
    );
    if dirs.is_empty() {
        return (vec![], 0);
    }
    for d in dirs {
        ans.push(Dir::from_dyx(d).to_c());
    }
    (ans, get_connected_size(&bd, 0, 0) as u64)
}

/* ------------------------------------------------------------------------
    solve!
------------------------------------------------------------------------ */
fn solve() {
    /* ------------ ready ------------ */
    let mut tm = mytool::TimeManager::new();
    let mut rng = mytool::MyRng::new();
    let state = State::new();
    let n = state.n;
    /* ------------ ready ------------ */

    // let target_board = get_target_board_by_rand4(&state, &mut rng, &mut tm);
    // let target_board = get_target_board_by_rand3(&state, &mut rng, &mut tm);
    // let target_board = get_target_board_by_rand_empty4(&state, &mut rng, &mut tm);
    // let target_board = get_target_board_by_rand_empty_two_score(&state, &mut rng, &mut tm);
    let target_board = if n <= 6 {
        // get_target_board_by_rand_empty4(&state, &mut rng, &mut tm)
        get_target_board_by_rand_empty_two_score_for_6(&state, &mut rng, &mut tm)
    } else {
        get_target_board_by_rand_empty_two_score_2(&state, &mut rng, &mut tm)
        // get_target_board_by_rand_empty_two_score(&state, &mut rng, &mut tm)
    };

    let t_tree = tm.get_time();
    if cfg!(feature = "mylocal") {
        dbg!(t_tree);
    }
    // target_board.print_pattern();

    // let (ans1, _) = solve_by_target_board_free_empty_2(&state, &target_board, 3, 3, &mut tm);
    // println!("{}", ans1.iter().collect::<String>());
    // let g = get_connected_size(&target_board, 0, 0);
    // if g != n * n - 1 {
    //     dbg!("no tree....");
    // }
    // return;

    // let (ans1, _) = solve_by_target_board(&state, &target_board);
    // let (ans2, _) = solve_by_target_board_3(&state, &target_board, 5, 5, &mut tm);
    // let (ans3, _) = solve_by_target_board_3(&state, &target_board, 6, 6, &mut tm);
    // let t_puzzle = tm.get_time();

    let (ans1, _) = solve_by_target_board_free_empty(&state, &target_board, 0, 0);

    // let a = solve_by_target_board_free_empty_3(&state, &target_board, 4, 4, &mut tm);
    // dbg!(a.0.iter().collect::<String>());

    // let (ans1, _) = solve_by_target_board_free_empty_2(&state, &target_board, 3, 3, &mut tm);
    // let (ans2, _) = solve_by_target_board_free_empty_2(&state, &target_board, 5, 5, &mut tm);
    // let (ans3, _) = solve_by_target_board_free_empty_2(&state, &target_board, 6, 6, &mut tm);
    let (ans2, _) = if tm.get_time() < TL * 3 / 4 {
        // let (ans2, _) = if tm.get_time() < TL {
        solve_by_target_board_free_empty_3(&state, &target_board, 5, 5, &mut tm)
    } else {
        solve_by_target_board_free_empty_3(&state, &target_board, 4, 4, &mut tm)
        // solve_by_target_board_free_empty_3(&state, &target_board, 5, 5, &mut tm)
    };
    // let (ans2, _) = solve_by_target_board_free_empty_3(&state, &target_board, 5, 5, &mut tm);
    let (mut ans3, _) = solve_by_target_board_free_empty_3(&state, &target_board, 6, 6, &mut tm);
    while tm.check_time(TL, true) {
        // let target_board = get_target_board_by_rand_empty4(&state, &mut rng, &mut tm);
        let target_board = get_target_board_by_rand_empty_two_score_2(&state, &mut rng, &mut tm);
        let (ans4, _) = solve_by_target_board_free_empty_3(&state, &target_board, 6, 6, &mut tm);
        if ans4.len() != 0 && ans4.len() < ans3.len() {
            ans3 = ans4;
        }
    }

    let t_puzzle = tm.get_time();

    let s = if ans2.is_empty() && ans3.is_empty() {
        ans1.iter().collect::<String>()
    } else if ans3.is_empty() {
        ans2.iter().collect::<String>()
    } else {
        if ans2.len() > ans3.len() {
            ans3.iter().collect::<String>()
        } else {
            ans2.iter().collect::<String>()
        }
    };
    println!("{}", s);

    if cfg!(feature = "mylocal") {
        dbg!(t_puzzle);
        dbg!(ans1.len());
        dbg!(ans2.len());
        dbg!(ans3.len());
        let mut bd = state.orig_board.clone();
        move_empty_cell_by_commands(s.clone(), &mut bd, state.orig_empty_pos);
        let g = get_connected_size(&bd, 0, 0);
        // let g = get_connected_size(&target_board, 0, 0);
        if g != n * n - 1 {
            dbg!("no tree....");
        }
        if g != n * n - 1 {
            eprintln!("Score = {}", 500000 * g / (n * n - 1));
        } else {
            let coef = 2.0 - s.len() as f64 / state.t as f64;
            eprintln!("Score = {}", (500000.0 * coef) as u64);
        }
    }
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
