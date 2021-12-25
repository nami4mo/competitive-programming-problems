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

const FIRST_EST_SKILL: i64 = 10;
const UPDATE_SKILL_LOOP_CNT: usize = 3600;

/* --------------------- enum --------------------- */
#[derive(Debug, PartialEq)]
enum Progress {
    Pending,
    Reserved,
    Proceeding(usize), // the day of start.
    Done,
}
impl Default for Progress {
    fn default() -> Self {
        Progress::Pending
    }
}

#[derive(Debug, PartialEq)]
pub enum Working {
    Free,
    OnWork(usize, usize), // task_id, est_end_time
}
impl Default for Working {
    fn default() -> Self {
        Working::Free
    }
}

/* --------------------- struct --------------------- */
#[derive(Debug, PartialEq, Default)]
pub struct Task {
    id: usize,
    req_skills: Vec<i64>,
    rem_deps: HashSet<usize>, // # element is removed when the dep is DONE
    deps: Vec<usize>,         // not mut
    children: Vec<usize>,
    progress: Progress,
    importance: usize,          // # of children. TODO: improve this
    reserved_by: Option<usize>, // member_id
    reserved_est_time: Option<usize>,
    norm: usize,
}

impl Task {
    pub fn start(&mut self, day: usize) {
        self.progress = Progress::Proceeding(day);
    }
    pub fn is_available(&self) -> bool {
        self.rem_deps.is_empty() && self.progress == Progress::Pending
    }
    pub fn is_proceeding(&self) -> bool {
        if let Progress::Proceeding(_) = self.progress {
            true
        } else {
            false
        }
    }
    pub fn reserve(&mut self, member_id: usize, est_time: usize) {
        self.reserved_by = Some(member_id);
        self.reserved_est_time = Some(est_time);
        self.progress = Progress::Reserved;
    }
    // called only when task is Proceeding. (= not called after task becomes Done.)
    pub fn get_start_day(&self) -> Option<usize> {
        match self.progress {
            Progress::Proceeding(start_day) => Some(start_day),
            _ => None,
        }
    }
    pub fn complete_task(&mut self) {
        assert!(self.is_proceeding());
        self.progress = Progress::Done;
    }
    pub fn complete_dep(&mut self, dep_task_id: usize) {
        assert!(self.rem_deps.contains(&dep_task_id));
        self.rem_deps.remove(&dep_task_id);
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Member {
    id: usize,
    skills: Vec<i64>,
    working: Working,
    task_results: Vec<TaskResult>, // the results of the completed tasks
    reserved_tasks: VecDeque<usize>, // task_id
}
impl Member {
    pub fn start_task(&mut self, task_id: usize, _req_skills: &Vec<i64>, est_end_day: usize) {
        self.working = Working::OnWork(task_id, est_end_day);
    }
    pub fn pop_reserved_tasks(&mut self) {
        self.reserved_tasks.pop_front();
    }
    pub fn reserve_task(&mut self, task_id: usize) {
        self.reserved_tasks.push_back(task_id);
    }
    pub fn is_available(&self) -> bool {
        self.working == Working::Free
    }
    pub fn is_working(&self) -> bool {
        if let Working::OnWork(_, _) = self.working {
            true
        } else {
            false
        }
    }
    pub fn get_current_task_id(&self) -> Option<usize> {
        match self.working {
            Working::OnWork(task_id, _) => Some(task_id),
            _ => None,
        }
    }
    pub fn get_current_task_est_end(&self) -> usize {
        assert!(self.is_working());
        match self.working {
            Working::OnWork(_, est_end_time) => est_end_time,
            _ => panic!(),
        }
    }
    pub fn complete_task(&mut self, task_result: TaskResult) {
        self.working = Working::Free;
        self.task_results.push(task_result);
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct TaskResult {
    task_id: usize,
    time: i64,
    est_time: i64,
}

#[derive(Default, Debug, PartialEq)]
pub struct State {
    n: usize, // # of task
    m: usize, // # of member
    k: usize, // # of skill
    r: i64,   // # of dep-edge
    day: usize,
    done_cnt: usize,
    tasks: Vec<Task>,

    tasks_order: Vec<(usize, i64, i64)>, // # (task_id, importance, norm)
    important_tasks_order: Vec<(usize, i64, i64)>, // # (task_id, importance, norm)
    anytime_tasks_order: Vec<(usize, i64, i64)>, // # (task_id, importance, norm)
    anytime_tasks_cnt: usize,

    members: Vec<Member>,
}

impl State {
    pub fn advance_day(&mut self) -> () {
        self.day += 1;
    }
    pub fn is_task_available(&self, task_id: usize) -> bool {
        self.tasks[task_id].is_available()
    }
    fn assert_available(&self, member: &Member, task: &Task) {
        assert!(
            member.is_available(),
            "member {} is not available!",
            member.id
        );
        assert!(
            task.is_available(),
            "task {} is {:?}",
            task.id,
            task.progress
        );
    }
    pub fn member_starts_task(&mut self, member_id: usize, task_id: usize) {
        self.assert_available(&self.members[member_id], &self.tasks[task_id]);
        let est_time = calc_est_time(self, member_id, task_id);
        self.members[member_id].start_task(
            task_id,
            &self.tasks[task_id].req_skills,
            est_time as usize + self.day,
        );
        self.tasks[task_id].start(self.day);
    }
    pub fn member_starts_reserved_task(&mut self, member_id: usize) {
        let task_id = *self.members[member_id].reserved_tasks.front().unwrap();
        let est_time = calc_est_time(self, member_id, task_id);
        self.members[member_id].start_task(
            task_id,
            &self.tasks[task_id].req_skills,
            est_time as usize + self.day,
        );
        self.members[member_id].pop_reserved_tasks();
        self.tasks[task_id].start(self.day);
    }
    pub fn complete_task(&mut self, member_id: usize) {
        assert!(
            self.members[member_id].is_working(),
            "member {} is not working!",
            member_id
        );
        let task_id = self.members[member_id].get_current_task_id().unwrap();
        assert!(
            self.tasks[task_id].is_proceeding(),
            "task {} is not proceeding!",
            task_id
        );
        let start_day = self.tasks[task_id].get_start_day().unwrap();
        let task_result = TaskResult {
            task_id,
            time: (self.day - start_day) as i64,
            est_time: calc_est_time(&self, member_id, task_id),
        };
        self.members[member_id].complete_task(task_result);
        self.tasks[task_id].complete_task();
        self.update_deps_info(task_id);
        self.done_cnt += 1;
    }
    pub fn update_deps_info(&mut self, task_id: usize) {
        let children = self.tasks[task_id].children.clone();
        for child_id in &children {
            self.tasks[*child_id].complete_dep(task_id);
        }
    }
    pub fn reserve_task(&mut self, member_id: usize, task_id: usize) {
        self.members[member_id].reserve_task(task_id);
        let est_time = calc_est_time(self, member_id, task_id);
        self.tasks[task_id].reserve(member_id, est_time as usize);
    }

    pub fn new(stdin: &mut LineSource<BufReader<Stdin>>) -> Self {
        input! {
            from stdin,
            n: usize, m: usize, k: usize, r: i64,
            req_skills: [[i64; k]; n], edges: [(Usize1, Usize1); r]
        }

        let mut members = vec![];
        for member_id in 0..m {
            members.push(Member {
                id: member_id,
                skills: vec![FIRST_EST_SKILL; k],
                ..Default::default()
            });
        }

        let mut children = vec![Vec::<usize>::new(); n];
        let mut deps = vec![Vec::<usize>::new(); n];
        let mut rem_deps = vec![HashSet::<usize>::new(); n];
        let mut important_tasks_id_importance_norm = vec![];
        let mut anytime_tasks_id_importance_norm = vec![];
        let mut tasks = vec![];

        for (u, v) in &edges {
            children[*u].push(*v);
            deps[*v].push(*u);
            rem_deps[*v].insert(*u);
        }

        for task_id in 0..n {
            let mut norm = 0;
            for v in &req_skills[task_id] {
                norm += v * v;
            }
            norm = sqrt(norm);
            tasks.push(Task {
                id: task_id,
                req_skills: req_skills[task_id].clone(),
                rem_deps: rem_deps[task_id].clone(),
                deps: deps[task_id].clone(),
                children: children[task_id].clone(),
                norm: norm as usize,
                ..Default::default()
            });
        }

        // let importances = calc_tasks_importance(&tasks);
        let importances = calc_tasks_importance_citical(&tasks);

        for i in 0..n {
            tasks[i].importance = importances[i];
            if importances[i] == 1 {
                anytime_tasks_id_importance_norm.push((
                    i,
                    tasks[i].importance as i64,
                    calc_task_norm(&tasks[i]),
                ));
            } else {
                important_tasks_id_importance_norm.push((
                    i,
                    tasks[i].importance as i64,
                    calc_task_norm(&tasks[i]),
                ));
            }
        }
        important_tasks_id_importance_norm.sort_by_key(|x| (-x.1, x.0, -x.2));
        anytime_tasks_id_importance_norm.sort_by_key(|x| -x.2);
        let tasks_order = [
            important_tasks_id_importance_norm.clone(),
            anytime_tasks_id_importance_norm.clone(),
        ]
        .concat();

        State {
            n,
            m,
            k,
            r,
            day: 0,
            done_cnt: 0,
            members,
            tasks,
            anytime_tasks_order: anytime_tasks_id_importance_norm.clone(),
            important_tasks_order: important_tasks_id_importance_norm.clone(),
            tasks_order,
            anytime_tasks_cnt: anytime_tasks_id_importance_norm.len(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Assign {
    member_id: usize,
    task_id: usize,
    reserve: bool,
}

// #[cfg(feature = "dbg")]
#[derive(Default, Debug, PartialEq)]
struct Judge {
    n: usize,
    m: usize,
    k: usize,
    ans_member_skills: Vec<Vec<i64>>,
    ans_task_member_times: Vec<Vec<i64>>,
    day: i64,
    members_done_day: Vec<i64>,
    today_done_members: Vec<usize>,
    done_task_cnt: usize,
    assigned_tasks: HashSet<usize>,
}

impl Judge {
    pub fn new(stdin: &mut LineSource<BufReader<Stdin>>, n: usize, m: usize, k: usize) -> Self {
        let mut ans_member_skills = vec![];
        let mut ans_task_member_times = vec![];
        if cfg!(feature = "dbg") {
            input! {
                from stdin,
                ans_skills: [[i64; k]; m],
                ans_task_time: [[i64; m]; n],
            }
            ans_member_skills = ans_skills;
            ans_task_member_times = ans_task_time;
        }
        Judge {
            n,
            m,
            k,
            day: 0,
            done_task_cnt: 0,
            members_done_day: vec![-1; m],
            ans_member_skills,
            ans_task_member_times,
            assigned_tasks: HashSet::<usize>::new(),
            ..Default::default()
        }
    }
    pub fn get_done_cnt(&self) -> i64 {
        if self.done_task_cnt == self.n || self.day == 2000 {
            -1
        } else {
            self.today_done_members.len() as i64
        }
    }
    pub fn get_done_members(&self) -> Vec<usize> {
        self.today_done_members.clone()
    }

    #[allow(dead_code)]
    pub fn advance_day(&mut self) {
        self.today_done_members.clear();
        self.day += 1;
        for member_id in 0..self.m {
            if self.members_done_day[member_id] == self.day {
                self.today_done_members.push(member_id);
                self.members_done_day[member_id] = -1;
                self.done_task_cnt += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn member_starts_task(&mut self, member_id: usize, task_id: usize) {
        // eprintln!("m: {}, t: {}", member_id, task_id);
        assert!(
            !self.assigned_tasks.contains(&task_id),
            "this task is already assigned."
        );
        assert_eq!(
            self.members_done_day[member_id], -1,
            "[Judge] member {} is working",
            member_id
        );
        self.assigned_tasks.insert(task_id);
        self.members_done_day[member_id] =
            self.day + self.ans_task_member_times[task_id][member_id];
    }
    #[allow(dead_code)]
    pub fn get_score(&self) -> i64 {
        self.n as i64 + 2000 - self.day
    }
}

fn calc_task_norm(task: &Task) -> i64 {
    let mut res = 0;
    for v in &task.req_skills {
        res += v * v;
    }
    sqrtf(res as f32) as i64
}

fn calc_tasks_importance_citical(tasks: &Vec<Task>) -> Vec<usize> {
    let n = tasks.len();
    let mut dists = vec![0; n];
    for i in (0..n).rev() {
        if dists[i] == 0 {
            dists[i] = 1;
        } else {
            dists[i] += tasks[i].norm;
        }
        for dep_i in &tasks[i].deps {
            dists[*dep_i] = max(dists[*dep_i], dists[i]);
        }
    }
    dists
}

/* ------------------- func -------------------*/

fn search_good_member_for_task(
    state: &State,
    task_id: usize,
    used_members: &HashSet<usize>,
) -> Option<usize> {
    let norm_thre = 30;
    let task = &state.tasks[task_id];
    let force_assign = task.norm > norm_thre;

    let mut best_mid_time = (None, 10000);
    let thre = if state.done_cnt < 700 { 3 } else { 1 };

    for mid in 0..state.m {
        let member = &state.members[mid];
        if !force_assign
            && (member.reserved_tasks.len() >= thre || used_members.contains(&member.id))
        {
            // eprintln!("m:{}, len:{}", member.id, member.reserved_tasks.len());
            continue;
        }
        let waiting_day = if member.is_working() {
            match member.working {
                Working::OnWork(_, est_end_day) => {
                    if state.day < est_end_day {
                        est_end_day - state.day
                    } else {
                        3
                    }
                }
                _ => panic!(),
            }
        } else {
            0
        };
        let mut waiting_day_reserved = 0;
        for tid in &member.reserved_tasks {
            waiting_day_reserved += state.tasks[*tid].reserved_est_time.unwrap();
        }
        let est_task_day = calc_est_time(state, mid, task_id) as usize;
        let time_sum = waiting_day + est_task_day + waiting_day_reserved;
        if time_sum < best_mid_time.1 {
            best_mid_time = (Some(mid), time_sum);
        }
    }
    best_mid_time.0
}

fn search_task_reservable(state: &State, r_cnt_max: usize, use_anytime_ok: bool) -> Vec<Assign> {
    let mut assigns = vec![];

    let mut used_tasks = HashSet::<usize>::new();
    let mut used_members = HashSet::<usize>::new();

    let orders = if use_anytime_ok {
        &state.tasks_order
    } else {
        &state.important_tasks_order
    };

    let mut r_cnt = 0;
    // for (task_id_ref, _, _) in &state.important_tasks_order {
    for (task_id_ref, _, _) in orders {
        let task_id = *task_id_ref;
        let task = &state.tasks[task_id];
        if !task.is_available() || used_tasks.contains(&task_id) {
            continue;
        }
        let best_mid = search_good_member_for_task(state, task_id, &used_members);
        if let Some(mid) = best_mid {
            let member = &state.members[mid];
            if member.is_working() || used_members.contains(&member.id) {
                assigns.push(Assign {
                    member_id: mid,
                    task_id,
                    reserve: true,
                });
                r_cnt += 1;
                used_members.insert(mid);
            } else {
                assigns.push(Assign {
                    member_id: mid,
                    task_id,
                    reserve: false,
                });
                used_members.insert(mid);
            }
            used_tasks.insert(task_id);
        }
        if r_cnt >= r_cnt_max {
            break;
        }
    }
    return assigns;
}

fn easiest_task_assign(state: &State, _myrng: &mut mytool::MyRng, _use_cos: bool) -> Vec<Assign> {
    let mut assigns = vec![];
    let mut used_tasks = HashSet::<usize>::new();
    let mut used_members = HashSet::<usize>::new();

    let mut free_members = vec![];
    for member in &state.members {
        if member.working == Working::Free {
            free_members.push(member);
        }
    }

    let mut task_high_prior = vec![];
    let mut task_low_prior = vec![];
    for task_id in 0..state.n {
        if state.tasks[task_id].is_available() {
            if state.tasks[task_id].importance <= 1 {
                task_low_prior.push(task_id);
            } else {
                task_high_prior.push(task_id);
            }
        }
    }
    let task_ids = [task_high_prior, task_low_prior].concat();

    for _ in 0..free_members.len() {
        let mut easiest_mi_ti_time = (None, 100000.0);
        for member in &free_members {
            if used_members.contains(&member.id) {
                continue;
            }
            let mut check_task_cnt = 0;
            // for task_id in 0..state.n {
            for task_id_ref in &task_ids {
                let task_id = *task_id_ref;
                if state.tasks[task_id].is_available() && !used_tasks.contains(&task_id) {
                    check_task_cnt += 1;
                    let est_time = calc_est_time(state, member.id, task_id) as f64;
                    if est_time < easiest_mi_ti_time.1 {
                        easiest_mi_ti_time = (
                            Some(Assign {
                                member_id: member.id,
                                task_id,
                                reserve: false,
                            }),
                            est_time,
                        );
                    }
                }
                // if check_task_cnt >= 1 && myrng.get_percent() < 0.2 {
                //     break;
                // }
                if check_task_cnt >= 5 {
                    break;
                };
            }
        }
        if let Some(assign) = easiest_mi_ti_time.0 {
            assert!(!used_members.contains(&assign.member_id));
            assigns.push(assign);
            used_members.insert(assign.member_id);
            used_tasks.insert(assign.task_id);
        }
    }

    return assigns;
}

fn important_task_assign(state: &State, myrng: &mut mytool::MyRng) -> Vec<Assign> {
    let mut assigns = vec![];
    let mut used_tasks = HashSet::<usize>::new();
    let mut used_members = HashSet::<usize>::new();

    let mut free_members = vec![];
    for member in &state.members {
        if member.working == Working::Free {
            free_members.push(member);
        }
    }

    for _ in 0..free_members.len() {
        let mut easiest_mi_ti_time = (None, 100000);
        let mut curr_imp = 1;
        for member in &free_members {
            if used_members.contains(&member.id) {
                continue;
            }
            let mut check_task_cnt = 0;
            let mut flag = false;
            // for task_id in 0..state.n {
            for (task_id_ref, importance, _norm) in &state.tasks_order {
                let task_id = *task_id_ref;
                if task_id / 3 > state.day && state.day > 50 {
                    continue;
                }
                if state.tasks[task_id].importance > 1
                    && state.tasks[task_id].progress == Progress::Pending
                    && !used_tasks.contains(&task_id)
                {
                    flag = true;
                }
                if state.tasks[task_id].is_available() && !used_tasks.contains(&task_id) {
                    if check_task_cnt == 0
                        && state.tasks[task_id].importance == 1
                        && flag
                        && state.done_cnt + state.anytime_tasks_cnt < 990
                    {
                        break;
                    }
                    if *importance == 1 && easiest_mi_ti_time.0 != None && curr_imp != 1 {
                        break;
                    }
                    check_task_cnt += 1;
                    let est_time = calc_est_time(state, member.id, task_id);
                    // let est_time = (calc_cos_sim(state, member.id, task_id) * 100.0) as i64;
                    if est_time < easiest_mi_ti_time.1 {
                        easiest_mi_ti_time = (
                            Some(Assign {
                                member_id: member.id,
                                task_id,
                                reserve: false,
                            }),
                            est_time,
                        );
                    }
                }
                curr_imp = *importance;
                let per = myrng.get_percent();
                if check_task_cnt >= 1 && (per < 0.5) {
                    break;
                };
            }
        }
        if let Some(assign) = easiest_mi_ti_time.0 {
            assert!(!used_members.contains(&assign.member_id));
            assigns.push(assign);
            used_members.insert(assign.member_id);
            used_tasks.insert(assign.task_id);
        }
    }

    return assigns;
}

fn important_task_assign_no_anytime(state: &State, _myrng: &mut mytool::MyRng) -> Vec<Assign> {
    let mut assigns = vec![];
    let mut used_tasks = HashSet::<usize>::new();
    let mut used_members = HashSet::<usize>::new();

    let mut free_members = vec![];
    for member in &state.members {
        if member.working == Working::Free {
            free_members.push(member);
        }
    }

    for _ in 0..free_members.len() {
        let mut easiest_mi_ti_time = (None, 100000);
        for member in &free_members {
            if used_members.contains(&member.id) {
                continue;
            }
            for (task_id_ref, _importance, _norm) in &state.anytime_tasks_order {
                let task_id = *task_id_ref;
                if state.tasks[task_id].is_available()
                    && !used_tasks.contains(&task_id)
                    && state.tasks[task_id].reserved_by == None
                {
                    let est_time = calc_est_time(state, member.id, task_id);
                    if est_time < easiest_mi_ti_time.1 {
                        easiest_mi_ti_time = (
                            Some(Assign {
                                member_id: member.id,
                                task_id,
                                reserve: false,
                            }),
                            est_time,
                        );
                    }
                }
                // let per = myrng.get_percent();
                // if check_task_cnt >= 1 && (per < 0.5) {
                //     break;
                // };
            }
        }
        if let Some(assign) = easiest_mi_ti_time.0 {
            assert!(!used_members.contains(&assign.member_id));
            assigns.push(assign);
            used_members.insert(assign.member_id);
            used_tasks.insert(assign.task_id);
        }
    }
    return assigns;
}

fn good_task_assign(state: &State, _myrng: &mut mytool::MyRng) -> Vec<Assign> {
    let mut assigns = vec![];
    let mut used_tasks = HashSet::<usize>::new();
    let mut used_members = HashSet::<usize>::new();

    let mut free_members = vec![];
    for member in &state.members {
        if member.working == Working::Free {
            free_members.push(member);
        }
    }

    for member in &free_members {
        let mut best_task_id_order = (None, 100000);
        let mut curr_imp = 1;
        let mut check_task_cnt = 0;

        for (task_id_ref, importance, _norm) in &state.tasks_order {
            let task_id = *task_id_ref;
            if state.tasks[task_id].is_available() && !used_tasks.contains(&task_id) {
                let my_est_time = calc_est_time(state, member.id, task_id);
                let mut order = 0;
                for other_m_i in 0..state.m {
                    if other_m_i == member.id {
                        continue;
                    }
                    let est_time = calc_est_time(state, other_m_i, task_id);
                    if est_time < my_est_time {
                        order += 1;
                    }
                }
                if *importance == 1 && best_task_id_order.0 != None && curr_imp != 1 {
                    break;
                }
                if order < best_task_id_order.1 {
                    best_task_id_order = (
                        Some(Assign {
                            member_id: member.id,
                            task_id,
                            reserve: false,
                        }),
                        order,
                    );
                }
                check_task_cnt += 1;
                if *importance != 1 && check_task_cnt >= max(2, 20 - (state.done_cnt as i64) / 30) {
                    // if check_task_cnt >= max(2, 5 - (state.done_cnt as i64) / 30) {
                    break;
                };
                if *importance == 1 && check_task_cnt >= 20 {
                    // if check_task_cnt >= max(2, 5 - (state.done_cnt as i64) / 30) {
                    break;
                };
                curr_imp = *importance;
            }
        }
        if let Some(assign) = best_task_id_order.0 {
            assert!(!used_members.contains(&assign.member_id));
            assigns.push(assign);
            used_members.insert(assign.member_id);
            used_tasks.insert(assign.task_id);
        } else {
        }
    }
    return assigns;
}

fn calc_est_time(state: &State, member_id: usize, task_id: usize) -> i64 {
    let mut res = 0;
    let task = &state.tasks[task_id];
    let member = &state.members[member_id];
    for skill_id in 0..state.k {
        res += max(0, task.req_skills[skill_id] - member.skills[skill_id]);
    }
    return res;
}

fn calc_new_est_time_diff(req_skill: i64, skill: i64, d: i64) -> i64 {
    let update_d = max(0, req_skill - (skill + d)) - max(0, req_skill - skill);
    update_d
}

fn yaki_prob(top: usize, bottom: usize, ther0: f64, ther1: f64, kaizen: i64) -> f64 {
    let time01 = (top as f64) / (bottom as f64);
    let mut tt = pow(ther0, 1.0 - time01) * pow(ther1, time01);
    tt *= kaizen as f64;
    tt.exp()
}

fn update_skill(state: &mut State, member_id: usize, myrng: &mut mytool::MyRng) {
    let d_list: Vec<i64> = vec![-1, 1];
    for i in 0..UPDATE_SKILL_LOOP_CNT {
        let rand_i = myrng.get_int(0, (state.k - 1) as i64) as usize;
        let d = myrng.choose(&d_list);
        let new_v_cand = state.members[member_id].skills[rand_i] + d;
        if new_v_cand < 0 || new_v_cand > 30 {
            continue;
        }
        let mut kaizen = 0;
        let mut new_vs = vec![];
        for task_res in &state.members[member_id].task_results {
            let task = &state.tasks[task_res.task_id];
            let orig_v = task_res.est_time;
            let update_d = calc_new_est_time_diff(
                task.req_skills[rand_i],
                state.members[member_id].skills[rand_i],
                d,
            );
            let new_v = orig_v + update_d;
            new_vs.push(new_v);

            // kaizen += abs(task_res.time - orig_v) - abs(task_res.time - new_v);
            let x = task_res.time - orig_v;
            let y = task_res.time - new_v;
            kaizen += x * x - y * y;
        }
        // if kaizen >= -1 {
        if yaki_prob(
            i,
            UPDATE_SKILL_LOOP_CNT,
            (state.members[member_id].task_results.len() as f64) * 5.0,
            0.1,
            kaizen,
        ) > myrng.get_percent()
        {
            state.members[member_id].skills[rand_i] += d;
            for (new_v, task_res) in izip!(&new_vs, &mut state.members[member_id].task_results) {
                task_res.est_time = *new_v;
            }
        }
    }
}

/* ------------------------------------------------------------------------
    solve!
------------------------------------------------------------------------ */
fn solve() {
    /* ------------ ready ------------ */
    // let mut tm = mytool::TimeManager::new();
    let mut myrng = mytool::MyRng::new();
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));

    let mut state = State::new(&mut stdin);
    let mut judge = Judge::new(&mut stdin, state.n, state.m, state.k);

    /* ------------ main loop ------------ */
    loop {
        let mut t_cnt = 0;
        let mut m_cnt = 0;
        let mut no_child_cnt = 0;
        let mut reserve_cnt = 0;
        for task in &state.tasks {
            if task.is_available() {
                t_cnt += 1;
                if task.importance == 1 {
                    no_child_cnt += 1;
                }
            }
        }
        for mem in &state.members {
            if mem.is_available() {
                m_cnt += 1;
            }
            reserve_cnt += mem.reserved_tasks.len();
        }
        // eprintln!(
        //     "day: {:>4}, done: {:>4}, t_ava: {}, no_c: {} ,m_ava: {}, thetask: {:?}, m9q: {:?}",
        //     state.day,
        //     state.done_cnt,
        //     t_cnt - no_child_cnt,
        //     no_child_cnt,
        //     m_cnt,
        //     state.tasks[903].progress,
        //     state.members[7].reserved_tasks
        // );

        let mut reserved_task_assigns = vec![];
        for mid in 0..state.m {
            if state.members[mid].is_available() && !state.members[mid].reserved_tasks.is_empty() {
                let task_id = *state.members[mid].reserved_tasks.front().unwrap();
                reserved_task_assigns.push(Assign {
                    member_id: mid,
                    task_id,
                    reserve: false,
                });
                // m_cnt -= 1;
                // t_cnt -= 1;
                state.member_starts_reserved_task(mid);
                #[cfg(feature = "dbg")]
                judge.member_starts_task(mid, task_id);
            }
        }

        /* ----- decide next member-task pairs ----- */
        let child_cnt = t_cnt - no_child_cnt;

        let assigns_merged = if t_cnt >= 15 && state.r < 2000 {
            good_task_assign(&state, &mut myrng)
        } else {
            if state.done_cnt > 300 && reserve_cnt < 40 && state.day < 800 {
                search_task_reservable(&state, 20, state.r < 2000)
            } else {
                if state.done_cnt > 980 {
                    easiest_task_assign(&state, &mut myrng, false)
                } else {
                    important_task_assign(&state, &mut myrng)
                }
            }
        };

        let mut assigns = vec![];
        let mut reserved_assigns = vec![];
        for assign in &assigns_merged {
            if assign.reserve {
                reserved_assigns.push(assign.clone());
            } else {
                assigns.push(assign.clone());
            }
        }

        /* ----- update state & output by assigns info ----- */
        for assign in &assigns {
            state.member_starts_task(assign.member_id, assign.task_id);
            #[cfg(feature = "dbg")]
            judge.member_starts_task(assign.member_id, assign.task_id);
        }

        for assign in &reserved_assigns {
            state.reserve_task(assign.member_id, assign.task_id);
        }

        if child_cnt <= assigns.len() && no_child_cnt > 0 && m_cnt as i64 - assigns.len() as i64 > 5
        {
            // eprintln!("{}", state.day);
            let cnt = m_cnt - assigns.len() - 5;
            let anytime_assigins = important_task_assign_no_anytime(&state, &mut myrng);
            let mut lcnt = 0;
            for assign in &anytime_assigins {
                if lcnt >= cnt {
                    break;
                }
                lcnt += 1;
                assigns.push(*assign);
                state.member_starts_task(assign.member_id, assign.task_id);
                #[cfg(feature = "dbg")]
                judge.member_starts_task(assign.member_id, assign.task_id);
            }
        }

        let out = [assigns.clone(), reserved_task_assigns.clone()].concat();
        output(&out);

        // output(&assigns);

        /* ----- advance day (today's night?) ----- */
        state.advance_day();
        #[cfg(feature = "dbg")]
        judge.advance_day();

        /* ----- get done members ----- */
        let done_cnt = get_done_cnt(&mut stdin, &mut judge);
        if done_cnt == -1 {
            break;
        }
        let done_members = get_done_members(&mut stdin, &mut judge, done_cnt as usize);

        /* ----- update status for done members ----- */
        for member_id in &done_members {
            // eprintln!(
            //     "[done] m: {}, t: {:?}",
            //     member_id, state.members[*member_id].working
            // );
            state.complete_task(*member_id as usize);
            update_skill(&mut state, *member_id, &mut myrng);
            #[cfg(feature = "dbg")]
            output_skill_debug(&state, *member_id);
        }
    }

    #[cfg(feature = "dbg")]
    eprintln!("score: {}", judge.get_score());
}

/* ------------------------------------------------------------------------
    need not change below
------------------------------------------------------------------------ */
fn main() {
    solve();
}

fn output(assigns: &Vec<Assign>) {
    print!("{} ", assigns.len());
    for ass in assigns {
        print!("{} {} ", ass.member_id + 1, ass.task_id + 1);
    }
    println!();
    std::io::stdout().flush().unwrap();
}

#[allow(dead_code)]
fn output_skill_debug(state: &State, member_id: usize) {
    print!("#s {} ", member_id + 1);
    for j in 0..state.k {
        print!("{} ", state.members[member_id].skills[j]);
    }
    println!();
    std::io::stdout().flush().unwrap();
}

fn get_done_cnt(stdin: &mut LineSource<BufReader<Stdin>>, judge: &mut Judge) -> i64 {
    if cfg!(feature = "dbg") {
        judge.get_done_cnt()
    } else {
        input! ( from stdin, done_cnt_in: i64 );
        done_cnt_in
    }
}

fn get_done_members(
    stdin: &mut LineSource<BufReader<Stdin>>,
    judge: &mut Judge,
    done_cnt: usize,
) -> Vec<usize> {
    if cfg!(feature = "dbg") {
        judge.get_done_members()
    } else {
        input! ( from stdin, done_members_in: [Usize1; done_cnt] );
        done_members_in
    }
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
