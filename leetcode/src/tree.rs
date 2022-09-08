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
        None
    }
}
