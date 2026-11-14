#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
    let mut index = m + n - 1;
    m -= 1;
    n -= 1;

    while m >= 0 && n >= 0 {
        if nums1[m as usize] > nums2[n as usize] {
            nums1[index as usize] = nums1[m as usize];
            m -= 1;
        } else {
            nums1[index as usize] = nums2[n as usize];
            n -= 1;
        }
        index -= 1;
    }

    while n >= 0 {
        nums1[index as usize] = nums2[n as usize];
        n -= 1;
        index -= 1;
    }
}