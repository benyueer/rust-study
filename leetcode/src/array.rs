/**
 * 数组
 */

#[cfg(test)]
mod array {
    use std::borrow::BorrowMut;

    /**
     * 78. 子集
     */
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 解法1
        // 先将空集合加入，然后遍历数组依次在结果的每一项中添加数组的当前项
        let mut res = vec![vec![]];

        for n in nums {
            for i in 0..res.len() {
                let mut new_set = res[i].clone();
                new_set.push(n);
                res.push(new_set);
            }
        }

        res
    }

    pub fn subsets_2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 解法2
        // 长度为n的数组有2**n个子集，用n位二进制表示当前项是否加入结果
        let mut res = vec![];

        for i in 0..(1 << nums.len()) {
            let mut set = vec![];

            for n in 0..nums.len() {
                if (i >> n) & 1 == 1 {
                    set.push(nums[n]);
                }
            }

            res.push(set);
        }

        res
    }

    pub fn subsets_3(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 解法3
        // 递归地构建结果，对于数组的每一项的有选择或不选择两种选项
        let mut res = vec![];
        let mut t = vec![];

        fn dfs(n: i32, res: &mut Vec<Vec<i32>>, t: &mut Vec<i32>, nums: &Vec<i32>) {
            if n == nums.len() as i32 {
                res.push(t.clone());
                return;
            }
            // 加入当前项
            t.push(nums[n as usize]);
            dfs(n + 1, res, t, nums);
            // 删除当前项
            t.pop();
            dfs(n + 1, res, t, nums);
        }

        dfs(0, &mut res, &mut t, &nums);
        res
    }

    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 4, 5];
        // let res = subsets(nums);
        let res = subsets_3(nums);
        println!("{res:?}");
    }

    /**
     * 90. 子集 II
     */
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 基于78，多了条件去除重复子集，因此要先将数组排序，在遍历时记录上一次最后添加的元素，如果与当前元素相同，则跳过本次
        let mut res = vec![];
        let mut t = vec![];
        let mut nums = nums;
        nums.sort();

        fn dfs(
            use_pre: bool,
            res: &mut Vec<Vec<i32>>,
            n: usize,
            t: &mut Vec<i32>,
            nums: &Vec<i32>,
        ) {
            if n == nums.len() {
                res.push(t.clone());
                return;
            }

            dfs(false, res, n + 1, t, nums);
            if n > 0 && nums[n] == nums[n - 1] && !use_pre {
                return;
            }
            t.push(nums[n]);
            dfs(true, res, n + 1, t, nums);
            t.pop();
        }

        dfs(false, &mut res, 0, &mut t, &nums);

        res
    }

    #[test]
    fn test_subsets_with_dup() {
        let nums = vec![1, 2, 2, 3];
        let res = subsets_with_dup(nums);

        println!("{res:?}");
    }

    /**
     * 120. 三角形最小路径和
     */
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                let llen = triangle[i - 1].len();
                let lt = match j {
                    x if x >= 1 && x <= llen => triangle[i - 1][j - 1],
                    _ => i32::MAX,
                };
                let rt = match j {
                    x if x < llen => triangle[i - 1][j],
                    _ => i32::MAX,
                };
                triangle[i][j] += lt.min(rt);
            }
        }

        let mut s = triangle.last().unwrap().clone();
        s.sort();

        *s.first().unwrap()
    }

    #[test]
    fn test_minimum_total() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];

        let res = minimum_total(triangle);
        println!("{res}");
    }

    /**
     * 122. 买卖股票的最佳时机 II
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..prices.len() {
            res += 0.max(prices[i] - prices[i - 1]);
        }

        res
    }

    /**
     * 473. 火柴拼正方形
     */
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let mut matchsticks = matchsticks;
        matchsticks.sort();
        matchsticks.reverse();
        let all = matchsticks.iter().cloned().reduce(|l, i| l + i).unwrap();
        if all % 4 != 0 {
            return false;
        }
        let base = all / 4;

        let mut side_lens = vec![0; 4];

        fn dfs(ind: usize, matchsticks: &Vec<i32>, side_lens: &mut Vec<i32>, len: i32) -> bool {
            if ind == matchsticks.len() {
                return true;
            }
            for i in 0..4 {
                side_lens[i] += matchsticks[ind];
                if side_lens[i] <= len && dfs(ind + 1, matchsticks, side_lens, len) {
                    return true;
                }
                side_lens[i] -= matchsticks[ind];
            }

            false
        }

        dfs(0, &matchsticks, &mut side_lens, base)
    }

    #[test]
    fn test_makesquare() {
        let matchsticks = vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3];
        let res = makesquare(matchsticks);
        println!("{res}");
    }
}
