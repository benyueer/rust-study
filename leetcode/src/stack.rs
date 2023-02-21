/**
 * 栈
 * 单调栈
 */

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
            while !stack.is_empty()
                && stack.last().unwrap().to_owned() > nums[i]
                && stack.len() + nums.len() - i > k
            {
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

    /**
     * 84
     */
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
        let heights = vec![1, 3, 2, 1];
        let res = largest_rectangle_area(heights);

        println!("{:?}", res);
    }

    /**
     * 739
     */
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = vec![];

        for i in 0..temperatures.len() {
            while !stack.is_empty()
                && temperatures[stack.last().unwrap().to_owned()] < temperatures[i]
            {
                res[stack.last().unwrap().to_owned()] =
                    (i - stack.last().unwrap().to_owned()) as i32;
                stack.pop();
            }
            stack.push(i);
        }

        res
    }

    #[test]
    fn test_daily_temperatures() {
        let temperatures = vec![30, 40, 50, 60];
        let res = daily_temperatures(temperatures);
        println!("{:?}", res);
    }

    /**
     * 42. 接雨水
     */
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut stack: Vec<usize> = vec![];

        for i in 0..height.len() {
            while !stack.is_empty() && height[i] > height[stack.last().unwrap().to_owned()] {
                let cur = stack.pop().unwrap().to_owned();
                if stack.is_empty() {
                    break;
                }
                let dis = i - stack.last().unwrap().to_owned() - 1;
                let h = height[i].min(height[stack.last().unwrap().to_owned()]) - height[cur];
                res += h * dis as i32;
            }
            stack.push(i);
        }

        res
    }

    #[test]
    fn test_trap() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let res = trap(height);
        println!("{res}");
    }

    /**
     * 901
     */
    struct StockSpanner {
        prices: Vec<i32>,
        ind: Vec<i32>,
    }

    impl StockSpanner {
        fn new() -> Self {
            StockSpanner {
                prices: vec![],
                ind: vec![],
            }
        }

        fn next(&mut self, price: i32) -> i32 {
            let mut res = 1;
            while !self.prices.is_empty() && price >= self.prices.last().unwrap().to_owned() {
                self.prices.pop();
                res += self.ind.pop().unwrap().to_owned();
            }
            self.prices.push(price);
            self.ind.push(res);

            println!("{:?}", self.prices);
            res
        }
    }

    #[test]
    fn test_stock_spanner() {
        let mut s = StockSpanner::new();
        assert_eq!(s.next(100), 1);
        assert_eq!(s.next(80), 1);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(70), 2);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(75), 4);
        assert_eq!(s.next(85), 6);
    }

    /**
     * 402
     */
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<usize> = vec![];
        let mut nums: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();

        for i in 0..nums.len() {
            while !stack.is_empty() && nums[i] < nums[stack.last().unwrap().to_owned()] && k > 0 {
                stack.pop();
                k -= 1;
            }
            stack.push(i);
        }

        while k > 0 {
            stack.pop();
            k -= 1;
        }

        let mut res = String::new();
        let mut flag = false;
        for i in 0..stack.len() {
            if nums[stack[i]] == 0 && !flag {
                continue;
            }
            flag = true;
            res.push_str(&nums[stack[i]].to_string());
        }
        if res == "" {
            res = "0".to_string();
        }
        res
    }

    #[test]
    fn test_remove_kdigits() {
        //                 0123456
        let num = "10".to_string();
        let k = 2;
        let res = remove_kdigits(num, k);
        println!("{res}");
    }

    /**
     * 581
     */
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut l = 0;
        let mut r: i32 = -1;

        // [2,6,4,8,10,9,15]

        for i in 0..nums.len() {
            if nums[i] <= max {
                r = i as i32;
            } else {
                max = nums[i];
            }

            if nums[nums.len() - 1 - i] >= min {
                l = (nums.len() - 1 - i) as i32;
            } else {
                min = nums[nums.len() - 1 - i];
            }
        }

        r - l + 1
    }

    #[test]
    fn test_find_unsorted_subarray() {
        let nums = vec![1, 2, 3, 4];
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        let res = find_unsorted_subarray(nums);
        println!("{res}");
    }

    /**
     * 316
     */
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut vis = [false; 26];
        let mut stack: Vec<u8> = vec![];

        for (i, c) in s.bytes().enumerate() {
            if !vis[c as usize - 97] {
                let t = s[i..].as_bytes();
                while !stack.is_empty()
                    && stack[stack.len() - 1] > c
                    && t.contains(&stack[stack.len() - 1])
                {
                    vis[stack.pop().unwrap() as usize - 97] = false;
                }
                stack.push(c);
                vis[c as usize - 97] = true;
            }
        }

        String::from_utf8(stack).unwrap()
    }

    #[test]
    fn test_remove_duplicate_letters() {
        let s = String::from("dabcd");
        let res = remove_duplicate_letters(s);
        println!("{res}");
    }

    
}
