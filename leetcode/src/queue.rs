/**
 * 队列
 * 优先队列
 * 堆
 */
#[cfg(test)]
mod test {

    use std::collections::HashMap;
    /**
     * 捡金币，A[i]表示i上的金币，从0出发，当走到i时要捡起或交出A[i]对应的金币，下一步可以选择走到A[i+1, i+k],求最多能获得多少金币
     * A = [1, -1, -100, -1000, 100, 3] k = 2  res = 4
     * get [1,  0, -99, ]
     * que [1,  0, -99, ]
     */
    use std::collections::VecDeque;
    fn collect_coin(coins: Vec<i32>, k: i32) -> i32 {
        let mut get: Vec<i32> = vec![0; coins.len()];
        let mut queue: VecDeque<i32> = VecDeque::new();

        for i in 0..coins.len() {
            if i as i32 - k > 0 {
                // if !queue.is_empty() && queue.front().unwrap().to_owned() == get[i-k as usize - 1] {
                //     queue.pop_front();
                // }
                while !queue.is_empty() && queue.len() > k as usize {
                    queue.pop_front();
                }
            }

            let old = queue.front().unwrap_or(&0);
            get[i] = old + coins[i];

            while !queue.is_empty() && queue.back().unwrap().to_owned() < get[i] {
                queue.pop_back();
            }

            queue.push_back(get[i]);
        }
        *get.last().unwrap()
    }

    #[test]
    fn test_collect_coin() {
        let coins = vec![1, -1, -100, -1000, 100, 3];
        let k = 2;
        let res = collect_coin(coins, k);
        println!("{res}");
    }

    use std::cmp::Reverse;
    /**
     * 返回最小的k个数
     */
    use std::collections::BinaryHeap;
    fn small_k_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut heap = BinaryHeap::new();

        for i in nums {
            heap.push(Reverse(i));
        }

        for i in 0..k {
            res.push(heap.pop().unwrap().0);
        }

        res
    }

    #[test]
    fn test_small_k_numbers() {
        let nums = vec![3, 2, 1];
        let k = 2;
        let res = small_k_numbers(nums, k);
        println!("{:?}", res);
    }

    /**
     * 215
     */
    // use std::collections::BinaryHeap;
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in nums {
            heap.push(i);
        }

        for i in 0..k - 1 {
            heap.pop();
        }

        heap.pop().unwrap()
    }

    #[test]
    fn test_find_kth_largest() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let res = find_kth_largest(nums, k);
        println!("{:?}", res);
    }

    /**
     * 347
     */
    pub fn top_k_frequent(nums: Vec<i32>, t: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();
        nums.into_iter()
            .for_each(|i| *map.entry(i).or_insert(0) += 1);

        for (k, v) in map.iter() {
            if heap.len() == t as usize {
                if *heap.peek().unwrap() > Reverse((v, k)) {
                    heap.pop();
                } else {
                    continue;
                }
            }
            heap.push(Reverse((v, k)));
        }

        heap.into_vec().iter().map(|&n| *n.0 .1).collect()
    }

    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let t = 2;
        let res = top_k_frequent(nums, t);
        println!("{:?}", res);
    }

    /**
     * 373
     */
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = nums1
            .iter()
            .enumerate()
            .map(|(n, v)| Reverse((nums1[n] + nums2[0], n, 0)))
            .collect();

        for i in 0..k as usize {
            if let Some(Reverse((i, j, k))) = heap.pop() {
                if k + 1 < nums2.len() {
                    heap.push(Reverse((nums1[j] + nums2[k + 1], j, k + 1)));
                }
                res.push(vec![nums1[j], nums2[k]]);
            } else {
                continue;
            }
        }

        res
    }

    #[test]
    fn test_k_smallest_pairs() {
        let nums1 = vec![1, 1, 2];
        let nums2 = vec![1, 2, 3];
        let k = 2;
        let res = k_smallest_pairs(nums1, nums2, k);
        println!("{:?}", res);
    }

    /**
     * 378
     * 堆并不是最优解，二分是
     */
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let cols = matrix.len();

        matrix[0].iter().enumerate().for_each(|(i, v)| {
            heap.push(Reverse((matrix[0][i], 0, i)));
        });

        for i in 1..k {
            if let Some(Reverse((v, j, i))) = heap.pop() {
                if j + 1 < cols {
                    heap.push(Reverse((matrix[j + 1][i], j + 1, i)));
                }
            }
        }

        heap.pop().unwrap().0 .0
    }

    #[test]
    fn test_kth_smallest() {
        // println!("{:?}", (1, 0, 0).cmp(&(2, 0, 1)));

        let matrix = vec![
            // vec![1,5,9],
            // vec![10,11,13],
            // vec![12,13,15]
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
        ];
        let k = 14;
        let res = kth_smallest(matrix, k);
        println!("{res}");
    }

    /**
     * 871
     */
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut heap = BinaryHeap::new();
        let mut cur_fuel = start_fuel;
        let mut cur_pos = 0;
        let mut ind = 0;

        while cur_pos < target {
            if cur_fuel == 0 {
                if !heap.is_empty() {
                    cur_fuel += heap.pop().unwrap();
                    res += 1;
                } else {
                    return -1;
                }
            }
            cur_pos += cur_fuel;
            cur_fuel = 0;

            while ind < stations.len() && stations[ind][0] <= cur_pos {
                heap.push(stations[ind][1]);
                ind += 1;
            }
        }

        res
    }

    #[test]
    fn test_min_refuel_stops() {
        let target = 1000;
        let start_fuel = 299;
        let stations = vec![
            // vec![10,60],
            // vec![20,30],
            // vec![30,30],
            // vec![60,40]
            vec![13,21],
            vec![26,115],
            vec![100,47],
            vec![225,99],
            vec![299,141],
            vec![444,198],
            vec![608,190],
            vec![636,157],
            vec![647,255],
            vec![841,123]
        ];
        let res = min_refuel_stops(target, start_fuel, stations);
        println!("{res}");
    }

    /**
     * 743
     */
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let res = 0;
        res
    }

    /**
     * 第i天会掉落A[i]个果子，保存时间B[i]，每天只能吃一个，求最多可以吃多少个果子
     * A=[3, 1] B=[3, 1]  res = 3
     */
    fn eat_apple(apples: Vec<i32>, shelf_life: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cur = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((shelf_life[0] + cur, apples[0])));

        while !heap.is_empty() {
            let mut a = heap.peek_mut().unwrap();
            if a.0.0 > cur && a.0.1 > 0 {
                res += 1;
                cur += 1;
                a.0.1 -= 1;
            } else {
                drop(a);
                heap.pop();
                continue;
            }

            if cur < apples.len().try_into().unwrap() {
                drop(a);
                heap.push(Reverse((shelf_life[cur as usize] + cur, apples[cur as usize])));
            }
        }
        

        res
    }

    #[test]
    fn test_eat_apple() {
        let apples = vec![1, 4, 1, 1, 5];
        let shelf_life = vec![1, 2, 1, 1, 4];
        // let apples = vec![3, 1];
        // let shelf_life = vec![3, 1];
        let res = eat_apple(apples, shelf_life);
        println!("{res}");
    }
}
