#[cfg(test)]
mod test {
    use std::{collections::HashSet, hash::Hash};

    #[test]
    fn set_new() {
        /*
            * new
            新建
        */
        let s: HashSet<String> = HashSet::new();


        /*
            * from()
            根据提供的值新建
        */
        let s:HashSet<i32> = HashSet::from([1, 2, 3, 2]);
        println!("{:#?}", s); // 1, 2, 3
    }

    #[test]
    fn set_iter() {
        /*
            * iter
            返回迭代器
        */
        let mut s: HashSet<i32> = HashSet::new();
        s.insert(1);

        for i in s.iter() {
            println!("{i}");
        }


        /*
            * drain
            清除集合，将所有元素作为迭代器返回
        */
        let mut s: HashSet<i32> = HashSet::from([1, 2, 3]);

        assert_eq!(s.drain().next(), Some(1));
        assert!(s.is_empty());
    }

    #[test]
    fn set_len() {
        /*
            * len
            返回集合中元素数量
        */
        let mut s: HashSet<i32> = HashSet::new();
        s.insert(1);
        assert_eq!(s.len(), 1);
        s.insert(2);
        assert_eq!(s.len(), 2);
        s.insert(2);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn set_contains() {
        /*
            * contains(& Q)
            返回是否包含给定值
        */
        let s: HashSet<i32> = HashSet::from([1, 2, 3]);
        assert!(s.contains(&1));

        /*
            * get(& Q) -> Option
            返回集合中与给定值相同的引用（如果有）
        */
        let s: HashSet<i32> = HashSet::from([1, 2, 3]);
        assert_eq!(s.get(&2), Some(&2));
    }

    #[test]
    fn set_insert() {
        /*
            * insert(T)
            向集合中添加一个值
        */
        let mut s: HashSet<i32> = HashSet::new();
        s.insert(1);

        /*
            * replace(T)
            向集合中添加一个值，并返回该值，如果已经有则替换
        */
        let mut s: HashSet<Vec<i32>> = HashSet::new();
        s.insert(Vec::<i32>::new());

        assert_eq!(s.get(&[][..]).unwrap().capacity(), 0);
        s.replace(Vec::with_capacity(10));
        assert_eq!(s.get(&[][..]).unwrap().capacity(), 10);

        /*
            * remove(& Q)
            删除集合中给定的值，返回删除是否成功
        */
        let mut s: HashSet<i32> = HashSet::from([1, 2, 3]);
        assert!(s.remove(&2));
        assert!(!s.remove(&2));


        /*
            * take(& Q) -> Option
            删除并返回集合中给定的值
        */
        let mut s: HashSet<i32> = HashSet::from([1, 2, 3]);
        assert_eq!(s.take(&1), Some(1));
        assert_eq!(s.take(&1), None);
    }

}