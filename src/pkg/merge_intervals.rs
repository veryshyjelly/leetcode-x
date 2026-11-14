use std::cmp::Ordering;

#[allow(dead_code)]
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut int_intervals: Vec<_> = intervals.iter().map(|x| Interval::from_vec(x)).collect();
    int_intervals.sort();

    let (mut starting, mut ending) = (int_intervals[0].start, int_intervals[0].end);
    let mut res = vec![];

    for inter in int_intervals {
        if inter.start <= ending {
            ending = i32::max(ending, inter.end);
        } else {
            res.push(vec![starting, ending]);
            starting = inter.start;
            ending = inter.end;
        }
    }

    res
}

pub struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    pub fn from_vec(int: &Vec<i32>) -> Self {
        Interval {
            start: int[0],
            end: int[1],
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start)
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}