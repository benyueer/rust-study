mod array;
mod binary_search;
mod queue;
mod search_backtrack;
mod sliding_window;
mod stack;
mod tree;
mod two_pointer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    // 1. 两数之和
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<i32, usize>::new();

        for (i, n) in nums.into_iter().enumerate() {
            match map.get(&(target - n)) {
                Some(&s) => return vec![s as i32, i as i32],
                None => {}, 
            }

            map.insert(n, i);
        }

        vec![]
    }

    // [1694] 重新格式化电话号码
    pub fn reformat_number(number: String) -> String {
        let mut res = String::from("");
        let mut num_str = number.replace("-", "").replace(" ", "");

        let len = num_str.len();

        let mut cur_idx = 0;

        while cur_idx + 4 < len {
            res.push_str(&num_str[cur_idx..cur_idx + 3]);
            res.push('-');
            cur_idx += 3;
        }

        if len - cur_idx == 4 {
            res.push_str(&num_str[cur_idx..cur_idx + 2]);
            cur_idx += 2;
            res.push('-');
            res.push_str(&num_str[cur_idx..cur_idx + 2]);
        } else {
            res.push_str(&num_str[cur_idx..])
        }

        println!("{res}");
        res
    }

    #[test]
    fn test_reformat_number() {
        reformat_number(String::from("1234"));
    }

    /**
     * 3. 无重复字符的最长子串
     */
    fn length_of_longest_substring(s: String) -> i32 {
        let mut ret = 0;
        let mut l = 0;
        let mut cache = vec![0; 128];

        s.chars().enumerate().for_each(|(i, ch)| {
            l = l.max(cache[ch as usize]);
            ret = ret.max(i as i32 - l + 1);
            cache[ch as usize] = i as i32 + 1;
        });

        ret
    }

    /**
     * 206. 反转链表
     */
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = None;
        let mut head = head;

        while let Some(mut node) = head {
            head = node.next.take();
            node.next = p;
            p = Some(node);
        }
        p
    }

    /**
     * 146. LRU 缓存
     */
    mod LRUCache {
        #[derive(Debug, Clone, Copy)]
        struct LRUCacheEntry {
            key: i32,
            val: i32,
        }

        struct LRUCache {
            cache: Vec<LRUCacheEntry>,
            capacity: i32,
        }
        impl LRUCache {
            fn new(capacity: i32) -> Self {
                LRUCache {
                    cache: vec![],
                    capacity,
                }
            }

            fn get(&mut self, key: i32) -> i32 {
                let mut res = -1;
                let mut ind = 0;

                for i in &self.cache {
                    ind += 1;
                    if i.key == key {
                        res = i.val;
                        // self.reset(ind);
                        self.cache.remove(ind - 1);
                        self.cache.insert(0, LRUCacheEntry { key, val: res });
                        break;
                    }
                }

                res
            }

            fn put(&mut self, key: i32, value: i32) {
                let mut ind = 0;

                for i in &self.cache {
                    ind += 1;
                    if i.key == key {
                        // self.reset(ind);
                        self.cache.remove(ind - 1);
                        break;
                    }
                }
                self.cache.insert(0, LRUCacheEntry { key, val: value });
                if self.cache.len() > self.capacity.try_into().unwrap() {
                    self.cache.pop();
                }
            }
        }

        #[test]
        fn test_lru() {
            let mut lru = LRUCache::new(2);
            println!("{}", lru.get(1));
            lru.put(1, 2);
            lru.put(2, 4);
            lru.put(3, 6);
            println!("{}", lru.get(1));
            println!("{}", lru.get(2));
        }
    }

    /**
     * 53. 最大子数组和
     */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut pre = nums[0];

        for i in 1..nums.len() {
            pre = nums[i].max(pre + nums[i]);
            res = res.max(pre);
        }

        res
    }

    #[test]
    fn test_max_sub_array() {
        let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        // let nums = [5,4,-1,7,8];
        // let nums = [-2, 1];
        let res = max_sub_array(nums.to_vec());
        println!("{res}");
    }

    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }
    /**
     * 102. 二叉树的层序遍历
     */
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut queue = VecDeque::new();

        if let Some(h) = root {
            queue.push_back(h);
        }

        while queue.len() > 0 {
            let len = queue.len();
            let mut base = vec![];
            for _i in 0..len {
                let node = queue.pop_front().unwrap();
                base.push(node.borrow().val);
                if let Some(l) = node.borrow_mut().left.take() {
                    queue.push_back(l);
                }
                if let Some(r) = node.borrow_mut().right.take() {
                    queue.push_back(r);
                };
            }
            res.push(base);
        }

        res
    }

    /**
     * 121. 买卖股票的最佳时机
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }

        let mut res = 0;
        let mut cost = prices[0];

        for i in 1..prices.len() {
            res = res.max(prices[i] - cost);
            cost = cost.min(prices[i]);
        }

        res
    }

    #[test]
    fn test_max_profit() {
        // let prices = vec![7,1,5,3,6,4];
        let prices = vec![7, 6, 4, 3, 1];
        let res = max_profit(prices);
        println!("{res}");
    }

    /**
     * 200. 岛屿数量
     */
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    res += 1;
                    reset_island(&mut grid, i, j);
                }
            }
        }

        res
    }

    fn reset_island(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let dirs: Vec<[i32; 2]> = vec![[0, 1], [0, -1], [1, 0], [-1, 0]];
        let i_l = grid.len() as i32;
        let j_l = grid[0].len() as i32;

        if grid[i][j] == '1' {
            grid[i][j] = '0';

            for d in dirs.iter() {
                let n_i = i as i32 + d[0];
                let n_j = j as i32 + d[1];
                if n_i >= 0 && n_i < i_l && n_j >= 0 && n_j < j_l {
                    reset_island(grid, n_i as usize, n_j as usize);
                }
            }
        }
    }

    #[test]
    fn test_num_islands() {
        let mut grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        let mut grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        let res = num_islands(grid);

        println!("{res}");
    }

    /**
     * 5. 最长回文子串
     */
    pub fn longest_palindrome(s: String) -> String {
        let mut max_len = 0;
        let (mut b, mut e) = (0, 0);
        let len = s.len();
        let mut dp = vec![];
        for i in 0..len {
            dp.push(vec![false; len]);
            dp[i][i] = true;
        }

        for i in (0..len).rev() {
            for j in i + 1..len {
                if s[i..=i] == s[j..=j] {
                    if j - i <= 1 {
                        dp[i][j] = true;
                    } else if dp[i + 1][j - 1] {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = false;
                    }

                    if dp[i][j] {
                        if j - i >= e - b {
                            (b, e) = (i, j);
                        }
                    }
                } else {
                    dp[i][j] = false;
                }
            }
        }

        s[b..=e].to_owned()
    }

    #[test]
    fn test_longest_palindrome() {
        let mut s = "acbbc".to_string();

        let res = longest_palindrome(s);
        println!("{res}");
    }

    /**
     * 46. 全排列
     */
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;

        fn dfs(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, first: usize) {
            if first == nums.len() {
                res.push(nums.to_vec());
            }

            for i in first..nums.len() {
                nums.swap(i, first);
                dfs(res, nums, first + 1);
                nums.swap(i, first)
            }
        }

        dfs(&mut res, &mut nums, 0);

        res
    }

    #[test]
    fn test_permute() {
        let nums = [1, 2, 3].to_vec();
        let res = permute(nums);
        println!("{res:?}");
    }

    /**
     * 300. 最长递增子序列
     */
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![1; nums.len()];

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j] > nums[i] {
                    dp[j] = dp[j].max(dp[i] + 1);
                    res = res.max(dp[j]);
                }
            }
        }

        res
    }

    #[test]
    fn test_length_of_lis() {
        let nums = [7, 7, 7, 7, 7, 7, 7].to_vec();
        let res = length_of_lis(nums);
        println!("{res}");
    }

    /**
     * 56. 合并区间
     */
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut intervals = intervals;

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        for i in 1..intervals.len() {
            if intervals[i - 1][1] >= intervals[i][0] {
                if intervals[i - 1][1] >= intervals[i][1] {
                    intervals[i][0] = intervals[i - 1][0];
                    intervals[i][1] = intervals[i - 1][1];
                } else {
                    intervals[i][0] = intervals[i - 1][0];
                }
            } else {
                res.push(intervals[i - 1].clone());
            }
        }

        res.push(intervals[intervals.len() - 1].clone());

        res
    }

    /**
     * 31. 下一个排列
     */
    pub fn next_permutation(nums: &mut Vec<i32>) {
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

    /**
     * 1143. 最长公共子序列
     */
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len1 = text1.len();
        let len2 = text2.len();
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if text1[i - 1..=i - 1] == text2[j - 1..=j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        // println!("{dp:?}");

        dp[len1][len2]
    }

    #[test]
    fn test_longest_common_subsequence() {
        let t1 = "abcde".to_string();
        let t2 = "ace".to_string();

        let res = longest_common_subsequence(t1, t2);

        println!("{res}");
    }

    /**
     * 32. 最长有效括号
     */
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut dp = vec![0; s.len()];
        let mut res = 0;

        for i in 1..s.len() {
            if s[i] == ')' {
                if s[i - 1] == '(' {
                    dp[i] = 2;

                    if i - 2 >= 0 {
                        dp[i] += dp[i - 2];
                    }
                } else if dp[i - 1] > 0 {
                    if (i - dp[i - 1] - 1 >= 0) && s[i - dp[i - 1] - 1] == '(' {
                        dp[i] = dp[i - 1] + 2;
                        if i - dp[i - 1] - 2 >= 0 {
                            dp[i] += dp[i - dp[i - 1] - 2];
                        }
                    }
                }
            }
            res = res.max(dp[i]);
        }
        println!("{dp:?}");
        res as i32
    }

    #[test]
    fn test_longest_valid_parentheses() {
        let s = ")()())".to_string();
        let res = longest_valid_parentheses(s);
        println!("{res}");
    }

    /**
     * 139. 单词拆分
     */
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];

        dp[0] = true;

        for i in 0..s.len() {
            for j in i + 1..s.len() + 1 {
                if dp[i] && word_dict.contains(&s[i..j].to_string()) {
                    dp[j] = true;
                }
            }
        }

        *dp.last().unwrap()
    }

    #[test]
    fn test_word_break() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        let res = word_break(s, word_dict);
        println!("{res}");
    }

    /**
     * 169. 多数元素
     */
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut count = 1;

        for i in 1..nums.len() {
            if nums[i] == res {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    res = nums[i];
                    count = 1;
                }
            }
        }

        res
    }

    #[test]
    fn test_majority_element() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let res = majority_element(nums);
        println!("{res}");
    }

    /**
     * 647. 回文子串
     */
    pub fn count_substrings(s: String) -> i32 {
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut res = 0;

        for i in 0..s.len() {
            for j in 0..=i {
                if s[i..=i] == s[j..=j] && (i - j < 2 || dp[j + 1][i - 1]) {
                    dp[j][i] = true;
                    res += 1;
                }
            }
        }

        res
    }

    #[test]
    fn test_count_substrings() {
        let s = "qweewqwe".to_string();
        let res = count_substrings(s);
        println!("{res}");
    }

    /**
     * 560. 和为 K 的子数组
     */
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut res = 0;
        let mut pre_count = 0;

        map.insert(0, 1);

        for i in nums {
            pre_count += i;

            if map.get(&(&pre_count - k)).is_some() {
                res += map.get(&(pre_count - k)).unwrap();
            }

            if map.get(&pre_count).is_some() {
                let base = map.remove(&pre_count).unwrap();
                map.insert(pre_count, base + 1);
            } else {
                map.insert(pre_count, 1);
            }
        }

        println!("{map:?}");

        res
    }

    #[test]
    fn test_subarray_sum() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1];
        let res = subarray_sum(nums, 3);
        println!("{res}");
    }

    /**
     * 198. 打家劫舍
     */
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];

        dp[1] = nums[0];

        for i in 2..=nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1])
        }

        dp[nums.len()]
    }

    /**
     * 221. 最大正方形
     */
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];

        for i in 1..=matrix.len() {
            for j in 1..=matrix[i - 1].len() {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1])) + 1;
                    res = res.max(dp[i][j]);
                }
            }
        }

        res * res
    }

    #[test]
    fn test_maximal_square() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];

        let res = maximal_square(matrix);

        println!("{res}");
    }

    /**
     * 剑指 Offer 14- I. 剪绳子
     */
    pub fn cutting_rope(n: i32) -> i32 {
        let mut dp = vec![0; 1000];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;

        match n {
            2 => 1,
            3 => 2,
            n => {
                for i in 4..=n {
                    dp[i as usize] =
                        ((2 * dp[i as usize - 2]).max(3 * dp[i as usize - 3])) % 1000000007;
                }
                dp[n as usize]
            }
        }
    }

    /**
     * 剑指 Offer 14- II. 剪绳子 II
     */
    pub fn cutting_rope2(n: i32) -> i32 {
        let mut n = n as i64;
        let mut res: i64 = 1;

        match n {
            2 => 1,
            3 => 2,
            mut n => {
                while n > 4 {
                    res = res * 3;
                    res = res % 1000000007;
                    n -= 3;
                }
                ((res * n) % 1000000007) as i32
            }
        }
    }

    /**
     * 64. 最小路径和
     */
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let mut base = i32::MAX;
                if i - 1 >= 0 && i - 1 < grid.len() {
                    base = base.min(grid[i - 1][j]);
                }
                if j - 1 >= 0 && j - 1 < grid[i].len() {
                    base = base.min(grid[i][j - 1]);
                }
                if !(i == 0 && j == 0) {
                    grid[i][j] += base;
                }
            }
        }

        println!("{grid:?}");

        *grid.last().unwrap().last().unwrap()
    }

    /**
     * 152. 乘积最大子数组
     */
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max, mut imax, mut imin) = (-i32::MAX, 1, 1);

        for i in nums {
            if i < 0 {
                let mut temp = imax;
                imax = imin;
                imin = temp;
            }

            imax = i.max(i * imax);
            imin = i.min(i * imin);

            max = max.max(imax);
        }

        max
    }

    /**
     * 134. 加油站
     */
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut has = 0;
        let mut cur_sum = 0;

        for i in 0..gas.len() {
            cur_sum += gas[i] - cost[i];
            has += gas[i] - cost[i];

            if cur_sum < 0 {
                cur_sum = 0;
                res = i as i32 + 1;
            }
        }

        if has < 0 {
            return -1;
        }

        res
    }

    /**
     * 2. 两数相加
     */
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut pre = res.as_mut();

        let mut l1 = l1;
        let mut l2 = l2;

        let mut ad = 0;

        while l1.is_some() || l2.is_some() {
            let mut val = ad;

            if let Some(n1) = l1 {
                val += n1.val;
                l1 = n1.next;
            }

            if let Some(n2) = l2 {
                val += n2.val;
                l2 = n2.next;
            }

            ad = val / 10;
            val = val % 10;

            let node = ListNode { val, next: None };

            if let Some(cur) = pre {
                cur.next = Some(Box::new(node));
                pre = cur.next.as_mut();
            }
        }

        if ad > 0 {
            pre.unwrap().next = Some(Box::new(ListNode::new(1)));
        }

        let s = "aa".to_string();
        let a = s.chars().enumerate();
        a.for_each(|(a, c)| {});

        res.unwrap().next
    }

    /**
     * 14. 最长公共前缀
     */
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = 0;
        let strs: Vec<Vec<char>> = strs.iter().map(|a| {
            return a.chars().collect::<Vec<char>>();
        }).collect();

        loop {
            let pre = strs[0][res];

            for i in 1..strs.len() {
                if res >= strs[i].len() || strs[i][res] != pre {
                    return strs[0][0..res].iter().collect::<String>();
                }
            }

            res+=1;
        }
    }
}
