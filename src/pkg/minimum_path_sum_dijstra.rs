#[allow(dead_code)]
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dist: Vec<Vec<_>> = vec![vec![i32::MAX; n]; m];
    let mut heap = BinaryHeap::new();
    // let start = (0, 0);
    dist[0][0] = grid[0][0];
    heap.push(State {
        cost: grid[0][0],
        position: (0, 0),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == (m - 1, n - 1) {
            return cost;
        }
        if cost > dist[position.0][position.1] {
            continue;
        }
        if position.0 < m - 1 {
            let next = State {
                cost: cost + grid[position.0 + 1][position.1],
                position: (position.0 + 1, position.1),
            };
            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
        if position.1 < n - 1 {
            let next = State {
                cost: cost + grid[position.0][position.1 + 1],
                position: (position.0, position.1 + 1),
            };
            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    0
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: i32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}