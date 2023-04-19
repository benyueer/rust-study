#[cfg(test)]
mod search_backtract {
    /**
     * 79. 单词搜索
     */
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();

        fn dfs(
            board: &Vec<Vec<char>>,
            word: &Vec<char>,
            ind: usize,
            vis: &mut Vec<Vec<bool>>,
            x: usize,
            y: usize,
        ) -> bool {
            if ind == word.len() {
                if board[x][y] == word[ind - 1] {
                    return true;
                }
                return false;
            }
            let dir = vec![(2, 1), (0, 1), (1, 2), (1, 0)];
            for i in dir {
                let nx = x + i.0;
                let ny = y + i.1;

                if nx >= 1
                    && nx <= board.len()
                    && ny >= 1
                    && ny <= board[0].len()
                    && !vis[nx - 1][ny - 1]
                    && board[nx - 1][ny - 1] == word[ind]
                {
                    vis[nx - 1][ny - 1] = true;
                    let f = dfs(board, word, ind + 1, vis, nx - 1, ny - 1);
                    vis[nx - 1][ny - 1] = false;
                    if f {
                        return true;
                    }
                }
            }

            false
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == word[0] {
                    let mut vis = vec![vec![false; board[0].len()]; board.len()];
                    vis[i][j] = true;
                    let f = dfs(&board, &word, 1, &mut vis, i, j);
                    if f {
                        return true;
                    }
                }
            }
        }

        false
    }

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let word = "ABCCED".to_string();

        let res = exist(board, word);
        println!("12`2`1{res}");
    }

    /**
     * 40. 组合总和 II
     */
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // let mut res = vec![];
        // let mut ans = vec![];
        // let mut candidates = candidates;
        // candidates.sort();

        // let mut map = vec![0; 60];

        // for i in candidates.clone() {
        //     map[i as usize] += 1;
        // }

        // ans.push(vec![]);

        // for i in 0..candidates.len() {
        //     for j in 0..ans.len() {
        //         if j <= map[candidates[i] as usize] && i > 0 && candidates[i-1] == candidates[i] {
        //             continue;
        //         }
        //         let mut new_set = ans[j].clone();
        //         new_set.push(candidates[i]);

        //         let count = new_set.iter().cloned().reduce(|p, c| p + c).unwrap();

        //         if count == target {
        //             res.push(new_set.clone());
        //         }

        //         if count < target {
        //             ans.push(new_set);
        //         }
        //     }
        // }

        // res

        let mut res = vec![];

        let mut candidates = candidates;
        candidates.sort();
        // candidates.reverse();


        fn dfs(
            ind: usize,
            path: &mut Vec<i32>,
            ans: i32,
            res: &mut Vec<Vec<i32>>,
            candidates: &Vec<i32>,
        ) {
            if ans == 0 {
                res.push(path.clone());
                return;
            }
            if ans < 0 {
                return;
            }

            for i in ind..candidates.len() {
                if candidates[i] > ans {
                    break;
                }

                if i > ind && candidates[i] == candidates[i-1] {
                    continue;
                }

                path.push(candidates[i]);
                dfs(i + 1, path, ans - candidates[i], res, candidates);
                path.pop();
            }
        }

        dfs(0, &mut vec![], target, &mut res, &candidates);

        res
    }

    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10,1,2,7,6,1,5];
        let target = 8;
        let res = combination_sum2(candidates, target);
        println!("{res:?}");
    }
}
