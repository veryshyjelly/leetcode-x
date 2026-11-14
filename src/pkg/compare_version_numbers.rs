#[allow(dead_code)]
pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut version1: Vec<usize> = version1.split(".").map(|x| x.parse().unwrap()).collect();
    let mut version2: Vec<usize> = version2.split(".").map(|x| x.parse().unwrap()).collect();
    let (n, m) = (version1.len(), version2.len());

    for i in n..m {
        version1.push(0);
    }
    for i in m..n {
        version2.push(0);
    }

    for i in 0..usize::max(n, m) {
        if version1[i] < version2[i] {
            return -1;
        } else if version1[i] > version2[i] {
            return 1;
        }
    }

    0
}