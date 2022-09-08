#[cfg(test)]
mod test {
    /**
     * 右边第一个小元素
     */
    fn get_right_small(nums: Vec<i32>) -> Vec<usize> {
        let mut res = vec![usize::MAX; nums.len()];
        let mut stack: Vec<usize> = vec![];

        for i in 0..nums.len() {
            while !stack.is_empty() && nums[stack.last().unwrap().to_owned()] > nums[i] {
                res[stack.last().unwrap().to_owned()] = i;
                stack.pop();
            }
            stack.push(i);
        }

        while !stack.is_empty() {
            res[stack.pop().unwrap()] = usize::MAX;
        }

        res
    }

    #[test]
    fn test_get_right_small() {
        let nums = vec![4, 1, 5, 2, 6];
        let res = get_right_small(nums);
        println!("{:?}", res);
    }


    /**
     * 取出字典序最小的k个数
     */
    fn small_k_numbers(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let mut stack: Vec<i32> = vec![];

        for i in 0..nums.len() {
            while !stack.is_empty() && stack.last().unwrap().to_owned() > nums[i] && stack.len() + nums.len() - i > k {
                stack.pop();
            }
            stack.push(nums[i]);
        }

        stack.get(0..k).unwrap().to_vec()
    }

    #[test] 
    fn test_small_k_numbers() {
        let nums = vec![3, 4, 1, 2, 7, 0];
        let res = small_k_numbers(nums, 3);
        println!("{:?}", res);
    }


    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut stack: Vec<usize> = vec![];
        let mut heights = heights;
        heights.insert(0, 0);
        heights.push(0);
        println!("{:?}", heights);

        for i in 0..heights.len() {
            while !stack.is_empty() && heights[stack.last().unwrap().to_owned()] > heights[i] {
                let cur = stack.pop().unwrap().to_owned();
                let left = stack.last().unwrap().to_owned() as i32;
                let right = i as i32;
                res = res.max((right - left - 1) * heights[cur]);
            }
            stack.push(i);
        }

        res
    }

    #[test]
    fn test_largest_rectangle_area() {
        let heights = vec!{2, 1, 2};
        let res = largest_rectangle_area(heights);

        println!("{:?}", res);
    }
}
