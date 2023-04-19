/**
 * 双指针
 * 通常是定义两个指针在数组/链表上遍历，解决区间问题
 * 最长区间 定长区间 最短区间
 * 单调性
 */

#[cfg(test)]
mod test {
    /**
     * 31 next_permutation
     */
    fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();

        let mut i = len - 1;

        while i > 0 {
            if nums[i - 1] < nums[i] {
                let mut j = i;
                while j < len && nums[i - 1] < nums[j] {
                    j += 1;
                }
                nums.swap(i - 1, j - 1);
                break;
            }
            i -= 1;
        }

        nums[i..].reverse();
    }

    #[test]
    fn test_next_permutation() {
        // let mut nums: Vec<i32> = vec!(3, 2, 1);
        let mut nums: Vec<i32> = vec![1, 2, 3];
        next_permutation(&mut nums);
        println!("{:?}", nums);
    }

    /**
     * 16. 最接近的三数之和
     */
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = f64::MAX;

        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() - 2 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                let ans = (nums[i] + nums[l] + nums[r]) as f64;
                if ans < target as f64 {
                    l += 1;
                } else if ans > target as f64 {
                    r -= 1;
                } else {
                    return target;
                }

                if (ans - target as f64).abs() < (res - target as f64).abs() {
                    res = ans;
                }
            }
        }

        res as i32
    }
}
