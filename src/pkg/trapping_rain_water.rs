#[allow(dead_code)]
pub fn trap(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, height.len() - 1);
    let (mut lmax, mut rmax) = (height[l], height[r]);

    let mut answer = 0;

    while l < r {
        lmax = i32::max(lmax, height[l]);
        rmax = i32::max(rmax, height[r]);
        if lmax < rmax {
            answer += lmax - height[l];
            l += 1;
        } else {
            answer += rmax - height[r];
            r -= 1;
        }
    }

    answer
}