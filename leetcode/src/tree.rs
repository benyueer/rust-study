#[cfg(test)]
mod test {
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
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    /**
     * * 树相关的问题解决方法都依赖树的遍历或特殊树的性质
     * 前序，中序，后续
     * 完全二叉树，二叉搜索树
     */

    /**
     * 递归遍历
     * 改变访问当前节点、左子树、右子树的顺序就能实现递归的遍历
     */
    fn recursion_traverse(root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(cur) = root {
            // 当前节点
            println!("{}", cur.borrow().val);
            // 左子树
            recursion_traverse(cur.borrow().left.clone());
            // 右子树
            recursion_traverse(cur.borrow().right.clone());
        }
    }

    /**
     * 栈遍历
     * 前序
     */
    fn pre_order_traverse(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res: Vec<i32> = vec![];
        while root.is_some() || !stack.is_empty() {
            while root.is_some() {
                stack.push(root.clone());
                res.push(root.clone().unwrap().borrow().val);
                root = root.unwrap().borrow().left.clone();
            }
            root = stack.pop().unwrap().unwrap().borrow().right.clone();
        }

        println!("{:?}", root);

        res
    }

    /**
     * 中序遍历
     */
    fn in_order_reaverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                stack.push(cur.clone());
                cur = cur.unwrap().borrow().left.clone();
            }

            cur = stack.pop().unwrap();
            res.push(cur.as_ref().unwrap().borrow().val.to_owned());
            cur = cur.unwrap().borrow().right.clone();
        }

        res
    }

    /**
     * 后序遍历
     */
    fn post_order_traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut cur = root;
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                stack.push(cur.clone());
                cur = cur.unwrap().borrow().left.clone();
            }

            cur = stack.last().unwrap().to_owned();

            if cur.as_ref().unwrap().borrow().right.is_none()
                || cur.as_ref().unwrap().borrow().right == pre
            {
                res.push(cur.as_ref().unwrap().borrow().val.to_owned());

                stack.pop();
                pre = cur.clone();
                cur = None;
            } else {
                cur = cur.unwrap().borrow().right.clone();
            }
        }

        res
    }

    #[test]
    fn test_tree_stack_traverse() {
        let n_1 = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let n_2 = TreeNode {
            val: 2,
            left: None,
            right: None,
        };
        let n_3 = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(n_1))),
            right: Some(Rc::new(RefCell::new(n_2))),
        };
        let root = Some(Rc::new(RefCell::new(n_3)));

        // let res = pre_order_traverse(root);
        // let res = in_order_reaverse(root);
        let res = post_order_traverse(root);
        println!("{:?}", res);
    }

    /**
     * 103
     */
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut queue = vec![];

        if let Some(node) = root {
            queue.push(node);
        }

        let mut dir = true;

        while !queue.is_empty() {
            // println!("{:?}", queue);
            let mut a = vec![];

            for i in 0..queue.len() {
                let mut t = queue.remove(0);
                a.push(t.borrow().val);
                if let Some(l) = t.clone().borrow().left.as_ref() {
                    queue.push(l.clone());
                }
                if let Some(r) = t.clone().borrow().right.as_ref() {
                    queue.push(r.clone());
                }
            }
            if !dir {
                a.reverse();
            }
            res.push(a);
            dir = !dir;
        }

        res
    }

    #[test]
    fn test_zigzag_level_order() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let res = zigzag_level_order(root);
        println!("{:?}", res);
    }

    /**
     * 98
     * [5,4,6,null,null,3,7]
     * 2147483647
     * 2147483647
     */
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, l: i64, r: i64) -> bool {
            if let Some(root) = root {
                let val = root.borrow().val as i64;
                if val > l && val < r {
                    return dfs(&root.borrow().left, l, val.min(r))
                        && dfs(&root.borrow().right, val.max(l), r);
                } else {
                    return false;
                }
            } else {
                return true;
            }
        }

        dfs(&root, i64::MIN, i64::MAX)
    }

    #[test]
    fn test_is_valid_bst() {
        let n_1 = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let n_3 = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let n_2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(n_1))),
            right: Some(Rc::new(RefCell::new(n_3))),
        };
        let root = Some(Rc::new(RefCell::new(n_2)));
        let res = is_valid_bst(root);
        assert!(res);
    }

    /**
     * 二叉树的所有目标路径
     */
    fn all_target_path(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            cur: &mut Vec<i32>,
            target: i32,
            res: &mut Vec<Vec<i32>>,
        ) {
            if let Some(root) = root {
                cur.push(root.borrow().val);
                let has_l = root.borrow().left.is_some();
                let has_r = root.borrow().right.is_some();
                if !has_l && !has_r {
                    let mut count = 0;
                    cur.iter().for_each(|i| count += i);
                    if count == target {
                        res.push(cur.to_vec());
                    }
                }
                if has_l {
                    dfs(root.borrow().left.clone(), &mut cur.clone(), target, res);
                }
                if has_r {
                    dfs(root.borrow().right.clone(), &mut cur.clone(), target, res);
                }
            }
        }

        dfs(root, &mut vec![], target, &mut res);

        res
    }

    #[test]
    fn test_all_target_path() {
        let mut n_1 = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let mut n_1_b = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(n_1))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        };
        let n_2 = TreeNode {
            val: 2,
            left: None,
            right: None,
        };
        let n_5 = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(n_1_b))),
            right: Some(Rc::new(RefCell::new(n_2))),
        };
        let root = Some(Rc::new(RefCell::new(n_5)));
        let target = 8;
        let res = all_target_path(root, target);
        println!("{:?}", res);
    }

    /**
     * 99
     * 恢复二叉搜索树
     */
    fn recover_tree<'a>(root: &'a mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut x: Option<Rc<RefCell<TreeNode>>> = None;
        let mut y: Option<Rc<RefCell<TreeNode>>> = None;
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
        // println!("{:?}", cur);
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                stack.push(cur.clone());
                cur = cur.unwrap().borrow().left.clone();
            }
            cur = stack.pop().unwrap();

            if pre.is_some()
                && cur.as_ref().unwrap().borrow().val < pre.as_ref().unwrap().borrow().val
            {
                y = cur.clone();
                if x.is_none() {
                    x = pre;
                } else {
                    break;
                }
            }

            pre = cur.clone();
            cur = cur.unwrap().borrow().right.clone();
        }
        let temp = x.as_ref().unwrap().borrow().val.to_owned();
        x.unwrap().borrow_mut().val = y.as_ref().unwrap().borrow().val.to_owned();
        y.unwrap().borrow_mut().val = temp;
    }

    #[test]
    fn test_recover_tree() {
        let n_1 = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let n_3 = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let n_2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(n_3))),
            right: Some(Rc::new(RefCell::new(n_1))),
        };
        let mut root = Some(Rc::new(RefCell::new(n_2)));

        recover_tree(&mut root);

        println!("{:?}", root);
    }

    /**
     * 530
     * 二叉搜索树的最小绝对差
     */
    fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let res = 0;

        res
    }

    /**
     * 450-删除二叉搜索树中的节点
     */
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }

    /**
     * 235-二叉搜索树的最近公共祖先
     */
    pub fn search_lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        loop {
            if q > root.as_ref().unwrap().borrow().val && p > root.as_ref().unwrap().borrow().val {
                root = root.unwrap().borrow().right.clone();
            } else if q < root.as_ref().unwrap().borrow().val
                && p < root.as_ref().unwrap().borrow().val
            {
                root = root.unwrap().borrow().left.clone();
            } else {
                return root;
            }
        }
    }

    #[test]
    fn test_search_lowest_common_ancestor() {
        let n_1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));
        let n_3 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let n_2 = TreeNode {
            val: 2,
            left: Some(Rc::clone(&n_3)),
            right: Some(Rc::clone(&n_1)),
        };
        let mut root = Some(Rc::new(RefCell::new(n_2)));
        let res = search_lowest_common_ancestor(root, Some(n_1), Some(n_3));
        println!("{:?}", res);
    }

    /**
     * 236-二叉树的最近公共祖先
     */
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let root_val = root.as_ref().unwrap().borrow().val;
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        if root_val == p_val || root_val == q_val {
            return root;
        }

        let in_left = lowest_common_ancestor(
            root.as_ref().unwrap().borrow_mut().left.take(),
            p.clone(),
            q.clone(),
        );
        let in_right =
            lowest_common_ancestor(root.as_ref().unwrap().borrow_mut().right.take(), p, q);

        if in_left.is_some() && in_right.is_some() {
            return root;
        } else if in_left.is_some() {
            return in_left;
        } else if in_right.is_some() {
            return in_right;
        } else {
            return None;
        }
    }

    /**
     * 124. 二叉树中的最大路径和
     */
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root = root;
        let mut res = i32::MIN;

        fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if root.is_none() {
                return 0;
            }
            let l_val = dfs(&mut root.as_ref().unwrap().borrow_mut().left.clone(), res).max(0);
            let r_val = dfs(&mut root.as_ref().unwrap().borrow_mut().right.clone(), res).max(0);

            *res = *res.max(&mut (root.as_ref().unwrap().borrow_mut().val + l_val + r_val));
            root.as_ref().unwrap().borrow_mut().val += std::cmp::max(l_val, r_val);

            return root.as_ref().unwrap().borrow_mut().val;
        }

        dfs(&mut root, &mut res);

        res
    }

    #[test]
    fn test_max_path_sum() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        max_path_sum(root);

        // println!("{:?}", root);
    }

    /**
     * 199. 二叉树的右视图
     */
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        res
    }

    /**
     * 617. 合并二叉树
     */
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // fn dfs(
        //     root1: Option<Rc<RefCell<TreeNode>>>,
        //     root2: Option<Rc<RefCell<TreeNode>>>,
        // ) -> Option<Rc<RefCell<TreeNode>>> {
        //     if root1.is_none() {
        //         return root2;
        //     } else if root2.is_none() {
        //         return root1;
        //     } else {
        //         let mut r1 = root1.as_ref().unwrap().borrow_mut();
        //         r1.val += root2.as_ref().unwrap().borrow_mut().val;
        //         r1.left = dfs(
        //             r1.left.to_owned(),
        //             root2.as_ref().unwrap().borrow_mut().left.to_owned(),
        //         );
        //         r1.right = dfs(
        //             r1.right.to_owned(),
        //             root2.as_ref().unwrap().borrow_mut().right.to_owned(),
        //         );
        //         drop(r1);
        //         return root1;
        //     }
        // }

        // dfs(root1, root2)

        match (root1, root2) {
            (Some(t1), Some(t2)) => match (t1.borrow_mut(), t2.borrow_mut()) {
                (mut t1, mut t2) => Some(Rc::new(RefCell::new(TreeNode {
                    val: t1.val + t2.val,
                    left: merge_trees(t1.left.take(), t2.left.take()),
                    right: merge_trees(t1.right.take(), t2.right.take()),
                }))),
            },
            (None, None) => None,
            (some, None) => some,
            (None, some) => some,
        }
    }

    /**
     * 543. 二叉树的直径
     */
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if root.is_none() {
                return 0;
            }

            let mut r_b = root.as_ref().unwrap().borrow_mut();
            let l = dfs(r_b.left.take(), res);
            let r = dfs(r_b.right.take(), res);

            *res = *res.max(&mut (l + r + 1));
            return l.max(r) + 1;
        }

        dfs(root, &mut res);

        res - 1
    }

    /**
     * 113. 路径总和 II
     */
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            path: &mut Vec<i32>,
            mut sum: i32,
            res: &mut Vec<Vec<i32>>,
            target_sum: i32,
        ) {
            if root.is_none() {
                return;
            };
            let mut r = root.as_ref().unwrap().borrow_mut();
            if r.left.is_none() && r.right.is_none() && sum + r.val == target_sum {
                path.push(r.val);
                res.push(path.to_vec());
            };

            path.push(r.val);
            sum += r.val;
            dfs(r.left.take(), &mut path.clone(), sum, res, target_sum);
            dfs(r.right.take(), &mut path.clone(), sum, res, target_sum);
        };

        dfs(root, &mut [].to_vec(), 0, &mut res, target_sum);

        res
    }

    /**
     * 101. 对称二叉树
     */
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        fn is_same(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if l.is_none() && r.is_none() {
                return true;
            }

            if l.is_some() && r.is_some() {
                if l.as_ref().unwrap().borrow().val != r.as_ref().unwrap().borrow().val {
                    return false;
                };

                let mut l = l.as_ref().unwrap().borrow_mut();
                let mut r = r.as_ref().unwrap().borrow_mut();

                return is_same(l.left.take(), r.right.take())
                    && is_same(l.right.take(), r.left.take());
            }

            return false;
        }

        let mut root = root.as_ref().unwrap().borrow_mut();

        return is_same(root.left.take(), root.right.take());
    }

    /**
     * 437. 路径总和 III
     */
    pub fn path_sum_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert(0, 1);

        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            target_sum: i64,
            mut cur_sum: i64,
            map: &mut std::collections::HashMap<i64, i32>,
        ) -> i32 {
            if root.is_none() {
                return 0;
            }

            let mut res = 0;
            cur_sum += root.as_ref().unwrap().borrow().val as i64;

            res += if let Some(&t) = map.get(&(cur_sum - target_sum)) {
                t
            } else {
                0
            };

            *map.entry(cur_sum).or_insert(0) += 1;

            res += dfs(
                &root.as_ref().unwrap().borrow().left,
                target_sum,
                cur_sum,
                map,
            );
            res += dfs(
                &root.as_ref().unwrap().borrow().right,
                target_sum,
                cur_sum,
                map,
            );

            *map.entry(cur_sum).or_insert(0) -= 1;

            res
        }

        let mut root = root;

        dfs(&root, target_sum as i64, 0, &mut map)
    }

    #[test]
    fn test_path_sum_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let res = path_sum_2(root, 1);

        println!("{res}");
    }

    /**
     * 687. 最长同值路径
     */
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(r) = root {
                let mut ans = 0;
                let t = r.as_ref().borrow();
                let (l, r) = (dfs(&t.left, res), dfs(&t.right, res));
                let (mut l1, mut r1) = (0, 0);

                if t.left.is_some() && t.left.as_ref().unwrap().borrow().val == t.val {
                    l1 = l +1;
                }

                if t.right.is_some() && t.right.as_ref().unwrap().borrow().val == t.val {
                    r1 = r + 1;
                }

                *res = (*res).max(l1 + r1);

                return l1.max(r1);
            } else {
                0
            }
        }

        let mut res = 0;

        dfs(&root, &mut res);

        res
    }
}
