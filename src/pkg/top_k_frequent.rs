use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count = HashMap::new();
    for n in nums {
        count.insert(n, count.get(&n).unwrap_or(&0) + 1);
    }
    let mut heap = BinaryHeap::new();
    for c in count {
        heap.push(Count {
            count: c.1,
            key: c.0,
        });
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_iter().map(|x| x.key).collect()
}

#[derive(Eq, PartialEq)]
pub struct Count {
    count: usize,
    key: i32,
}

impl Ord for Count {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for Count {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}