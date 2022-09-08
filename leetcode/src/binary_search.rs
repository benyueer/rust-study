/**
 * 二分
 * 有序皆可二分
 * * 开闭原则
 * * 区间的变化
 * 
 * * 一般方法：
 * 确定搜索空间
 * 确定结果类型
 * 根据结果是否满足确定求值为0 -1 1
 * 根据最优解的0的位置确定lower或upper
 */
#[cfg(test)]
mod test {
    /**
     * 有序数组中最左边的元素
     * 给定一个游戏数组，请返回指定元素的最左边位置
     * 输入：A=[1, 2, 2, 2, 3, 3], target = 2
     * 输出：1
     */
    fn first_ind(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] >= target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l as i32
    }

    #[test]
    fn test_first_ind() {
        let nums = vec![1, 2, 2, 2, 3, 3];
        let target = 2;
        let res = first_ind(nums, target);
        println!("{res}");
    }

    /**
     * 实现upperBound函数
     * 返回第一个大于给定值的元素的位置
     */
    fn upper_bound(nums: Vec<i32>, target: i32) -> usize {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] <= target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        l
    }

    #[test]
    fn test_upper_bound() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 5;
        let res = upper_bound(nums, target);
        println!("{res}");
    }

    /**
     * 34-在排序数组中查找元素的第一个和最后一个位置
     */
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];

        fn binary_search(arr: &Vec<i32>, target: i32, lower: bool) -> i32 {
            let mut l = 0;
            let mut r = arr.len() - 1;
            let mut ans = match lower {
                true => 0,
                false => arr.len() - 1,
            };

            while l>=0 && l <= r {
                let mid = l + (r- l) / 2;
                if arr[mid] > target || (lower && arr[mid] >= target) {
                    if mid == 0 {
                        match lower {
                            true => ans = mid,
                            false => (),
                        }
                        break;
                    }
                    r = mid - 1;
                    ans = match lower {
                        true => mid,
                        false => mid - 1,
                    }
                } else {
                    l = mid + 1;
                }
            }

            ans as i32
        }

        let start = binary_search(&nums, target, true);
        let end = binary_search(&nums, target, false);

        println!("{start} {end}");

        if start <= end && end < nums.len().try_into().unwrap() && nums[start as usize] == target && nums[end as usize] == target {
            res = vec![start, end];
        }
        res
    }

    #[test]
    fn test_search_range() {
        let nums = vec![1, 2, 3];
        let target = 1;
        let res = search_range(nums, target);
        println!("{:?}", res);
    }

    /**
     * 35-搜索插入位置
     */
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        0
    }

    /**
     * 852-山脉数组的峰顶索引
     */
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 1;
        let mut r = arr.len() - 2;

        while l < r {
            let mid = l + (r - l) / 2;
            if arr[mid] > arr[mid-1] && arr[mid] < arr[mid + 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        l as i32
    }

    #[test]
    fn test_peak_index_in_mountain_array() {
        let arr = vec![3,4,5,1];
        let res = peak_index_in_mountain_array(arr);
        println!("{res}");
    }

    /**
     * 一个正整数数组A，整数s，找出最小长度的连续子数组，使得子数组和>=s
     */
    fn shortest_arr(arr: Vec<i32>, s: i32) -> i32 {
        let mut res = 1;



        res
    }

    /**
     * 76-最小覆盖子串
     */
    pub fn min_window(s: String, t: String) -> String {
        "a".to_string()
    }

    /**
     * 643-子数组最大平均数 I
     */
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        0.0
    }

    /**
     * 33-搜索旋转排序数组
     */
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        0
    }

    /**
     * 4-寻找两个正序数组的中位数
     */
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        0.0
    }
}
