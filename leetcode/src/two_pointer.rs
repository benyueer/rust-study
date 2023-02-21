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
            if nums[i-1] < nums[i] {
                let mut j = i;
                while j < len && nums[i-1] < nums[j] {
                    j+=1;
                }
                nums.swap(i-1, j-1);
                break;
            }
            i-=1;
        }

        nums[i..].reverse();
    }

    #[test]
    fn test_next_permutation() {
        // let mut nums: Vec<i32> = vec!(3, 2, 1);
        let mut nums: Vec<i32> = vec!(1, 2, 3);
        next_permutation(&mut nums);
        println!("{:?}", nums);
    }

}