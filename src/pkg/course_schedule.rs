#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    NotVisited,
    UnderProcessing,
    Processed,
}

#[allow(dead_code)]
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut edges = vec![vec![]; num_courses as usize];
    let mut state = vec![State::NotVisited; num_courses as usize];

    for p in prerequisites {
        edges[p[0] as usize].push(p[1] as usize);
    }

    for i in 0..num_courses as usize {
        if state[i] == State::Processed {
            continue;
        }
        if !dfs(i, &edges, &mut state) {
            return false;
        }
    }

    true
}

pub fn dfs(course: usize, edges: &Vec<Vec<usize>>, state: &mut Vec<State>) -> bool {
    if state[course] == State::UnderProcessing {
        return false;
    }
    if state[course] == State::Processed {
        return true;
    }
    state[course] = State::UnderProcessing;
    for &prereq in &edges[course] {
        if !dfs(prereq, edges, state) {
            return false;
        }
    }
    state[course] = State::Processed;

    return true;
}