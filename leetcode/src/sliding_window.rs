#[cfg(test)]
mod test {
    /**
     * 1004. 最大连续1的个数 III
     */
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r, mut lcount, mut rcount, mut res ) = (0, 0, 0, 0, 0);
        

        for i in 0..nums.len() {
            rcount += 1-nums[i];
            if lcount < rcount - k {
                lcount += 1-nums[l];
                l+=1;
            }

            res = res.max(i as i32 - l as i32 +1);
        }

        res as i32
    }

    #[test]
    fn test_longest_ones() {
        let nums = vec![0,0,0,0];
        let k = 0;
        let res = longest_ones(nums, k);
        println!("{res}");
    }


    /**
     * 209. 长度最小的子数组
     */
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;

        let (mut l, mut r, mut sum) = (0, 0, 0);

        for i in 0..nums.len() {
            sum += nums[i];
            r = i;

            while sum >= target {
                res = res.min(r as i32-l as i32+1);
                sum -= nums[l as usize];
                l += 1;
            }
        }

        if res == i32::MAX {
            return 0;
        }
        res
    }

    /**
     * 219. 存在重复元素 II
     */
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::<i32, usize>::new();
        let (mut l, mut r) = (0, 0);

        for i in 0..nums.len() {
            r = i;
            while r - l > k as usize {
                let base = *map.entry(nums[l]).or_insert(0);
                if base > 0 {
                    *map.entry(nums[l]).or_insert(0) -= 1;
                }
                l += 1;
            }
            if l != r && map.entry(nums[r]).or_insert(0) > &mut 0 {
                return true;
            }
            *map.entry(nums[i]).or_insert(0) += 1;
        }

        false
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        let nums = vec![1,2,3,1,2,3];
        let k = 2;
        let res = contains_nearby_duplicate(nums, k);
        println!("{res}");
    }


    /**
     * 220. 存在重复元素 III
     */
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let mut l = 0;
        


        false
    }

}